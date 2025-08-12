use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use actix_web::http::StatusCode;

#[derive(Deserialize)]
pub(crate) struct SigninData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub(crate) struct SigninPayload {
    status: u16,
    username: Option<String>, // Some(username) if success, None if error
    jwt: Option<String>,      // JWT token on success
    error: Option<String>,    // Error message on failure
}

#[post("/api/signin")]
async fn signin(data: web::Json<SigninData>) -> impl Responder {
    let username = data.username.clone();
    let password = data.password.clone();

    let secret = match env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(SigninPayload {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    username: None,
                    jwt: None,
                    error: Some("JWT_SECRET not set".to_string()),
                });
        }
    };

    // TODO: Verify username and password from DB here
    let is_valid_user = true; // replace with real check

    let response = if is_valid_user {
        // TODO: generate JWT token using `secret` and user info
        let token = "dummy.jwt.token".to_string();

        SigninPayload {
            status: StatusCode::OK.as_u16(),
            username: Some(username),
            jwt: Some(token),
            error: None,
        }
    } else {
        SigninPayload {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            username: None,
            jwt: None,
            error: Some("Invalid username or password".to_string()),
        }
    };

    HttpResponse::Ok().json(response)
}
