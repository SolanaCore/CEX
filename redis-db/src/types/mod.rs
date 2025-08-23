pub mod type;
pub use type::*;

use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell
}

// #[serde(rename_all = "lowercase")] -> this lets you specify how the enum should be serialized and deserialized. In this case, it will convert the enum variants to lowercase strings when serialized to JSON.
#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "lowercase")] 
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected
}