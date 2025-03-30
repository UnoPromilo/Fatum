use crate::authentication::{Credentials, generate_jwt, validate_credentials};
use crate::utils::{AccountEmail, HmacSecret};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub async fn post_handler(
    Extension(db): Extension<PgPool>,
    Extension(hmac_secret): Extension<HmacSecret>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, StatusCode> {
    let email = match AccountEmail::parse(request.email) {
        Ok(email) => Ok(email),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }?;

    let credentials = Credentials {
        email,
        password: request.password.into(),
    };

    let user_id = match validate_credentials(credentials, &db).await {
        Ok(validate_credentials) => Ok(validate_credentials),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }?;

    let jwt = match generate_jwt(&user_id, &hmac_secret) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }?;

    Ok(Json(Response { jwt }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub jwt: String,
}
