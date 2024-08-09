use std::sync::Arc;

use crate::{
    core::{error::CustomError, types::AppResult},
    features::auth::{
        entity::auth::AuthEntity, models::login::Login, repository::auth::IAuthRepository,
    },
};

#[derive(Clone)]
pub struct AuthService {
    pub auth_repo: Arc<dyn IAuthRepository>,
}

impl AuthService {
    pub fn new(auth_repo: Arc<dyn IAuthRepository>) -> Self {
        Self { auth_repo }
    }
}

pub trait IAuthService: Send + Sync {
    fn login(&self, params: Login) -> AppResult<AuthEntity>;
}

impl IAuthService for AuthService {
    fn login(&self, params: Login) -> AppResult<AuthEntity> {
        self.auth_repo
            .login(params)
            .map_err(|e| CustomError::BadRequest {
                message: e.to_string(),
            })
    }
}
