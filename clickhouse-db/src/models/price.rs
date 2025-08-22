use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Error};
use validator::Validate;
use chrono::Utc;

use crate::utils::{SYMBOL_REGEX, UTC_REGEX};
use crate::types::{Kline, GetKlines};
use crate::connection::CONNECTION_POOL;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, FromRow, Validate)]
pub struct Price {
    pub id: Uuid,

    #[validate(regex(path = *SYMBOL_REGEX))]
    pub symbol: String,

    #[validate(range(min = 0.0))]
    pub price: f64,

    #[validate(regex(path = *UTC_REGEX))]
    pub created_at: String,
}

impl Price {
    pub fn new(symbol: String, price: f64) -> Self {
        Price {
            id: Uuid::new_v4(),
            symbol,
            price,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub async fn create_table() -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prices (
                id UUID PRIMARY KEY,
                symbol String,
                price Float64,
                created_at String DEFAULT toString(now())
            )
            "#
        )
        .execute(&*CONNECTION_POOL)
        .await?;

        Ok(())
    }

    pub async fn insert(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO prices (id, symbol, price, created_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(self.id)
        .bind(&self.symbol)
        .bind(self.price)
        .bind(&self.created_at)
        .execute(&*CONNECTION_POOL)
        .await?;

        Ok(())
    }

    pub async fn get_by_symbol(symbol: &str) -> Result<Vec<Price>, Error> {
        let prices = sqlx::query_as::<_, Price>(
            "SELECT * FROM prices WHERE symbol = $1"
        )
        .bind(symbol)
        .fetch_all(&*CONNECTION_POOL)
        .await?;

        Ok(prices)
    }

    pub async fn get_klines(payload: GetKlines) -> Result<Vec<Kline>, Error> {
        let group_by = payload.interval;
        let query = format!(
            r#"
            SELECT
                {group_by} as interval_time,
                min(price)   as low,
                max(price)   as high,
                avg(price)   as avg_price,
                any(price)   as open,
                anyLast(price) as close
            FROM prices
            WHERE symbol = $1
              AND created_at BETWEEN $2 AND $3
            GROUP BY interval_time
            ORDER BY interval_time ASC
            "#,
            group_by = group_by
        );

        let rows = sqlx::query_as::<_, Kline>(&query)
            .bind(payload.symbol)
            .bind(payload.start)
            .bind(payload.end)
            .fetch_all(&*CONNECTION_POOL)
            .await?;

        Ok(rows)
    }
}
