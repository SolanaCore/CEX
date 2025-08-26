use crate::utils::{UTC_REGEX};
use validator::Validate;
use crate::utils::{validate_kline_interval};
use clickhouse::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate, Row)]
pub struct Kline {
    #[validate(regex(path = *UTC_REGEX))]
    pub interval_time: String,
    #[validate(range(min = 0.0))]
    pub open: f64,
    #[validate(range(min = 0.0))]
    pub high: f64,
    #[validate(range(min = 0.0))]
    pub low: f64,
    #[validate(range(min = 0.0))]
    pub close: f64,
    #[validate(range(min = 0.0))]
    pub avg_price: f64,
}

impl Kline {
    pub fn column_names() -> Vec<&'static str> {
        vec![
            "interval_time",
            "open",
            "high",
            "low",
            "close",
            "avg_price",
        ]
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct GetKlines {
    pub symbol: String,
    #[validate(regex(path = *UTC_REGEX))]
    pub start: String,
    #[validate(regex(path = *UTC_REGEX))]
    pub end: String,
    #[validate(custom(function = "validate_kline_interval"))]
    pub interval: String,
}
