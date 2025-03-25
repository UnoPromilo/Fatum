use crate::utils::{HmacSecret, UserId};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const EXPIRATION_HOURS: u64 = 24;

pub fn generate_jwt(user_id: &UserId, secret: &HmacSecret) -> Result<String, anyhow::Error> {
    let expiration_time = Utc::now() + Duration::from_secs(EXPIRATION_HOURS * 3600);
    let claims = Claims {
        sub: *user_id,
        exp: expiration_time.timestamp(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| anyhow::anyhow!(e))
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: UserId,
    exp: i64,
}
