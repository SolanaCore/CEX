use regex::Regex;
use once_cell::sync::Lazy;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::env;
//Argon2 -> for password hashing in rust...
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::Salt};

//JWT STUFF 
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new() -> Self {
        Self {
            encoding: EncodingKey::from_secret(env::var(secret)),
            decoding: DecodingKey::from_secret(env::var(secret)),
        }
    }
}

//constants
pub const MIN_CONST: u64 = 1;
pub const MAX_CONST: u64 = 25;

//STATIC 
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    Keys::new()
});

pub static ARGON2: Lazy<Argon2> = Lazy::new(|| {
    Argon2::default()
})

pub static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,10}/[A-Z]{2,10}$").unwrap()
});

pub static UTC_REGEX - LAZY::new(|| {
    Regex::new(r"[a-z]{2}$").unwrap()
});


pub fn hash_password(password: &str) {
    let hash = (ARGON2.get()).hash_password(password.as_bytes(), salt).unwrap();
    hash
}

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
