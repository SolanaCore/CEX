use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::utils::{SYMBOL_REGEX, MIN_CONST, MAX_CONST};
use crate::types::{OrderSide, OrderStatus};

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(tag = "type")]
pub enum MessageFromOrderbook {
    #[serde(rename = "DEPTH")]
    Depth {
        #[validate(nested)]
        payload: DepthPayload,
    },
    #[serde(rename = "ORDERPLACED")]
    OrderPlaced {
        #[validate(nested)]
        payload: OrderPlacedPayload,
    },
    #[serde(rename = "ORDERCANCELLED")]
    OrderCancelled {
        #[validate(nested)]
        payload: OrderCancelledPayload,
    },
    #[serde(rename = "OPENORDERS")]
    OpenOrders {
        #[validate(nested)]
        payload: Vec<OpenOrderPayload>,
    },
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct OrderPlacedPayload {
    #[validate(length(min = 1))]
    order_id: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct OrderCancelledPayload {
    #[validate(length(min = MIN_CONST))]
    order_id: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct OpenOrderPayload {
    #[validate(length(min = MIN_CONST))]
    pub order_id: String,

    pub executed_qty: f64,

    #[validate(length(min = MIN_CONST))]
    pub price: String,

    pub quantity: String,
    pub side: OrderSide,

    #[validate(length(min = MIN_CONST, max = MAX_CONST))]
    pub user_id: String,

    pub status: OrderStatus,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct DepthPayload {
    #[validate(regex(path = "SYMBOL_REGEX"))]
    pub symbol: String,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}
