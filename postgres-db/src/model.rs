use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
<<<<<<< Updated upstream
use chrono::{NaiveDateTime, DateTime, Utc};
use bigdecimal::BigDecimal;

use crate::schema::{trades, orders, users, user_balances};
=======
use chrono::NaiveDateTime;
use crate::trades::dsl::{trades, orders, users, user_balances};
>>>>>>> Stashed changes

// ---------------- USERS ----------------
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub wallet_pubkey: String,
    pub wallet_privkey_enc: Vec<u8>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub wallet_pubkey: String,
    pub wallet_privkey_enc: Vec<u8>,
}

// ---------------- BALANCES ----------------
<<<<<<< Updated upstream
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
=======
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
>>>>>>> Stashed changes
#[diesel(table_name = user_balances)]
#[diesel(belongs_to(User))]
pub struct UserBalance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_mint: String,
    pub available_balance: BigDecimal,
    pub locked_balance: BigDecimal,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_balances)]
<<<<<<< Updated upstream
pub struct NewUserBalance {
=======
pub struct NewBalance {
>>>>>>> Stashed changes
    pub user_id: Uuid,
    pub token_mint: String,
    pub available_balance: BigDecimal,
    pub locked_balance: BigDecimal,
}

// ---------------- ORDERS ----------------
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = Order)]
#[diesel(belongs_to(User))]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String, // BUY / SELL
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub filled_quantity: BigDecimal,
    pub status: String, // OPEN, PARTIALLY_FILLED, FILLED, CANCELLED
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = Order)]
pub struct NewOrder {
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub status: String,
}

// ---------------- TRADES ----------------
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
<<<<<<< Updated upstream
#[diesel(table_name = trades)]
=======
#[diesel(table_name = Trade)]
#[diesel(belongs_to(Order, foreign_key = buy_order_id))]
#[diesel(belongs_to(Order, foreign_key = sell_order_id))]
>>>>>>> Stashed changes
pub struct Trade {
    pub id: Uuid,
    pub is_buyer_maker: bool,
    pub price: BigDecimal,
    pub qty: BigDecimal,
    pub symbol: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = Trade)]
pub struct NewTrade {
    pub is_buyer_maker: bool,
    pub price: BigDecimal,
    pub qty: BigDecimal,
    pub symbol: String,
}
