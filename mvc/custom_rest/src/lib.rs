mod user;
mod constants;

use std::{env, io};
use actix_web::{App, HttpServer, middleware};
use log::info;
use custom_common::CustomAppContext;
use custom_logger::Level::DEBUG;
use custom_logger::LogFactory;

#[actix_rt::main]
pub async fn run_server(context: CustomAppContext) -> io::Result<()> {
    //todo use service instead of context
    info!("Running web server on {}",context.props.port);
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(user::register)
            .service(user::send)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}