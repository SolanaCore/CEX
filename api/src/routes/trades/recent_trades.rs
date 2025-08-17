use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;


pub(crate) struct Response {
    
} 
#[get("/api/v1/recent_trades")]
async fn recent_trade() {

        HttpResponse::Ok().finish()

}