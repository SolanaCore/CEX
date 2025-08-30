use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use jsonwebtoken::{DecodingKey, EncodingKey, encode, Header};
use uuid::Uuid;

//JWT CLAIMS
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,  // user id
    expiration_time: usize,   // expiration time
}

//JWT KEY
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new() -> Self {
        Self {
            encoding: EncodingKey::from_secret(env::var(encoding_secret)),
            decoding: DecodingKey::from_secret(env::var(decoding_secret)),
        }
    }
}

//STATIC 
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    Keys::new()
});