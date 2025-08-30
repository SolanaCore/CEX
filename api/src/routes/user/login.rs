use actix_web::{
    cookie::Cookie,
    http::StatusCode,
    post, web, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    types::auth::{SigninData, SigninPayload, Claims},
    utils::{KEYS, hash_password, get_token, add_cookie},
};

use postgres::{
    DB_POOL,
    DbPool as Pool,
    models::{NewUser},
    schema::users,
};


#[post("/api/login")]
async fn login(db_conn: web::Data<Pool>, form: web::Json<SigninData>) -> impl Responder {
    let conn = db_conn.get().expect("Failed to get DB connection");

    // Extract config (default 1h = 3600s)
    let expiration_time = env::var("TOKEN_EXPIRATION_TIME")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(3600);

    // Check user existence
    let user_result = users
        .filter(email.eq(&form.email))
        .filter(hash_password.eq(hash_password(&form.password)))
        .first::<User>(&conn);

    match user_result {
        Ok(user) => {
            // Build claims
            let claims = Claims {
                user_id: user.username.clone(),
                expiration_time,
            };

            // Generate token
            let token = get_token(&claims, *KEYS);

            // Add cookie (example: HttpOnly, secure if using HTTPS)
            let _ = add_cookie("token", &token, "/");

            let response = SigninPayload {
                status_code: StatusCode::OK.as_u16(),
                username: Some(user.username),
                password: None, // never expose
                token: Some(token),
                error: None,
            };

            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let response = SigninPayload {
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                username: None,
                password: None,
                token: None,
                error: Some("Invalid username or password".into()),
            };
            HttpResponse::Unauthorized().json(response)
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}