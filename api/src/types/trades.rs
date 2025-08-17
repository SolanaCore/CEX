use validate::Validator;
use serde::Deserialize;
use crate::utils::{MIN_CONST, MAX_CONST, UTC_REGEX, SYMBOL_REGEX}
#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    limit: Option<u16>,  //User may or may not provide the limit, so we take 50 as default and 999 max..
    #[validate(regex(path = *UTC_REGEX))]
    start_time: String, 
    #[validate(regex(path = *UTC_REGEX))]
    end_time: Option<String>,
}


#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String
}


#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String
}
