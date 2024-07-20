use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::core::{config::db::init_db, response::ResponseBody, types::AppResult};

use super::{
    models::login::Login,
    repository::auth::AuthRepository,
    service::auth::{AuthService, IAuthService},
};

pub async fn login(params: web::Json<Login>) -> AppResult<HttpResponse> {
    let auth_repo = AuthRepository::new(init_db().clone());
    let auth_service = AuthService::new(Arc::new(auth_repo.clone()));

    auth_service
        .login(params.into_inner())
        .map(|_| ResponseBody::<()>::success(None).into())
}
