use uuid::Uuid;
use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use clickhouse::Row;
use clickhouse_connection_pool::pool_manager::PoolManager;

use crate::utils::{
    SYMBOL_REGEX,
    interval_to_group_by,
};
use crate::types::{Kline, GetKlines};
use crate::InsertPayload;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate,Row)]
pub struct Price {
    pub id: Uuid,

    #[validate(regex(path = *SYMBOL_REGEX))]
    pub symbol: String,

    #[validate(range(min = 0.0))]
    pub price: f64,

    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub created_at: DateTime<Utc>,
}

impl Price {
    pub fn new(symbol: String, price: f64) -> Self {
        Price {
            id: Uuid::new_v4(),
            symbol,
            price,
            created_at: Utc::now(),
        }
    }

    /// Create table in ClickHouse
    pub async fn create_table_sql(pm: &PoolManager) -> Result<(), Box<dyn std::error::Error>> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS prices (
                id UUID,
                symbol String,
                price Float64,
                created_at DateTime
            ) ENGINE = MergeTree()
            ORDER BY (symbol, created_at)
        "#;

        pm.execute_with_retry(query).await?;
        println!("DB Price table init successful");
        Ok(())
    }

    /// Insert a single row into ClickHouse
    pub async fn insert_price(
        &self,
        pm: &PoolManager,
        payload: InsertPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // validate the payload
        payload.validate()?;

        let query = format!(
            "INSERT INTO prices (id, symbol, price, created_at) 
             VALUES ('{}', '{}', {}, '{}')",
            payload.id, payload.symbol, payload.price, payload.created_at
        );

        pm.execute_with_retry(&query).await?;
        Ok(())
    }

    /// Query prices by symbol
    pub async fn get_by_symbol(
        pm: &PoolManager,
        symbol: &str,
    ) -> Result<Vec<Price>, Box<dyn std::error::Error>> {

        let query = format!(
            "SELECT id, symbol, price, created_at 
             FROM prices 
             WHERE symbol = '{}'
             ORDER BY created_at ASC",
            symbol
        );

        let results: Vec<Price> = pm.execute_select_with_retry(&query).await?;
        Ok(results)
    }

    /// Get candlestick (kline) data
    pub async fn get_klines(
        pm: &PoolManager,
        mut payload: GetKlines, //marked `mut`
    ) -> Result<Vec<Kline>, Box<dyn std::error::Error>> {
        //validate the payload 
        payload.validate()?;
        payload.interval = interval_to_group_by(&payload.interval).to_string();

        let query = format!(
            r#"
            SELECT
                {group_by} AS interval_time,
                min(price)       AS low,
                max(price)       AS high,
                avg(price)       AS avg_price,
                any(price)       AS open,
                anyLast(price)   AS close
            FROM prices
            WHERE symbol = '{symbol}'
              AND created_at BETWEEN '{start}' AND '{end}'
            GROUP BY interval_time
            ORDER BY interval_time ASC
            "#,
            group_by = payload.interval, // e.g. toStartOfHour(created_at)
            symbol   = payload.symbol,
            start    = payload.start,
            end      = payload.end,
        );

        let klines: Vec<Kline> = pm.execute_select_with_retry(&query).await?;
        Ok(klines)
    }
}
