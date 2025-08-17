use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

// ---------------- USERS ----------------
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

// Insertable for creating new users
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// ---------------- BALANCES ----------------
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = balances)]
#[diesel(belongs_to(User))]
pub struct Balance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset: String,
    pub amount: bigdecimal::BigDecimal,
}

// Insertable for balances
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = balances)]
pub struct NewBalance {
    pub user_id: Uuid,
    pub asset: String,
    pub amount: bigdecimal::BigDecimal,
}

// ---------------- ORDERS ----------------
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
#[diesel(belongs_to(User))]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String, // "BUY" or "SELL"
    pub price: bigdecimal::BigDecimal,
    pub quantity: bigdecimal::BigDecimal,
    pub filled_quantity: bigdecimal::BigDecimal,
    pub status: String, // "OPEN", "PARTIALLY_FILLED", "FILLED", "CANCELLED"
    pub created_at: NaiveDateTime,
}

// Insertable for orders
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub price: bigdecimal::BigDecimal,
    pub quantity: bigdecimal::BigDecimal,
    pub status: String,
}

// ---------------- TRADES ----------------
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = trades)]
#[diesel(belongs_to(Order, foreign_key = buy_order_id))]
#[diesel(belongs_to(Order, foreign_key = sell_order_id))]
pub struct Trade {
    pub id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub symbol: String,
    pub price: bigdecimal::BigDecimal,
    pub quantity: bigdecimal::BigDecimal,
    pub executed_at: NaiveDateTime,
}

// Insertable for trades
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub symbol: String,
    pub price: bigdecimal::BigDecimal,
    pub quantity: bigdecimal::BigDecimal,
}
