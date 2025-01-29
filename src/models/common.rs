use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketStatus {
    pub status: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockQuote {
    pub symbol: String,
    pub company_name: String,
    pub last_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: i64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub last: f64,
    pub change: f64,
    pub change_percent: f64,
    pub high: f64,
    pub low: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopGainer {
    pub symbol: String,
    pub last_price: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopLoser {
    pub symbol: String,
    pub last_price: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum StockError {
    #[error("Failed to fetch data: {0}")]
    FetchError(String),

    #[error("Failed to parse data: {0}")]
    ParseError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Service error: {0}")]
    ServiceError(String),
}
