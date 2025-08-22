use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;
use crate::utils::{validate_utc_time_format};


pub(crate) struct Response {
    
} 
#[get("/api/v1/historic_trades")]
async fn historic_trades(data: web::json<Request>) {
    let symbol = data.symbol.clone();
    let limit = data.limit.unwrap_or(100);
        HttpResponse::Ok().finish()

}
