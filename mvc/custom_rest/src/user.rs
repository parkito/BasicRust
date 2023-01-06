use std::sync::Arc;
use actix_web::{HttpResponse, post, get, delete};
use custom_logger::Logger;
use custom_logger::LogFactory;
use std::{env, io};
use actix_web::web::Json;
use log::info;
use custom_api::UserPersonalDataDto;
use serde_json::json;
use crate::constants::APPLICATION_JSON;

#[post("/user")]
pub async fn register(user: Json<UserPersonalDataDto>) -> HttpResponse {
    info!("This is log {}", user.id);
    HttpResponse::Ok().finish()
}

#[delete("/user")]
pub async fn send() -> HttpResponse {
    HttpResponse::Ok()
        .json("{\"var\":\"value\"}".to_string())
}


