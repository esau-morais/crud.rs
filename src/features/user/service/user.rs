use std::sync::Arc;

use crate::{
    core::types::AppResult,
    features::user::{entity::user::UserEntity, repository::user::IUserRepository},
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
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity>;
}

impl IUserService for UserService {
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity> {
        self.user_repo.get_user_by_id(user_id)
    }
}
