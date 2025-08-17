use validator::Validate;
use serde::Deserialize;
use crate::utils::{MIN_CONST, MAX_CONST, check_if_exist}


#[derive(Deserialize, Validate)]
pub(crate) struct SigninData {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"),  custom(function = "check_if_exist"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
}


#[derive(Deserialize, Validate)]
pub(crate) struct SignupData {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"), custom(function = "check_if_exist"))]
    username: String,
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    password: String,
    #[validate(email)]
    email: String,
}

