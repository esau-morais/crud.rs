use actix_web::web;

use super::post::{
    create_post_route, delete_post_route, get_post_by_id_route, get_posts_route, update_post_route,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0")
        .service(create_post_route)
        .service(get_posts_route)
        .service(get_post_by_id_route)
        .service(update_post_route)
        .service(delete_post_route);

    cfg.service(scope);
}
