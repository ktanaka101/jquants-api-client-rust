//! TOPIX Prices (OHLC) API.

use serde::{Deserialize, Serialize};

use super::{
    shared::traits::{
        builder::JQuantsBuilder,
        pagination::{HasPaginationKey, MergePage, Paginatable},
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for TOPIX Prices (OHLC) API.
#[derive(Clone, Serialize)]
pub struct TopixPricesBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Starting point of data period (e.g., "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g., "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// Date of data (e.g., "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<TopixPricesResponse> for TopixPricesBuilder {
    async fn send(self) -> Result<TopixPricesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<TopixPricesResponse, crate::JQuantsError> {
        self.client.inner.get("indices/topix", self).await
    }
}

impl Paginatable<TopixPricesResponse> for TopixPricesBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl TopixPricesBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            from: None,
            to: None,
            date: None,
            pagination_key: None,
        }
    }

    /// Set starting point of data period (e.g., "20210901" or "2021-09-01")
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set end point of data period (e.g., "20210907" or "2021-09-07")
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set date of data (e.g., "20210907" or "2021-09-07")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Builder for TOPIX Prices (OHLC) API.
pub trait TopixPricesApi: JQuantsPlanClient {
    /// Get API builder for TOPIX Prices (OHLC).
    ///
    /// Use [TOPIX Prices (OHLC) (/indices/topix) API](https://jpx.gitbook.io/j-quants-en/api-reference/topix)
    fn get_topix_prices(&self) -> TopixPricesBuilder {
        TopixPricesBuilder::new(self.get_api_client().clone())
    }
}

/// TOPIX Prices (OHLC) response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/topix)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TopixPricesResponse {
    /// List of TOPIX prices data
    pub topix: Vec<TopixPriceItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for TopixPricesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for TopixPricesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.topix.extend(p.topix);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single TOPIX price (OHLC) data item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TopixPriceItem {
    /// Trade date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Open Price
    #[serde(rename = "Open")]
    pub open: f64,

    /// High Price
    #[serde(rename = "High")]
    pub high: f64,

    /// Low Price
    #[serde(rename = "Low")]
    pub low: f64,

    /// Close Price
    #[serde(rename = "Close")]
    pub close: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_topix_prices_response() {
        let json = r#"
            {
                "topix": [
                    {
                        "Date": "2022-06-28",
                        "Open": 1885.52,
                        "High": 1907.38,
                        "Low": 1885.32,
                        "Close": 1907.38
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: TopixPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response = TopixPricesResponse {
            topix: vec![TopixPriceItem {
                date: "2022-06-28".to_string(),
                open: 1885.52,
                high: 1907.38,
                low: 1885.32,
                close: 1907.38,
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_topix_prices_response_no_pagination_key() {
        let json = r#"
            {
                "topix": [
                    {
                        "Date": "2022-06-28",
                        "Open": 1885.52,
                        "High": 1907.38,
                        "Low": 1885.32,
                        "Close": 1907.38
                    }
                ]
            }
        "#;

        let response: TopixPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response = TopixPricesResponse {
            topix: vec![TopixPriceItem {
                date: "2022-06-28".to_string(),
                open: 1885.52,
                high: 1907.38,
                low: 1885.32,
                close: 1907.38,
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_topix_prices_esponse_multiple_items() {
        let json = r#"
            {
                "topix": [
                    {
                        "Date": "2022-06-27",
                        "Open": 1850.50,
                        "High": 1875.75,
                        "Low": 1845.00,
                        "Close": 1860.25
                    },
                    {
                        "Date": "2022-06-28",
                        "Open": 1885.52,
                        "High": 1907.38,
                        "Low": 1885.32,
                        "Close": 1907.38
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: TopixPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response = TopixPricesResponse {
            topix: vec![
                TopixPriceItem {
                    date: "2022-06-27".to_string(),
                    open: 1850.50,
                    high: 1875.75,
                    low: 1845.00,
                    close: 1860.25,
                },
                TopixPriceItem {
                    date: "2022-06-28".to_string(),
                    open: 1885.52,
                    high: 1907.38,
                    low: 1885.32,
                    close: 1907.38,
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_topix_prices_response_no_data() {
        let json = r#"
            {
                "topix": []
            }
        "#;

        let response: TopixPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response = TopixPricesResponse {
            topix: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
