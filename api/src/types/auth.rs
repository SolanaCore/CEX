use validator::Validate;
use serde::Deserialize;
use crate::utils::{MIN_CONST, MAX_CONST, check_if_exist}


//Request 
#[derive(Deserialize, Validate)]
pub(crate) struct LoginRequest {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"),  custom(function = "check_if_exist"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
    #[validate(email)]
    email: String,
}


#[derive(Deserialize, Validate)]
pub(crate) struct RegisterRequest {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"), custom(function = "check_if_exist"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
    #[validate(email)]
    email: String,
}


//Response 
#[derive(Serialize, Debug)]
pub(crate) struct RegisterResponse {
    username: Option<String>, // None if error
    password: Option<String>, // None if error, but usually don't send password back
    token: Option<String>,    // JWT token on success
    error: Option<String>,    // Error message if any
}


#[derive(Serialize)]
pub(crate) struct LoginResponse {
    username: Option<String>, // Some(username) if success, None if error
    jwt: Option<String>,      // JWT token on success
    expiration_time: DateTime<Utc>, // Token expiration time
    error: Option<String>,    // Error message on failure
}

