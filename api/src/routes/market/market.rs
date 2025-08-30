use std::env;

use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};

#[get("/markets")]
async fn markets() {
        HttpResponse::Ok().finish()

}


#[get("/market")]
async fn markets(data: web::json<Request>) {
    pub symbol = data.symbol.clone();

    //call get_market on the postgres_db crate to retrieve the info about that market...
        HttpResponse::Ok().finish()

}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(markets);
    cfg.service(market);
}