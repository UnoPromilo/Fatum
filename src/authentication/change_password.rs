use crate::utils::UserId;
use anyhow::{Context, anyhow};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use tokio::task::spawn_blocking;

pub async fn change_password(
    user_id: UserId,
    new_password: SecretString,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking(move || compute_password_hash(new_password)).await??;

    sqlx::query!(
        "UPDATE accounts SET password_hash = $1 WHERE user_id = $2",
        password_hash.expose_secret(),
        user_id.as_ref()
    )
    .execute(pool)
    .await
    .context("Failed to change password in database")?;
    Ok(())
}

fn compute_password_hash(password: SecretString) -> Result<SecretString, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)
    .map_err(|e| anyhow!(e))
    .context("Failed to hash password")?
    .to_string();
    Ok(SecretString::from(password_hash))
}
