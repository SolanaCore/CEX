use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageFromOrderbook {
    Depth {
        payload: DepthPayload,
    },
    OrderPlaced {
        payload: OrderPlacedPayload,
    },
    OrderCancelled {
        payload: OrderCancelledPayload,
    },
    OpenOrders {
        payload: Vec<OpenOrderPayload>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPlacedPayload {
    order_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCancelledPayload {
    order_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenOrderPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub price: String,
    pub quantity: String,
    pub side: OrderSide,
    pub user_id: String,
}

pub enum OrderSide {
    Buy,
    Sell
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthPayload {
    pub symbol: String,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}