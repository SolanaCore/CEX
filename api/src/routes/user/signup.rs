use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::Salt};
use crate::utils::hash_password;
#[derive(Deserialize)]
pub(crate) struct SignupData {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct SignupPayload {
    status_code: u16,
    username: Option<String>, // None if error
    password: Option<String>, // None if error, but usually don't send password back
    token: Option<String>,    // JWT token on success
    error: Option<String>,    // Error message if any
}

#[post("/api/signup")]
async fn signup(data: web::Json<SignupData>) -> impl Responder {
    let username = data.username.clone();
    let password = data.password;
    let hash_password = hash_password(&password);
    // TODO: Store user credentials in DB, handle errors
    // Simulate success or failure condition (replace with real logic)
    let is_success = true;
    
    let response = if is_success {
        SignupPayload {
            status_code: StatusCode::OK.as_u16(),
            username: Some(username),
            password: None, // Never send password back!
            token: Some("dummy.jwt.token".to_string()),
            error: None,
        }
    } else {
        SignupPayload {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            username: None,
            password: None,
            token: None,
            error: Some("Signup failed due to XYZ reason".to_string()),
        }
    };

    HttpResponse::Ok().json(response)
}
