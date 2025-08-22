use regex::Regex;
use std::env;
use once_cell::sync::Lazy;

//constants
pub const MIN_CONST: u64 = 1;
pub const MAX_CONST: u64 = 25;

//static 
pub static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,10}/[A-Z]{2,10}$").unwrap()
});

pub static UTC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap()
});

pub fn validate_kline_interval(interval: &str) -> bool {
    matches!(
        interval,
        "1m" | "3m" | "5m" | "15m" | "30m" |
        "1h" | "2h" | "4h" | "6h" | "8h" | "12h" |
        "1d" | "3d" | "1w" | "1M"
    )
}
