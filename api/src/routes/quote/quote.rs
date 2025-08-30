use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};

#[get("/quote")]
async fn quote(){

}

pub fn quote_config(cfg: &mut web::ServiceConfig) {
    cfg.service(quote);
}