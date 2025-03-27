use crate::authentication::get_user_id_if_token_is_valid;
use crate::utils::HmacSecret;
use axum::Extension;
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

pub async fn reject_unauthorized_user(
    Extension(hmac_secret): Extension<HmacSecret>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = get_token(&headers);
    
    if let Some(token) = token {
        if let Ok(user_id) = get_user_id_if_token_is_valid(token, &hmac_secret) {
            request.extensions_mut().insert(user_id);
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
}
