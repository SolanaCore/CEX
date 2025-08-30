use jsonwebtoken::{encode, Header};
use std::env;
use crate::security::jwt::{
    keys::{Claims, Keys},
};

pub fn encode_jwt(claims: &Claims, keys: &Keys) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), claims, &keys.encoding)
}
