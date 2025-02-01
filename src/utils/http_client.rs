use crate::models::common::StockError;
use reqwest::{header::HeaderMap, Client};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub fn with_default_config() -> Self {
        Self::new(30)
    }

    /// Make a GET request with custom headers and deserialize the response
    pub async fn get<T>(&self, url: &str, headers: HeaderMap) -> Result<T, StockError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| StockError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(StockError::FetchError(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StockError::ParseError(e.to_string()))
    }

    /// Make a POST request with custom headers and body, and deserialize the response
    pub async fn post<T, B>(&self, url: &str, headers: HeaderMap, body: &B) -> Result<T, StockError>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| StockError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(StockError::FetchError(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| StockError::ParseError(e.to_string()))
    }

    /// Convert a slice of header tuples into a HeaderMap
    pub fn create_headers(headers: &[(&str, &str)]) -> HeaderMap {
        headers.iter().fold(HeaderMap::new(), |mut map, &(k, v)| {
            map.insert(
                k.parse().expect("Invalid header name"),
                v.parse().expect("Invalid header value"),
            );
            map
        })
    }
}
