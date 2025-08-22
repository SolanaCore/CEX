use once_cell::Lazy;
use clickhouse_connection_pool::{
    pool_manager::PoolManager,
    config::{DatalakeConfig, ClickhouseConfig, RetryConfig},
    traits::Model
};
use anyhow::Result;
use dotenv::dotenv;
use std::sync::Arc;
use std::error::Error;

const ConnectionPool = Lazy::new(|| {
    // Initialize your connection pool here
    // For example, using a hypothetical `create_connection_pool` function
        let datalake_config = load_config().unwrap();
            let pool_manager = create_manager_and_instantiate_table(datalake_config).await.unwrap();

});

fn load_config() -> Result<Arc<DatalakeConfig>, Box<dyn Error>> {
    let host = std::env::var("CLICKHOUSE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("CLICKHOUSE_PORT")
        .map(|p| p.parse::<u16>().unwrap_or(9000))
        .unwrap_or(9000);
    let database = std::env::var("CLICKHOUSE_DB").unwrap_or_else(|_| "test".to_string());
    let username = std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "default".to_string());
    let password = std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_default();
    
    let config = ClickhouseConfig::new(host, port, database, username, password, 10, 30, 5);
        
    let retry_config = RetryConfig::default();

    let datalake_config = Arc::new(DatalakeConfig::new(config, retry_config));
    
    Ok(datalake_config)
}

async fn create_pool_manager(
    config: Arc<DatalakeConfig>,
) -> Result<Arc<PoolManager>, Box<dyn Error>> {
    let manager = PoolManager::new(
        config,
        None,
    ).await;
    
    let _ = match manager
        .execute_with_retry(Price::create_table_sql())
        .await {
            Ok(_) => {
                println!("DB init successful");
                Ok(())
            },
            Err(e) => {
                println!("Error initing db: {:?}", e);
                Err(e)
            }
        };

    Ok(Arc::new(manager))
}
