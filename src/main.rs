use std::{env, io};

use actix_web::{middleware::Logger, App, HttpServer};
use rust_crud::{
    core::{config::db::init_db, middlewares::cors::cors},
    routes::config::config,
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let server_host = env::var("APP_HOST").unwrap_or(String::from("127.0.0.1"));
    let server_port = env::var("APP_PORT").unwrap_or(String::from("8080"));
    let server_url = format!("{}:{}", &server_host, &server_port);

    init_db();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            // TODO: create `AppState` to prevent too many clients psql error
            // .app_data(actix_web::web::Data::new(init_db()))
            .configure(config)
    });

    server.bind(&server_url)?.run().await
}
