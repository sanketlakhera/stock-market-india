mod constants;

use crate::models::bse::*;
use crate::models::common::StockError;
use crate::utils::HttpClient;
use constants::*;

pub struct BseService {
    client: HttpClient,
}

impl BseService {
    pub fn new() -> Self {
        Self {
            client: HttpClient::with_default_config(),
        }
    }

    /// Get all BSE indices
    pub async fn get_indices(&self) -> Result<Vec<BseIndex>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(INDICES_URL, headers).await
    }

    /// Get information about a specific index
    pub async fn get_index_info(&self, index_id: &str) -> Result<BseIndex, StockError> {
        let url = format!("{}{}", INDEX_INFO_URL, urlencoding::encode(index_id));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get stocks in a specific index
    pub async fn get_index_stocks(&self, index_id: &str) -> Result<Vec<BseQuote>, StockError> {
        let url = format!("{}{}", INDEX_STOCKS_URL, urlencoding::encode(index_id));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get detailed information about a stock
    pub async fn get_stock_info(&self, security_code: &str) -> Result<BseStockInfo, StockError> {
        let url = format!("{}{}", STOCK_INFO_URL, urlencoding::encode(security_code));
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get top gainers
    pub async fn get_gainers(&self) -> Result<Vec<BseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(GAINERS_URL, headers).await
    }

    /// Get top losers
    pub async fn get_losers(&self) -> Result<Vec<BseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(LOSERS_URL, headers).await
    }

    /// Get top turnover stocks
    pub async fn get_top_turnover(&self) -> Result<Vec<BseQuote>, StockError> {
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(TOP_TURNOVER_URL, headers).await
    }

    /// Get stock chart data
    pub async fn get_stock_chart_data(
        &self,
        security_code: &str,
        time_frame: &str,
    ) -> Result<Vec<BseChartData>, StockError> {
        let url = format!(
            "{}{}?timeframe={}",
            STOCK_CHART_URL,
            urlencoding::encode(security_code),
            time_frame
        );
        let headers = HttpClient::create_headers(&DEFAULT_HEADERS);
        self.client.get(&url, headers).await
    }

    /// Get stock info and day chart data
    pub async fn get_stock_info_and_day_chart(
        &self,
        security_code: &str,
    ) -> Result<(BseStockInfo, Vec<BseChartData>), StockError> {
        let stock_info = self.get_stock_info(security_code).await?;
        let chart_data = self.get_stock_chart_data(security_code, "1D").await?;
        Ok((stock_info, chart_data))
    }
}
