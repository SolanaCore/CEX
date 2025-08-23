use serde::{Serialize, Deserialize}
use validator::Validate;
use crate::utils::{SYMBOL_REGEX, MIN_CONST, MAX_CONST};
use crate::types::{OrderSide};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub enum MessageToSend{
    CreateOrder{
        payload: CreateOrderPayload
    }

    CloseOrder{
        payload: CloseOrderPayload
    }

    TradeExecuted {
        payload: TradeExecutedPayload
    }
    GetDeth {
        payload: GetDepthPayload
    }

    GetOpenOrders {
        payload: GetOpenOrdersPayload
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateOrderPayload {
        #[validate(length(min = "MIN_CONST", max = "MAX_CONST"))]
        user_id: u64,

        #[validate(regex(path = *SYMBOL_REGEX))]
        symbol: String,
        #[validate(nested)]
        side: OrderSide,
        
        executed_qty: String,
        price: String,
        quantity: String,
}


#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CloseOrderPayload {
            #[validate(length(min = "MIN_CONST"))]
            order_id: u64

}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TradeExecutedPayload {
        #[validate(length(min = "MIN_CONST"))]
        buy_order_id: u64,
            #[validate(length(min = "MIN_CONST"))]
        sell_order_id: u64,
                #[validate(regex(path = *SYMBOL_REGEX))]

        symbol: String,
        price: String,
        quantity: String
}

#[derive(Serialize, Deserialize, Debug , Validate)]
pub struct GetDepthPayload {
        #[validate(regex(path = *SYMBOL_REGEX))]
        symbol: String,

}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct GetOpenOrdersPayload {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
}