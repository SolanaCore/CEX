
use crate::security::hash::keys::{get_argon2, get_salt};
use argon2::{PasswordHash, PasswordVerifier};

pub fn hash_password(password: &str) -> String {
    let salt = get_salt();
    let hash = get_argon2().hash_password(password.as_bytes(), salt).unwrap();
    hash
}


pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    get_argon2().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

