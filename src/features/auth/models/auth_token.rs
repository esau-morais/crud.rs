use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::core::{error::CustomError, types::AppResult};

use super::login::LoginInfo;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}

impl AuthToken {
    pub fn generate_token(login: &LoginInfo) -> AppResult<String> {
        let bytes_private_key = general_purpose::STANDARD
            .decode(
                env::var("ACCESS_TOKEN_PRIVATE_KEY").expect("ACCESS_TOKEN_PRIVATE_KEY must be set"),
            )
            .unwrap();
        let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();

        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(login.password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| CustomError::InternalError)?;

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::minutes(60)).timestamp();
        let payload = AuthToken {
            sub: login.id.to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(decoded_private_key.as_ref()),
        )
        .map_err(|_| CustomError::InternalError)
    }
}
