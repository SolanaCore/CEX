use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use once_cell::sync::Lazy;
use std::env;
//once_cell::Lazy provides a way to lazily initialize a value, meaning it's computed only when first accessed. This is particularly useful for scenarios where the initialization is expensive or might not even be needed during the program's execution. 
pub static REDIS_POOL: Lazy<r2d2::Pool<RedisConnectionManager>> = Lazy::new(|| {
    let url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let manager = RedisConnectionManager::new(url).expect("Invalid Redis URL");
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create Redis pool")
});