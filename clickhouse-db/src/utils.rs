use regex::Regex;
use once_cell::sync::Lazy;
use validator::ValidationError;

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


pub fn validate_kline_interval(interval: &str) -> Result<(), ValidationError> {
    if matches!(
        interval,
        "1m" | "3m" | "5m" | "15m" | "30m" |
        "1h" | "2h" | "4h" | "6h" | "8h" | "12h" |
        "1d" | "3d" | "1w" | "1M"
    ) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_interval_passed, accepted intervals are 1m,3m,5m,15m,30m,1h,2h,4h,6h,8h,12h,1d,3d,1w,1M"))
    }
}


pub fn interval_to_group_by(interval: &str) -> &'static str {
    match interval {
        "1m"  => "toStartOfMinute(created_at)",
        "3m"  => "toStartOfMinute(created_at, 3)",
        "5m"  => "toStartOfMinute(created_at, 5)",
        "15m" => "toStartOfMinute(created_at, 15)",
        "30m" => "toStartOfMinute(created_at, 30)",
        "1h"  => "toStartOfHour(created_at)",
        "2h"  => "toStartOfHour(created_at, 2)",
        "4h"  => "toStartOfHour(created_at, 4)",
        "6h"  => "toStartOfHour(created_at, 6)",
        "8h"  => "toStartOfHour(created_at, 8)",
        "12h" => "toStartOfHour(created_at, 12)",
        "1d"  => "toStartOfDay(created_at)",
        "3d"  => "toStartOfDay(created_at, 3)",
        "1w"  => "toStartOfWeek(created_at)",
        "1M"  => "toStartOfMonth(created_at)",
        &_    => "toStartOfHour(created_at)", // default fallback
    }
}