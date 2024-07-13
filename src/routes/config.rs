use actix_web::web;

use super::post::{create_post, delete_post, get_posts};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0")
        .service(create_post)
        .service(get_posts)
        .service(delete_post);

    cfg.service(scope);
}
