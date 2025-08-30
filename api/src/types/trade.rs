use validate::Validator;
use serde::Deserialize;
use crate::utils::{MIN_CONST, MAX_CONST, UTC_REGEX, SYMBOL_REGEX, default_end_time};

#[derive(Serialize, Validator)]
pub(crate) struct GetRecentTradeRequest {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    #[validate(range(min = 1, max = 1000))]
    limit: Option<u16>,  //User may or may not provide the limit, so we take 50 as default and 999 max..
};

#[derive(Serialize, Validator)]
pub(crate) struct GetHistoricTradeRequest {
    #[validate(regex(path = *SYMBOL_REGEX))]
    symbol: String,
    #[validate(range(min = 1, max = 1000))]
    limit: Option<u16>,  //User may or may not provide the limit, so we take 50 as default and 999 max..
    #[validate(regex(path = *UTC_REGEX))]
    start_time: String, 
    #[validate(regex(path = *UTC_REGEX))]
    #[serde(default = "default_end_time")]
    end_time: Option<String>,
}

/*
base_qty: e.g. sol
quote_qty: e.g. usdc
is_buyer_maker: if true, buyer is the maker. If false, seller is the taker.

//maker - > who places a limit order that adds liquidity to the order book.
//taker -> who places a market order that matches immediately against an existing order in the order book
*/
#[derive(Deserialize)]
pub(crate) struct RecentTradeResponse {
    id: u64,
    price: f64,
    qty: f64,
    quote_qty: f64,
    timestamp: DateTime<Utc>,
    is_buyer_maker: bool,
}

#[derive(Deserialize)]
pub(crate) struct HistoricTradeResponse {
    id: u64,
    price: f64,
    qty: f64,
    quote_qty: f64,
    timestamp: DateTime<Utc>,
    is_buyer_maker: bool,
}
