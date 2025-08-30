use jsonwebtoken::{Validation, encode, Header};
use std::env;
use crate::security::jwt::{
    keys::{Claims, Keys},
};

pub fn decode_jwt(token: &str, keys: &Keys) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &keys.decoding,
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}