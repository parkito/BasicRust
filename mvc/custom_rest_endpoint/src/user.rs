use std::sync::Arc;
use actix_web::{HttpResponse, post, get};
use custom_logger::Logger;
use custom_logger::LogFactory;
use std::{env, io};
use log::info;
use crate::constants::APPLICATION_JSON;


#[post("/receive")]
pub async fn receive(mess: String) -> HttpResponse {
    info!("This is log {}", mess.to_string());
    // log()
    // LOG.info(mess.as_str());
    HttpResponse::Ok().finish()
}

#[get("/send")]
pub async fn send() -> HttpResponse {
    HttpResponse::Ok()
        .json("{\"var\":\"value\"}".to_string())
}
