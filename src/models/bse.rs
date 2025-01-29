use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BseIndex {
    pub index_id: String,
    pub name: String,
    pub last: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BseQuote {
    pub security_code: String,
    pub company_name: String,
    pub group: String,
    pub face_value: f64,
    pub last_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: i64,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BseStockInfo {
    pub security_code: String,
    pub company_name: String,
    pub market_cap: f64,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
    pub today_open: f64,
    pub today_high: f64,
    pub today_low: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BseChartData {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

// Constants
pub const BSE_INDICES: &[(&str, &str)] = &[
    ("SENSEX", "16"),
    ("BSE100", "22"),
    ("BSE200", "23"),
    // Add more indices as needed
];

// Type aliases for common data structures
pub type BseIndexMap = HashMap<String, String>;
pub type BseSecurityCodeMap = HashMap<String, String>;
