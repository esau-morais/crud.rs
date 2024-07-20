use actix_web::web;

use crate::features::{auth, post};

use super::{health::health_checker, not_found::route_not_found};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0")
        .service(web::resource("/health_checker").route(web::get().to(health_checker)))
        .service(
            web::scope("/auth")
                .service(web::resource("/login").route(web::post().to(auth::controller::login))),
        )
        .service(
            web::scope("/posts")
                .route("", web::post().to(post::controller::create_post))
                .route("", web::get().to(post::controller::get_posts))
                .route("/{id}", web::get().to(post::controller::get_post_by_id))
                .route("/{id}", web::put().to(post::controller::update_post))
                .route("/{id}", web::delete().to(post::controller::delete_post)),
        )
        .default_service(web::route().to(route_not_found));

    cfg.service(scope);
}
