use actix_web::web;

use crate::services::{
    post::{
        create::create_post,
        delete::delete_post,
        read::{get_post_by_id, get_posts},
        update::update_post,
    },
    user::create::create_user,
};

use super::{health::health_checker, not_found::route_not_found};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0")
        .service(web::resource("/health_checker").route(web::get().to(health_checker)))
        .service(
            web::scope("/posts")
                .route("", web::post().to(create_post))
                .route("", web::get().to(get_posts))
                .route("/{id}", web::get().to(get_post_by_id))
                .route("/{id}", web::put().to(update_post))
                .route("/{id}", web::delete().to(delete_post)),
        )
        .service(web::resource("/user").route(web::post().to(create_user)))
        .default_service(web::route().to(route_not_found));

    cfg.service(scope);
}
