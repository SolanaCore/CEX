use once_cell::sync::Lazy;
use clickhouse_connection_pool::{
    pool_manager::PoolManager,
    config::{ClickhouseConfig, DatalakeConfig, RetryConfig},
    traits::Model,
};
use anyhow::Result;
use dotenv::dotenv;
use std::{error::Error, sync::Arc};
use tokio::runtime::Runtime;
use crate::error::ClickhouseError;

pub static CONNECTION_POOL: Lazy<Arc<PoolManager>> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    let config = load_config().unwrap();
    rt.block_on(create_pool_manager(config)).unwrap()
});


fn load_config() -> Result<Arc<DatalakeConfig>, Box<dyn Error>> {
    dotenv().ok(); // Load from .env

    let host = std::env::var("CLICKHOUSE_HOST")
        .unwrap_or_else(|_| "localhost".to_string());

    let port = std::env::var("CLICKHOUSE_PORT")
        .map(|p| p.parse::<u16>().unwrap_or(9000))
        .unwrap_or(9000);

    let database = std::env::var("CLICKHOUSE_DB")
        .unwrap_or_else(|_| "test".to_string());

    let username = std::env::var("CLICKHOUSE_USER")
        .unwrap_or_else(|_| "default".to_string());

    let password = std::env::var("CLICKHOUSE_PASSWORD")
        .unwrap_or_default();

    let connect_timeout_seconds = std::env::var("CLICKHOUSE_CONNECT_TIMEOUT_SECONDS")
        .map(|s| s.parse::<u64>().unwrap_or(10))
        .unwrap_or(10);

    let query_timeout_seconds = std::env::var("CLICKHOUSE_QUERY_TIMEOUT_SECONDS")
        .map(|s| s.parse::<u64>().unwrap_or(60))
        .unwrap_or(60);

    let max_connections = std::env::var("CLICKHOUSE_MAX_CONNECTIONS")
        .map(|s| s.parse::<u32>().unwrap_or(10))
        .unwrap_or(10);

    let config = ClickhouseConfig::new(
        host,
        port,
        database,
        username,
        password,
        connect_timeout_seconds,
        query_timeout_seconds,
        max_connections,
    );

    let retry_config = RetryConfig {
        max_retries: 3,
        initial_backoff_ms: 100,
        max_backoff_ms: 500,
        backoff_multiplier: 1.5,
    };

    Ok(Arc::new(DatalakeConfig::new(config, retry_config)))
}

async fn create_pool_manager(
    config: Arc<DatalakeConfig>,
) -> Result<Arc<PoolManager>, ClickhouseError> {
    let manager = PoolManager::new(config, None).await;

    // if let Err(e) = manager.execute_with_retry(Price::create_table_sql()).await {
    //     return Err(ClickhouseError::Query {
    //         query: Price::create_table_sql().to_string(),
    //         reason: e.to_string(),
    //     });
    // }

    println!("DB init successful");
    Ok(Arc::new(manager))
}
