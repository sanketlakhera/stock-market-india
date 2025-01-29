use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NseIndex {
    pub name: String,
    pub last: f64,
    pub change: f64,
    pub change_percent: f64,
    pub year_high: f64,
    pub year_low: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NseQuote {
    pub symbol: String,
    pub company_name: String,
    pub last_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub market_cap: f64,
    pub volume: i64,
    pub trading_value: f64,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NseAdvanceDecline {
    pub advances: i32,
    pub declines: i32,
    pub unchanged: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NseIntraDay {
    pub timestamp: String,
    pub price: f64,
    pub volume: i64,
}

// Constants
pub const NSE_INDICES: &[(&str, &str)] = &[
    ("NIFTY 50", "nifty"),
    ("NIFTY BANK", "bankNifty"),
    ("NIFTY IT", "cnxit"),
    // Add more indices as needed
];

// Type aliases for common data structures
pub type NseIndexMap = HashMap<String, String>;
pub type NseStockList = Vec<String>;
