use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/v0");

    cfg.service(scope);
}
