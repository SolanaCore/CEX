use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;

//one for both the api's
pub(crate) struct Response {
    
} 

#[get("/api/v1/markets")]
async fn markets() {
        HttpResponse::Ok().finish()

}

// used (crate) this would allow other parts of your crate to access it, but keep it hidden from external users or other crates that depend on your crate.



#[get("/api/v1/market")]
async fn markets(data: web::json<Request>) {
    pub symbol = data.symbol.clone();

    //call get_market on the postgres_db crate to retrieve the info about that market...
        HttpResponse::Ok().finish()

}
