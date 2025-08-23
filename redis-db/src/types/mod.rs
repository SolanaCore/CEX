use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell
}

// #[serde(rename_all = "lowercase")] -> this lets you specify how the enum should be serialized and deserialized. In this case, it will convert the enum variants to lowercase strings when serialized to JSON.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")] 
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected
}

pub mod message_to_engine;
pub mod message_from_engine;

pub use message_to_engine::*;
pub use message_from_engine::*;