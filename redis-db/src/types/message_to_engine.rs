use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::utils::{SYMBOL_REGEX, MIN_CONST, MAX_CONST};
use crate::types::{OrderSide};

/// Outgoing messages (client → orderbook)
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageToSend {
    #[serde(rename = "PLACE_ORDER")]
    PlaceOrder {
        payload: PlaceOrderPayload,
        client_id: String,
    },
    #[serde(rename = "CANCEL_ORDER")]
    CancelOrder {
        payload: CancelOrderPayload,
        client_id: String,
    },
    #[serde(rename = "GET_DEPTH")]
    GetDepth {
        payload: DepthRequestPayload,
        client_id: String,
    },
    #[serde(rename = "GET_OPENORDERS")]
    GetOpenOrders {
        payload: OpenOrdersRequestPayload,
        client_id: String,
    },
}

//
// -------- PAYLOADS --------
//

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PlaceOrderPayload {
    #[validate(regex(path = *SYMBOL_REGEX))]
    pub symbol: String,

    pub side: OrderSide, // BUY or SELL

    #[validate(length(min = MIN_CONST))]
    pub price: String,

    #[validate(length(min = MIN_CONST))]
    pub quantity: String,

    #[validate(length(min = MIN_CONST, max = MAX_CONST))]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CancelOrderPayload {
    #[validate(length(min = MIN_CONST))]
    pub order_id: String,

    #[validate(length(min = MIN_CONST, max = MAX_CONST))]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct DepthRequestPayload {
    #[validate(regex(path = *SYMBOL_REGEX))]
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct OpenOrdersRequestPayload {
    #[validate(length(min = MIN_CONST, max = MAX_CONST))]
    pub user_id: String,
}
