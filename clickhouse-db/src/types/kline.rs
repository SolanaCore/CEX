#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Kline {
    pub interval_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub avg_price: f64,
}
