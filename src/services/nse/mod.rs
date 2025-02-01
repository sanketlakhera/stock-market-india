mod constants;

use crate::models::common::StockError;
use crate::models::nse::*;
use crate::utils::HttpClient;
use constants::*;

pub struct NseService {
    client: HttpClient,
}

impl NseService {
    pub fn new() -> Self {
        Self {
            client: HttpClient::with_default_config(),
        }
    }

    /// Get the market status (open/closed)
    pub async fn get_market_status(&self) -> Result<String, StockError> {
        #[derive(serde::Deserialize)]
        struct MarketStatusResponse {
            #[serde(rename = "NormalMktStatus")]
            status: String,
        }

        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        let response: MarketStatusResponse = self.client.get(MARKET_STATUS_URL, headers).await?;
        Ok(response.status)
    }

    /// Get all NSE indices
    pub async fn get_indices(&self) -> Result<Vec<NseIndex>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(INDICES_WATCH_URL, headers).await
    }

    /// Get top gainers
    pub async fn get_gainers(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(GAINERS_URL, headers).await
    }

    /// Get top losers
    pub async fn get_losers(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(LOSERS_URL, headers).await
    }

    /// Get advances/declines
    pub async fn get_advance_decline(&self) -> Result<NseAdvanceDecline, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(ADVANCES_DECLINES_URL, headers).await
    }

    /// Get quote information for a symbol
    pub async fn get_quote_info(&self, symbol: &str) -> Result<NseQuote, StockError> {
        let url = format!("{}{}", QUOTE_INFO_URL, urlencoding::encode(symbol));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get index stocks
    pub async fn get_index_stocks(&self, index: &str) -> Result<Vec<NseQuote>, StockError> {
        let url = format!("{}{}", INDEX_STOCKS_URL, urlencoding::encode(index));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Search stocks
    pub async fn search_stocks(&self, query: &str) -> Result<Vec<NseQuote>, StockError> {
        let url = format!("{}{}", SEARCH_URL, urlencoding::encode(query));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get 52 week high stocks
    pub async fn get_52_week_high(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(YEAR_HIGH_URL, headers).await
    }

    /// Get 52 week low stocks
    pub async fn get_52_week_low(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(YEAR_LOW_URL, headers).await
    }

    /// Get top value stocks
    pub async fn get_top_value_stocks(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(TOP_VALUE_URL, headers).await
    }

    /// Get top volume stocks
    pub async fn get_top_volume_stocks(&self) -> Result<Vec<NseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(TOP_VOLUME_URL, headers).await
    }
}
