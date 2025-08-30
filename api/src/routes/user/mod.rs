pub mod balance;
pub mod login;
pub mod register;
pub mod logout;
pub mod user;

pub use balance::*;
pub use login::*;
pub use register::*;
pub use logout::*;
pub use user::*;

pub fn user_config(cfg: &mut actix_web::web::ServiceConfig) {
    balance::config(cfg);
    login::config(cfg);
    register::config(cfg);
    logout::config(cfg);
    user::config(cfg);
}