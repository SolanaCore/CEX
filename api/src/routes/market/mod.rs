pub mod depth;
pub mod klines;
pub mod market;

pub use depth::*;
pub use klines::*;
pub use market::*;

pub fn market_config(cfg: &mut actix_web::web::ServiceConfig) {
    depth::config(cfg);
    klines::config(cfg);
    market::config(cfg);
}