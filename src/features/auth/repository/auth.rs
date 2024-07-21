use crate::{
    core::{
        error::CustomError,
        types::{AppResult, PsqlConn},
    },
    features::{
        auth::{
            entity::auth::AuthEntity,
            models::{
                auth_token::AuthToken,
                login::{Login, LoginInfo},
            },
        },
        user::models::user::User,
    },
    schema::users::{self, dsl::*},
};
use bcrypt::verify;
use diesel::{prelude::*, ExpressionMethods, RunQueryDsl};

#[derive(Clone)]
pub struct AuthRepository {
    source: PsqlConn,
}

impl AuthRepository {
    pub fn new(source: PsqlConn) -> Self {
        AuthRepository { source }
    }
}

pub trait IAuthRepository: Send + Sync {
    fn login(&self, params: Login) -> AppResult<AuthEntity>;
}

impl IAuthRepository for AuthRepository {
    fn login(&self, params: Login) -> AppResult<AuthEntity> {
        users::table
            .filter(email.eq(&params.email))
            .get_result::<User>(&mut self.source.get().unwrap())
            .map(|u| {
                let login_info = LoginInfo {
                    id: u.id,
                    email: u.email,
                    password: u.password,
                };

                (!&login_info.password.is_empty()
                    && verify(params.password.as_bytes(), &login_info.password).unwrap())
                .then(|| {
                    AuthToken::generate_token(&login_info)
                        .map(AuthEntity::new)
                        .map_err(|_| CustomError::InternalError)
                })
                .unwrap_or(Err(CustomError::InvalidCredentials))
            })
            .map_err(|_| CustomError::UserNotFoundError)?
    }
}
