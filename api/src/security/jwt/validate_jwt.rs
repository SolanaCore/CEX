use jsonwebtoken::{Validation, encode, Header};
use std::env;
use crate::security::jwt::{
    keys::{Claims, Keys},
};

use postgres::{
    utils::PostgresInvoker,
    connection::DB_POOL,
};

pub fn validate_jwt(token: &str, keys: &Keys) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode_jwt(token, keys)?;
    //db check user id exists --- TODO
    let user_id = claims.user_id;
    let db_invoker = PostgresInvoker::new(DB_POOL);
    db_invoker.get_user(user_id.parse().unwrap())?;
    Ok(token_data.claims)
}