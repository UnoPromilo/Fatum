use crate::authentication::{update_stored_password, validate_password};
use crate::utils::UserId;
use axum::{Extension, Json, http::StatusCode};
use secrecy::SecretString;
use serde::Deserialize;
use sqlx::PgPool;

pub async fn put_handler(
    Extension(db): Extension<PgPool>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<Request>,
) -> Result<StatusCode, StatusCode> {
    if let Err(_) = validate_password(&user_id, request.old_password.into(), &db).await {
        return Err(StatusCode::FORBIDDEN);
    }

    if is_password_secure_enough(&request.new_password) == false {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    if let Err(_) =
        update_stored_password(user_id, SecretString::from(request.new_password), &db).await
    {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::NO_CONTENT)
}

fn is_password_secure_enough(password: &str) -> bool {
    password.len() > 8
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub old_password: String,
    pub new_password: String,
}
