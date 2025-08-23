use r2d2_redis::RedisConnectionManager;
use once_cell::sync::Lazy;
use std::env;
use dotenv::dotenv;

pub type RedisPool = Lazy<r2d2::Pool<RedisConnectionManager>>;

pub static REDIS_POOL: RedisPool = Lazy::new(|| {
    dotenv().ok(); // Load .env variables
    let url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let manager = RedisConnectionManager::new(url).expect("Invalid Redis URL");

    r2d2::Pool::builder()
        .max_size(50)
        .build(manager)
        .expect("Failed to create Redis pool")
});
