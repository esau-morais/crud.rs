use std::sync::Arc;

use actix_web::{http::StatusCode, web, HttpResponse};

use crate::core::{
    config::db::init_db,
    middlewares::auth::AuthMiddleware,
    response::{Diagnostic, ResponseBody},
    types::AppResult,
};

use super::{
    models::post::{NewPost, UpdatePost},
    repository::post::PostRepository,
    service::post::{IPostService, PostService},
};

pub async fn create_post(
    auth: AuthMiddleware,
    params: web::Json<NewPost>,
) -> AppResult<HttpResponse> {
    let post_repo = PostRepository::new(init_db().clone());
    let post_service = PostService::new(Arc::new(post_repo.clone()));

    post_service
        .create_post(auth.user.id, params.into_inner())
        .map(|_| ResponseBody::<()>::success(None).into())
}

pub async fn get_posts() -> AppResult<HttpResponse> {
    let post_repo = PostRepository::new(init_db().clone());
    let post_service = PostService::new(Arc::new(post_repo.clone()));

    post_service
        .get_posts()
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn get_post_by_id(path: web::Path<i32>) -> AppResult<HttpResponse> {
    let post_repo = PostRepository::new(init_db().clone());
    let post_service = PostService::new(Arc::new(post_repo.clone()));

    post_service
        .get_post_by_id(path.into_inner())
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn update_post(
    path: web::Path<i32>,
    params: web::Json<UpdatePost>,
) -> AppResult<HttpResponse> {
    let post_repo = PostRepository::new(init_db().clone());
    let post_service = PostService::new(Arc::new(post_repo.clone()));

    post_service
        .update_post(path.into_inner(), params.into_inner())
        .map(|data| ResponseBody::success(Some(data)).into())
}

pub async fn delete_post(path: web::Path<i32>) -> AppResult<HttpResponse> {
    let post_repo = PostRepository::new(init_db().clone());
    let post_service = PostService::new(Arc::new(post_repo.clone()));

    post_service.delete_post(path.into_inner()).map(|data| {
        ResponseBody::<()>::new(
            Diagnostic::new(&StatusCode::OK.as_u16().to_string(), data.as_str()),
            None,
        )
        .into()
    })
}
