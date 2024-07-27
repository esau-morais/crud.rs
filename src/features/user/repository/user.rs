use actix_web::http::StatusCode;
use bcrypt::hash;
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use diesel::ExpressionMethods;

use crate::core::error::CustomError;
use crate::core::types::{AppResult, PsqlConn};
use crate::features::user::entity::user::UserEntity;
use crate::features::user::models::user::NewUser;
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
    fn create(&self, params: NewUser) -> AppResult<String>;
    fn get_user_by_id(&self, user_id: i32) -> AppResult<UserEntity>;
}

impl IUserRepository for UserRepository {
    fn create(&self, mut params: NewUser) -> AppResult<String> {
        params.password =
            hash(params.password, DEFAULT_COST).map_err(|_| CustomError::InternalError)?;

        diesel::insert_into(users)
            .values(&params)
            .execute(&mut self.source.get().unwrap())
            .map(|_| StatusCode::OK.as_u16().to_string())
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => CustomError::BadRequest {
                    message: format!("email {} is already used", params.email),
                },
                _ => CustomError::InternalError,
            })
    }

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
