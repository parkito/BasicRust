mod user;
mod constants;

use std::{env, io};
use actix_web::{App, HttpServer, middleware};
use custom_logger::Level::DEBUG;
use custom_logger::LogFactory;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(user::receive)
            .service(user::send)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}