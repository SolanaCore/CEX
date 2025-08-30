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



//ENCRYPTION/DEPCRIPTION

use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use anyhow::anyhow;
use data_encoding::HEXLOWER;
use rand::seq::SliceRandom;
use std::str;

#[derive(Debug)]
struct EncryptionKey(String);
#[derive(Debug)]
struct EncryptionNonce(String);

impl From<String> for EncryptionKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}

impl From<String> for EncryptionNonce {
    fn from(nonce: String) -> Self {
        Self(nonce)
    }
}

const RAND_BASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 12;

fn main() -> anyhow::Result<()> {
    let (encryption_key, encryption_nonce) = init()?;
    let key = encryption_key.0.as_bytes();
    let nonce = encryption_nonce.0.as_bytes();

    // contents to be encrypted
    let contents = "plain text".to_string();

    // encryption
    let encrypted_contents =
        aes_encrypt(contents.as_bytes(), key, nonce).map_err(|e| anyhow!(e))?;
    println!("{:?}", encrypted_contents);

    // encode
    let encoded_contents = HEXLOWER.encode(&encrypted_contents);
    println!("{}", encoded_contents);

    // decode
    let decoded_contents = HEXLOWER
        .decode(encoded_contents.as_ref())
        .map_err(|e| anyhow!(e))?;
    println!("{:?}", decoded_contents);

    // decryption
    let plain_text = aes_decrypt(&encrypted_contents, key, nonce).map_err(|e| anyhow!(e))?;
    let decrypted_contents: &str = str::from_utf8(&plain_text)?;
    println!("{}", decrypted_contents);

    assert_eq!(&contents, decrypted_contents);

    Ok(())
}

fn init() -> anyhow::Result<(EncryptionKey, EncryptionNonce)> {
    let key = gen_rand_string(KEY_SIZE)?.into();
    let nonce = gen_rand_string(NONCE_SIZE)?.into();

    println!("{:?}, {:?}", key, nonce);
    Ok((key, nonce))
}

fn gen_rand_string(size: usize) -> anyhow::Result<String> {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        RAND_BASE
            .as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect(),
    )
    .map_err(|e| anyhow!(e))
}

fn aes_encrypt(contents: &[u8], key: &[u8], nonce: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = Nonce::from_slice(nonce);

    // encryption
    let cipher = Aes256Gcm::new(key);
    cipher
        .encrypt(nonce, contents.as_ref())
        .map_err(|e| anyhow!(e))
}

fn aes_decrypt(cipher_text: &[u8], key: &[u8], nonce: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = Nonce::from_slice(nonce);

    // decryption
    let cipher = Aes256Gcm::new(key);
    cipher.decrypt(nonce, cipher_text).map_err(|e| anyhow!(e))
}