use actix_web::{post, web, HttpResponse, Responder, cookie::Cookie};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;
use jsonwebtoken::encode;
use crate::utils::KEYS;
use crate::postgres::schema::users
use validator::validate;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,  // user id
    exp: usize,   // expiration time
}


#[derive(Serialize)]
pub(crate) struct SigninPayload {
    status: u16,
    username: Option<String>, // Some(username) if success, None if error
    jwt: Option<String>,      // JWT token on success
    error: Option<String>,    // Error message on failure
}


#[post("/api/signin")]
async fn signin(db_conn:web::Data<DbPool>, web::Json<SigninData>) -> impl Responder {
    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1)).timestamp() as usize;
    users.filter(email.eq(&form.email), hash_password.eq(hash_password(&data.password))).first::<User>(conn);
    let claims = Claims {
        username: data.username,
        exp,
    };

     let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(
        |_| AuthError::TokenCreation
    )?;
    let cookie = Cookie::build("token", token)
                        .path("/api")
                        .secure(true)
                        .http_only(true)
                        .finish();


    HttpResponse::Ok().json(response)
}
