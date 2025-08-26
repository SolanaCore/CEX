pub mod kline;
pub use kline::*;

use validator::Validate;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::utils::SYMBOL_REGEX;

#[derive(Debug, Clone, PartialEq, Validate, Serialize, Deserialize)]
pub struct InsertPayload {
    pub id: uuid::Uuid,
    #[validate(regex(path = *SYMBOL_REGEX))]
    pub symbol: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub created_at: DateTime<Utc>,
}