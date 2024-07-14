use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{
    models::post::{NewPost, UpdatePost},
    services::post::{create, delete, read, update},
};

#[post("/posts/create")]
pub async fn create_post_route(post: web::Json<NewPost>) -> HttpResponse {
    create::create_post(post).await
}

#[get("/posts")]
pub async fn get_posts_route() -> HttpResponse {
    read::get_posts().await
}

#[get("/posts/{id}")]
pub async fn get_post_by_id_route(path: web::Path<i32>) -> HttpResponse {
    read::get_post_by_id(path).await
}

#[put("/posts/{id}")]
pub async fn update_post_route(path: web::Path<i32>, post: web::Json<UpdatePost>) -> HttpResponse {
    update::update_post(path, post).await
}

#[delete("/posts/{id}")]
pub async fn delete_post_route(path: web::Path<i32>) -> HttpResponse {
    delete::delete_post(path).await
}
