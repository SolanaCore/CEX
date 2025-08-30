use serde::{Deserialize, Serialize};
use std::env;

use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};

#[get("depth")]
async fn depth(data:web::Json<Request> ) {
    data.validate().map_err(|e| {
        HttpResponse::BadRequest().json(format!("Invalid request: {}", e))
    })?;

    let symbol = data.symbol.clone();
    let limit = data.limit.unwrap_or(100);

    //cache the symbol for a cex and verify it exists in the db... & then send redis the request to get the depth for that symbol...
    //add redis rate limiter 
        HttpResponse::Ok().finish()

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(depth);
}