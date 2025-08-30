pub mod historic_trades;
pub mod recent_trades;

pub use historic_trades::*;
pub use recent_trades::*;

pub fn trades_config(cfg: &mut actix_web::web::ServiceConfig) {
    historic_trades::config(cfg);
    recent_trades::config(cfg);
}