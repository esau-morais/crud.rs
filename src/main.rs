use std::{env, io};

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use rust_crud::routes::config::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().expect("env variables should be set");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(config));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("please set host in .env");
            let port = env::var("PORT").expect("please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
