use actix_web::web;

use super::post::{create_post, delete_post, get_post_by_id, get_posts};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0")
        .service(create_post)
        .service(get_posts)
        .service(get_post_by_id)
        .service(delete_post);

    cfg.service(scope);
}
