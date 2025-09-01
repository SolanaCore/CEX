use regex::Regex;

use std::env;
//Argon2 -> for password hashing in rust...

//constants
pub const MIN_CONST: u64 = 1;
pub const MAX_CONST: u64 = 25;



pub static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,10}/[A-Z]{2,10}$").unwrap()
});

pub static UTC_REGEX - LAZY::new(|| {
    Regex::new(r"[a-z]{2}$").unwrap()
});




pub fn is_valid_kline(klines_interval: &str) -> Result<(), ValidationError> {
    // Allowed kline intervals
    let allowed_intervals = [
        "1m", "3m", "5m", "15m", "30m",
        "1h", "2h", "4h", "6h", "8h", "12h",
        "1d", "3d",
        "1w",
        "1M",
    ];

    if allowed_intervals.contains(&klines_interval) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_kline_interval"))
    }
}



pub fn check_if_exist(username: &str) {
    
}



fn default_end_time() -> Some(DateTime<Utc>) {
    Some(Utc::now());
}


pub fn add_cookie(placeholder: &str, payload, path: &str) -> Cookie {
    let cookie = Cookie::build(placeholder, payload)
                        .path(path)
                        .secure(true)
                        .http_only(true)
                        .finish();
                        cookie
}

pub fn get_token(claims: &Claims, KEYS: Keys) -> Result<String, AuthError> {
     encode(&Header::default(), &claims, &KEYS.encoding).map_err(
        |_| err!("Token creation failed")
    )?;
}
