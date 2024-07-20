use actix_cors::Cors;
use actix_web::http::header;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_origin("http://localhost:3000")
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::CONTENT_ENCODING,
            header::ACCEPT,
        ])
        .supports_credentials()
}
