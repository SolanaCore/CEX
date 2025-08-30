use serde::{Deserialize, Serialize};
use chrono::{Utc};

use crate::{
    utils::{validate_utc_time_format}
};

use actix_web::{
    http::StatusCode,
    web, post, HttpResponse, Responder, cookie::Cookie
};


#[get("/klines")]
async fn klines(data: web::Json<Request>) -> impl Responder {
    let symbol = data.symbol.clone();
    let klines_interval = data.klines_interval.clone();

    if !validate_utc_time_format(&data.start_time) {
        return HttpResponse::BadRequest().json("Invalid start_time format. Expected format: YYYY-MM-DDTHH:mm:ssZ");
    }

    let end_time = if let Some(end_time_str) = &data.end_time {
        if !validate_utc_time_format(end_time_str) {
            return HttpResponse::BadRequest().json("Invalid end_time format. Expected format: YYYY-MM-DDTHH:mm:ssZ");
        }
        end_time_str.clone()
    } else {
        //RFC 3339 is a standard for representing date and time in a human-readable string format, based on ISO 8601. It is commonly used in APIs and data interchange.
        Utc::now().to_rfc3339()
    };

    HttpResponse::Ok().json(Response)
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(klines);
}