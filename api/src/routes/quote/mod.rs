pub mod quote;
pub use quote::*;

pub fn quote_config(cfg: &mut actix_web::web::ServiceConfig) {
    quote::config(cfg);
}