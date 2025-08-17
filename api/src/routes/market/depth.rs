use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;



pub(crate) struct Response {
    
} 

#[get("/api/v1/depth")]
async fn depth(data:web::Json<Request> ) {
    let symbol = data.symbol.clone();

    //query the db to return the depth of the orderbook for the particular symbol
    HttpResponse::Ok().finish()

}