use actix_web::web;

use crate::features::{auth, post, user};

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
                .service(
                    web::resource("")
                        .route(web::post().to(post::controller::create_post))
                        .route(web::get().to(post::controller::get_posts)),
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(post::controller::get_post_by_id))
                        .route(web::put().to(post::controller::update_post))
                        .route(web::delete().to(post::controller::delete_post)),
                ),
        )
        .service(web::resource("/user").route(web::post().to(user::controller::register)));

    cfg.service(scope)
        .default_service(web::route().to(route_not_found));
}
