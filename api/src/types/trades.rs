use validate::Validator;
use serde::Deserialize;
use crate::utils::{MIN_CONST, MAX_CONST, UTC_REGEX}
#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
    symbol: String,
    limit: Option<u16>,  //User may or may not provide the limit, so we take 50 as default and 999 max..
    #[validate(regex(path = *UTC_REGEX))]
    start_time: String, 
    #[validate(regex(path = *UTC_REGEX))]
    end_time: Option<String>,
}


#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(custom(function = "is_valid_symbol"))]
    symbol: String
}


#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(custom(function = "is_valid_symbol"))]
    symbol: String
}


pub fn is_valid_symbol(symbol: &str) -> Result<(), ValidationError> {
    // Regex: only allows uppercase letters + "/" + uppercase letters
    let re = Regex::new(r"^[A-Z]+/[A-Z]+$").unwrap();

    if re.is_match(symbol) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_symbol"))
    }
}