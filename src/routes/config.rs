use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
