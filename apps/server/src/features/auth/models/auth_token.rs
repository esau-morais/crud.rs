use std::env;

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
        let private_key =
            env::var("ACCESS_TOKEN_PRIVATE_KEY").expect("ACCESS_TOKEN_PRIVATE_KEY must be set");

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
            &EncodingKey::from_secret(private_key.as_ref()),
        )
        .map_err(|_| CustomError::InternalError)
    }
}
