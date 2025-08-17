use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderPayload {
 user_id: u64,
        symbol: String,
        side: String,
        price: String,
        quantity: String,
        status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseOrderPayload {
            order_id: u64

}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeExecutedPayload {
    buy_order_id: u64,
        sell_order_id: u64,
        symbol: String,
        price: String,
        quantity: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDepthPayload {
        symbol: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenOrdersPayload {
            symbol: String,

}