use std::{env, future::Future, pin::Pin, sync::Arc};

use actix_web::{http::header::HeaderValue, FromRequest};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

use crate::{
    core::{config::db::init_db, constants::AUTHORIZATION, error::CustomError, types::AppResult},
    features::{
        auth::models::auth_token::AuthToken,
        user::{
            entity::user::UserEntity,
            repository::user::UserRepository,
            service::user::{IUserService, UserService},
        },
    },
};

pub struct AuthMiddleware {
    pub user: UserEntity,
}

impl FromRequest for AuthMiddleware {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<AuthMiddleware, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let user_repo = UserRepository::new(init_db().clone());
        let user_service = UserService::new(Arc::new(user_repo.clone()));

        let auth_header = req
            .headers()
            .get(AUTHORIZATION)
            .cloned()
            .expect("authorization headers must be provided");

        Box::pin(async move {
            if !is_auth_header_valid(&auth_header) {
                return Err(CustomError::UnauthorizedMessage {
                    message: "invalid authorization headers".to_string(),
                });
            }

            let auth_str = auth_header
                .to_str()
                .map_err(|_| CustomError::UnauthorizedMessage {
                    message: "invalid authorization headers".to_string(),
                })?;

            let token = token_extractor(auth_str);
            let token_data = decode_token(&token).map_err(|_| CustomError::Unauthorized)?;

            let user_id = token_data.claims.sub.parse().unwrap();
            let user = user_service.get_user_by_id(user_id).map_err(|_| {
                CustomError::UnauthorizedMessage {
                    message: "user not found".to_string(),
                }
            })?;

            Ok(AuthMiddleware { user })
        })
    }
}

pub fn token_extractor(auth: &str) -> String {
    let bearer_str = auth.split(' ').collect::<Vec<&str>>();
    bearer_str[1].to_owned()
}

pub fn decode_token(jwt: &str) -> AppResult<TokenData<AuthToken>> {
    decode::<AuthToken>(
        jwt,
        &DecodingKey::from_secret(
            env::var("ACCESS_TOKEN_PRIVATE_KEY")
                .expect("ACCESS_TOKEN_PRIVATE_KEY must be set")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| CustomError::Unauthorized)
}

pub fn is_auth_header_valid(auth_header: &HeaderValue) -> bool {
    if let Ok(auth_str) = auth_header.to_str() {
        return auth_str.starts_with("bearer") || auth_str.starts_with("Bearer");
    }
    false
}
