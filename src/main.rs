use std::{env, io};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use rust_crud::{core::config::db::init_db, routes::config::config};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let server_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let server_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let server_url = format!("{}:{}", &server_host, &server_port);

    init_db();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // TODO: create `AppState` to prevent too many clients psql error
            // .app_data(actix_web::web::Data::new(init_db()))
            .configure(config)
    });

    server.bind(&server_url)?.run().await
}
