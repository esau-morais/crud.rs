use std::sync::Arc;

use crate::{
    core::{error::CustomError, types::AppResult},
    features::user::{
        entity::user::UserEntity, models::user::NewUser, repository::user::IUserRepository,
    },
};

#[derive(Clone)]
pub struct UserService {
    pub user_repo: Arc<dyn IUserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self { user_repo }
    }
}

pub trait IUserService: Send + Sync {
    fn create(&self, params: NewUser) -> AppResult<String>;
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity>;
}

impl IUserService for UserService {
    fn create(&self, params: NewUser) -> AppResult<String> {
        self.user_repo
            .create(params)
            .map_err(|e| CustomError::BadRequest {
                message: e.to_string(),
            })
    }
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity> {
        self.user_repo.get_user_by_id(user_id)
    }
}
