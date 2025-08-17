use std::env;
use once_cell::sync::Lazy;

pub mod connection;


#[cfg(test)]
mod tests {
    use super::connection::get_redis_connection;
    use redis::{Commands};

    #[tokio::test]
    async fn test_redis_connection_pool() {
    let mut con = get_redis_connection();

        Commands("SET")
            .arg(&["deadpool/test_key", "42"])
            .query_async::<()>(&mut con)
            .await.unwrap();
    
    }
}
