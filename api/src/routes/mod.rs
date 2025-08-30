pub mod market;
pub mod quote;
pub mod trades;
pub mod user;

pub use market::*;
pub use quote::*;
pub use trades::*;
pub use user::*;

pub fn routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    market::market_config(cfg);
    quote::quote_config(cfg);
    trades::trades_config(cfg);
    user::user_config(cfg);
}