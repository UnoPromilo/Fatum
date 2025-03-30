use crate::utils::{HmacSecret, UserId};
use anyhow::{Error, anyhow};
use chrono::{TimeZone, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const EXPIRATION_HOURS: u64 = 24;

pub fn generate_jwt(user_id: &UserId, secret: &HmacSecret) -> Result<String, Error> {
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

pub fn get_user_id_if_token_is_valid(token: &str, secret: &HmacSecret) -> Result<UserId, Error> {
    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?
    .claims;

    // Check if token is still valid
    let valid_until = Utc
        .timestamp_opt(claims.exp, 0)
        .single()
        .ok_or_else(|| anyhow!("Failed to parse expiration timestamp"))?;

    if valid_until < Utc::now() {
        return Err(anyhow!("Token has expired"));
    }

    Ok(claims.sub)
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: UserId,
    exp: i64,
}

#[cfg(test)]
mod tests {
    use crate::authentication::{generate_jwt, get_user_id_if_token_is_valid};
    use crate::utils::{HmacSecret, UserId};
    use uuid::Uuid;

    #[test]
    fn token_generated_by_generate_jwt_token_is_valid() {
        let secret: HmacSecret = "very_secret_secret".into();
        let expected_user_id: UserId = UserId::from(Uuid::new_v4());
        let jwt = generate_jwt(&expected_user_id, &secret).unwrap();
        let returned_user_id = get_user_id_if_token_is_valid(&jwt, &secret)
            .expect("Token is invalid");
        
        assert_eq!(expected_user_id, returned_user_id);
    }
}
