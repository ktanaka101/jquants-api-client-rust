//! Indices (OHLC) API.

use serde::{Deserialize, Serialize};

use super::{
    shared::{
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::index_code::IndexCode,
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Indices (OHLC) API.
#[derive(Clone, Serialize)]
pub struct IndicesBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Index code (e.g., "0000" or "0028")
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<IndexCode>,
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

impl JQuantsBuilder<IndicesResponse> for IndicesBuilder {
    async fn send(self) -> Result<IndicesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<IndicesResponse, crate::JQuantsError> {
        self.client.inner.get("indices", self).await
    }
}

impl Paginatable<IndicesResponse> for IndicesBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl IndicesBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            code: None,
            from: None,
            to: None,
            date: None,
            pagination_key: None,
        }
    }

    /// Set index code (e.g., "0000" or "0028")
    pub fn code(mut self, code: impl Into<IndexCode>) -> Self {
        self.code = Some(code.into());
        self
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

/// Builder for Indices (OHLC) API.
pub trait IndicesApi: JQuantsPlanClient {
    /// Get API builder for Indices (OHLC).
    ///
    /// Use [Indices (OHLC) (/indices) API](https://jpx.gitbook.io/j-quants-en/api-reference/indices)
    fn get_indices(&self) -> IndicesBuilder {
        IndicesBuilder::new(self.get_api_client().clone())
    }
}

/// Indices (OHLC) response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/indices)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndicesResponse {
    /// List of indices data
    pub indices: Vec<IndexItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for IndicesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for IndicesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.indices.extend(p.indices);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single indices (OHLC) data item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndexItem {
    /// Trade date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Index code
    #[serde(rename = "Code")]
    pub code: IndexCode,

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
    fn test_deserialize_indices_response() {
        let json = r#"
            {
                "indices": [
                    {
                        "Date": "2023-12-01",
                        "Code": "0028",
                        "Open": 1199.18,
                        "High": 1202.58,
                        "Low": 1195.01,
                        "Close": 1200.17
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: IndicesResponse = serde_json::from_str(json).unwrap();
        let expected_response = IndicesResponse {
            indices: vec![IndexItem {
                date: "2023-12-01".to_string(),
                code: IndexCode::TOPIXCore30,
                open: 1199.18,
                high: 1202.58,
                low: 1195.01,
                close: 1200.17,
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_indices_response_no_pagination_key() {
        let json = r#"
            {
                "indices": [
                    {
                        "Date": "2023-12-01",
                        "Code": "0028",
                        "Open": 1199.18,
                        "High": 1202.58,
                        "Low": 1195.01,
                        "Close": 1200.17
                    }
                ]
            }
        "#;

        let response: IndicesResponse = serde_json::from_str(json).unwrap();
        let expected_response = IndicesResponse {
            indices: vec![IndexItem {
                date: "2023-12-01".to_string(),
                code: IndexCode::TOPIXCore30,
                open: 1199.18,
                high: 1202.58,
                low: 1195.01,
                close: 1200.17,
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_indices_response_multiple_items() {
        let json = r#"
            {
                "indices": [
                    {
                        "Date": "2023-11-30",
                        "Code": "0000",
                        "Open": 1500.50,
                        "High": 1520.75,
                        "Low": 1495.00,
                        "Close": 1510.25
                    },
                    {
                        "Date": "2023-12-01",
                        "Code": "0028",
                        "Open": 1199.18,
                        "High": 1202.58,
                        "Low": 1195.01,
                        "Close": 1200.17
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: IndicesResponse = serde_json::from_str(json).unwrap();
        let expected_response = IndicesResponse {
            indices: vec![
                IndexItem {
                    date: "2023-11-30".to_string(),
                    code: IndexCode::TOPIX,
                    open: 1500.50,
                    high: 1520.75,
                    low: 1495.00,
                    close: 1510.25,
                },
                IndexItem {
                    date: "2023-12-01".to_string(),
                    code: IndexCode::TOPIXCore30,
                    open: 1199.18,
                    high: 1202.58,
                    low: 1195.01,
                    close: 1200.17,
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_indices_response_no_data() {
        let json = r#"
            {
                "indices": []
            }
        "#;

        let response: IndicesResponse = serde_json::from_str(json).unwrap();
        let expected_response = IndicesResponse {
            indices: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
