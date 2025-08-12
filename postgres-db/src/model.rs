use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub wallet_pubkey: String,
    pub wallet_privkey_enc: Vec<u8>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub wallet_pubkey: &'a str,
    pub wallet_privkey_enc: &'a [u8],
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::user_balances)]
#[diesel(belongs_to(User))]
pub struct UserBalance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_mint: String,
    pub available_balance: BigDecimal,
    pub locked_balance: BigDecimal,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::user_balances)]
pub struct NewUserBalance<'a> {
    pub user_id: Uuid,
    pub token_mint: &'a str,
    pub available_balance: BigDecimal,
    pub locked_balance: BigDecimal,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::orders)]
#[diesel(belongs_to(User))]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub filled_quantity: BigDecimal,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::orders)]
pub struct NewOrder<'a> {
    pub user_id: Uuid,
    pub symbol: &'a str,
    pub side: &'a str,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub filled_quantity: BigDecimal,
    pub status: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
