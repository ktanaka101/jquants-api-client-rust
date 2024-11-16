//! Morning Session Stock Prices API.

use serde::{Deserialize, Serialize};

use super::{
    shared::traits::{
        builder::JQuantsBuilder,
        pagination::{HasPaginationKey, MergePage, Paginatable},
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Morning Session Stock Prices API.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
#[derive(Clone, Serialize)]
pub struct MorningSessionStockPricesApiBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Issue code (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<MorningSessionStockPricesResponse> for MorningSessionStockPricesApiBuilder {
    async fn send(self) -> Result<MorningSessionStockPricesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<MorningSessionStockPricesResponse, crate::JQuantsError> {
        self.client.inner.get("/prices/prices_am", &self).await
    }
}

impl Paginatable<MorningSessionStockPricesResponse> for MorningSessionStockPricesApiBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl MorningSessionStockPricesApiBuilder {
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            code: None,
            pagination_key: None,
        }
    }

    /// Issue code (e.g. 27800 or 2780)
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// Morning Session Stock Prices API.
pub trait MorningSessionStockPricesApi: JQuantsPlanClient {
    /// Get api builder for morning session stock prices.
    fn morning_session_stock_prices(&self) -> MorningSessionStockPricesApiBuilder {
        MorningSessionStockPricesApiBuilder::new(self.get_api_client().clone())
    }
}

/// Morning Session Stock Prices response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MorningSessionStockPricesResponse {
    /// List of morning session stock prices.
    prices_am: Vec<MorningStockPriceItem>,
    /// Pagination key for fetching next set of data.
    pagination_key: Option<String>,
}
impl HasPaginationKey for MorningSessionStockPricesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for MorningSessionStockPricesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.prices_am.extend(p.prices_am);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Morning session stock price.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MorningStockPriceItem {
    /// Date
    #[serde(rename = "Date")]
    pub date: String,
    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,
    /// Open price of the morning session
    #[serde(rename = "MorningOpen")]
    pub morning_open: Option<f64>,
    /// High price of the morning session
    #[serde(rename = "MorningHigh")]
    pub morning_high: Option<f64>,
    /// Low price of the morning session
    #[serde(rename = "MorningLow")]
    pub morning_low: Option<f64>,
    /// Close price of the morning session
    #[serde(rename = "MorningClose")]
    pub morning_close: Option<f64>,
    /// Trading volume of the morning session
    #[serde(rename = "MorningVolume")]
    pub morning_volume: Option<f64>,
    /// Trading value of the morning session
    #[serde(rename = "MorningTurnoverValue")]
    pub morning_turnover_value: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_morning_session_stock_prices_response() {
        let json = r#"
            {
                "prices_am": [
                    {
                        "Date": "2023-03-20",
                        "Code": "39400",
                        "MorningOpen": 232.0,
                        "MorningHigh": 244.0,
                        "MorningLow": 232.0,
                        "MorningClose": 240.0,
                        "MorningVolume": 52600.0,
                        "MorningTurnoverValue": 12518800.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;
        let response: MorningSessionStockPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response: MorningSessionStockPricesResponse =
            MorningSessionStockPricesResponse {
                prices_am: vec![MorningStockPriceItem {
                    date: "2023-03-20".to_string(),
                    code: "39400".to_string(),
                    morning_open: Some(232.0),
                    morning_high: Some(244.0),
                    morning_low: Some(232.0),
                    morning_close: Some(240.0),
                    morning_volume: Some(52600.0),
                    morning_turnover_value: Some(12518800.0),
                }],
                pagination_key: Some("value1.value2.".to_string()),
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_morning_session_stock_prices_response_no_data() {
        let json = r#"
            {
                "prices_am": [
                    {
                        "Date": "2023-03-20",
                        "Code": "39400",
                        "MorningOpen": null,
                        "MorningHigh": null,
                        "MorningLow": null,
                        "MorningClose": null,
                        "MorningVolume": null,
                        "MorningTurnoverValue": null
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;
        let response: MorningSessionStockPricesResponse = serde_json::from_str(json).unwrap();
        let expected_response: MorningSessionStockPricesResponse =
            MorningSessionStockPricesResponse {
                prices_am: vec![MorningStockPriceItem {
                    date: "2023-03-20".to_string(),
                    code: "39400".to_string(),
                    morning_open: None,
                    morning_high: None,
                    morning_low: None,
                    morning_close: None,
                    morning_volume: None,
                    morning_turnover_value: None,
                }],
                pagination_key: Some("value1.value2.".to_string()),
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
