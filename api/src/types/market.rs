use serde::Deserialize;
use validate::Validator;
use crate::utils::{MIN_CONST, MAX_CONST, SYMBOL_REGEX, UTC_REGEX};

#[derive(Deserialize, Validator)]
pub(crate) struct Request {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String
}

#[derive(Deserialize, Validate)]
pub(crate) struct Request {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    
    #[validate(custom(function = "is_valid_kline"))]
    klines_interval: String,

    #[validate(regex(path = *UTC_REGEX))]
    start_time: String,

    #[validate(regex(path = *UTC_REGEX))]
    end_time: Option<String>,
}

