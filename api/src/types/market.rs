use serde::Deserialize;
use validate::Validator;
use chrono::{DateTime, Utc};

use crate::{
    utils::{MIN_CONST, MAX_CONST, SYMBOL_REGEX, UTC_REGEX, is_valid_kline, default_end_time}
}


//KLINES
#[derive(Deserialize, Validate)]
pub(crate) struct getKlinesRequest {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    
    #[validate(custom(function = "is_valid_kline"))]
    klines_interval: String,

    start_time: DateTime<Utc>,

    #[serde(default = "default_end_time")]
    end_time: Option<DateTime<Utc>>,
}


#[derive(Deserialize)]
pub(crate) struct KlineResponse {
    symbol: String,
    klines: vec<Kline>,
}

#[derive(Deserialize)]
pub (crate) struct Kline {
    interval_time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    avg_price: f64,
}

//DEPTH

pub(crate) struct DepthRequest {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    #[validate(range(min = 1, max = 100))]
    
    limit: Option<u16>,  //User may or may not provide the limit, so we take 50 as default and 999 max..
}

pub(crate) struct DepthResponse {
    timestamp: DateTime<Utc>,
    last_update_id: u64,
    bids: Vec<OrderBookEntry>, // (price, quantity)
    asks: Vec<OrderBookEntry>, // (price, quantity)
}

pub(crate) struct OrderBookEntry {
    price: f64,
    quantity: f64,
}

//MARKET
#[derive(Deserialize, Validator)]
pub(crate) struct MarketRequest {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String
}


#[derive(Deserialize)]
pub(crate) struct MarketResponse {
    symbol: String,
    price: f64,
    volume: f64,
    timestamp: DateTime<Utc>,
}

pub(crate) struct AllMarketResponse {
    markets: Vec<MarketResponse>
}
