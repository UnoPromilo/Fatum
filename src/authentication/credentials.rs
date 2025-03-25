use crate::utils::{AccountEmail, UserId};
use anyhow::{Context, anyhow};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use tokio::task::spawn_blocking;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub email: AccountEmail,
    pub password: SecretString,
}

pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<UserId, AuthError> {
    let mut user_id = None;
    // To prevent timing attack
    let mut expected_password_hash = SecretString::from(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno",
    );

    let email = credentials.email;

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&email, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking(move || verify_password_hash(&expected_password_hash, &credentials.password))
        .await
        .context("Failed to verify credentials")??;

    user_id
        .ok_or_else(|| anyhow::Error::msg("Invalid email address"))
        .map_err(AuthError::InvalidCredentials)
}

async fn get_stored_credentials(
    email: &AccountEmail,
    pool: &PgPool,
) -> Result<Option<(UserId, SecretString)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"SELECT user_id, password_hash FROM accounts WHERE email = $1"#,
        email.as_ref(),
    )
    .fetch_optional(pool)
    .await
    .context("Failed to fetch credentials from stored account")?
    .map(|row| (row.user_id.into(), SecretString::from(row.password_hash)));

    Ok(row)
}

fn verify_password_hash(
    expected_password_hash: &SecretString,
    password_candidate: &SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .map_err(|e| anyhow!(e))
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|e| anyhow!(e))
        .context("Failed to verify password")
        .map_err(AuthError::InvalidCredentials)
}
