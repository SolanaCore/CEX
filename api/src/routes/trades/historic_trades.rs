use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    utils::{validate_utc_time_format},
}

use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};


#[get("/historic_trades")]
async fn historic_trades(data: web::json<Request>) {
    let symbol = data.symbol.clone();
    let limit = data.limit.unwrap_or(100);
        HttpResponse::Ok().finish()

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(historic_trades);
}