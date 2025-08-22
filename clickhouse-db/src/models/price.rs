use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Pool, Executor, Error};
use sqlx::any::AnyPool;



#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, FromRow, Validat)]
pub struct Price {
    #[validate(length(min = 1))]
    pub id: Uuid,
    
    pub symbol: String,
    pub price: f64,
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

    pub async fn create_table(pool: &AnyPool) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS prices (
                id UUID PRIMARY KEY,
                symbol String,
                price Float64,
                created_at DateTime64(3, 'UTC')
            )"
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn insert(&self, pool: &AnyPool) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO prices (id, symbol, price, created_at) VALUES ($1, $2, $3, $4)"
        )
        .bind(self.id)
        .bind(&self.symbol)
        .bind(self.price)
        .bind(self.created_at)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_by_symbol(pool: &AnyPool, symbol: &str) -> Result<Vec<Price>, Error> {
        let prices = sqlx::query_as::<_, Price>(
            "SELECT * FROM prices WHERE symbol = $1"
        )
        .bind(symbol)
        .fetch_all(pool)
        .await$;
        Ok(prices)
    }

    pub async fn get_klines(
        pool: &AnyPool,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        interval: KlineInterval
    ) -> Result<Vec<Kline>, Error> {
        let group_by = interval.to_sql();
        let query = format!(
            "SELECT \
                {group_by} as interval_time,\
                min(price) as low,\
                max(price) as high,\
                avg(price) as avg_price,\
                any(price) as open,\
                anyLast(price) as close\
            FROM prices\
            WHERE symbol = $1 AND created_at BETWEEN $2 AND $3\
            GROUP BY interval_time\
            ORDER BY interval_time ASC",
            group_by = group_by
        );
        let rows = sqlx::query(&query)
            .bind(symbol)
            .bind(start)
            .bind(end)
            .fetch_all(pool)
            .await?;
        let mut klines = Vec::new();
        for row in rows {
            let interval_time: DateTime<Utc> = row.try_get("interval_time")?;
            let open: f64 = row.try_get("open")?;
            let high: f64 = row.try_get("high")?;
            let low: f64 = row.try_get("low")?;
            let close: f64 = row.try_get("close")?;
            let avg_price: f64 = row.try_get("avg_price")?;
            klines.push(Kline {
                interval_time,
                open,
                high,
                low,
                close,
                avg_price,
            });
        }
        Ok(klines)
    }
}