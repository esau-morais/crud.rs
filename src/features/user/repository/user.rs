use diesel::prelude::*;
use diesel::ExpressionMethods;

use crate::core::error::CustomError;
use crate::core::types::{AppResult, PsqlConn};
use crate::features::user::entity::user::UserEntity;
use crate::features::user::models::user::User;
use crate::schema::users::{self, dsl::*};

#[derive(Clone)]
pub struct UserRepository {
    source: PsqlConn,
}

impl UserRepository {
    pub fn new(source: PsqlConn) -> Self {
        UserRepository { source }
    }
}

pub trait IUserRepository: Send + Sync {
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity>;
}

impl IUserRepository for UserRepository {
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity> {
        users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map(|u| UserEntity {
                id: u.id,
                email: u.email,
                name: u.name,
                role: u.role,
            })
            .map_err(|_| CustomError::UserNotFoundError)
    }
}
