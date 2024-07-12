use std::{env, io};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use listenfd::ListenFd;
use rust_crud::{
    db,
    routes::{config::config, health},
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
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
            .app_data(actix_web::web::Data::new(db::init()))
            .service(health::check_health)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("please set host in .env");
            let port = env::var("PORT").expect("please set port in .env");
            server
                .bind(format!("{}:{}", host, port))?
                .max_connections(10)
        }
    };

    server.run().await
}
