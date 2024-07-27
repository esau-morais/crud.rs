use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::core::{config::db::init_db, response::ResponseBody, types::AppResult};

use super::{
    models::user::NewUser,
    repository::user::UserRepository,
    service::user::{IUserService, UserService},
};

pub async fn register(params: web::Json<NewUser>) -> AppResult<HttpResponse> {
    let user_repo = UserRepository::new(init_db().clone());
    let user_service = UserService::new(Arc::new(user_repo.clone()));

    user_service
        .create(params.into_inner())
        .map(|_| ResponseBody::<()>::success(None).into())
}
