use serde::{Deserialize, Serialize};
use std::env;

use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};


#[get("/recent_trades")]
async fn recent_trade() {

        HttpResponse::Ok().finish()

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(recent_trade);
}