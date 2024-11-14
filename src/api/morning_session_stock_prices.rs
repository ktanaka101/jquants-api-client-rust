//! Morning Session Stock Prices API.
use std::{fmt, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    builder::JQuantsBuilder,
    pagination::{HasPaginationKey, MergePage, Paginatable},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Morning Session Stock Prices API.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
#[derive(Clone, Serialize)]
pub struct MorningSessionStockPricesApiBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R>
    for MorningSessionStockPricesApiBuilder<R>
{
    /// Get prices daily quotes.
    async fn send(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("/prices/prices_am", self).await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for MorningSessionStockPricesApiBuilder<R>
{
    fn pagination_key(&mut self, pagination_key: impl Into<String>) -> &mut Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> MorningSessionStockPricesApiBuilder<R> {
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            pagination_key: None,
        }
    }

    /// Issue code (e.g. 27800 or 2780)
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        self.code = Some(code.into());
        self
    }
}

/// Morning Session Stock Prices API.
pub trait MorningSessionStockPricesApi: JQuantsPlanClient {
    /// Response type for morning session stock prices API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get morning session stock prices.
    fn morning_session_stock_prices(&self) -> MorningSessionStockPricesApiBuilder<Self::Response> {
        MorningSessionStockPricesApiBuilder::new(self.get_api_client().clone())
    }
}

/// Morning Session Stock Prices response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
pub type MorningSessionStockPricesFreePlanResponse = MorningSessionStockPricesPremiumPlanResponse;

/// Morning Session Stock Prices response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
pub type MorningSessionStockPricesLightPlanResponse = MorningSessionStockPricesPremiumPlanResponse;

/// Morning Session Stock Prices response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
pub type MorningSessionStockPricesStandardPlanResponse =
    MorningSessionStockPricesPremiumPlanResponse;

/// Morning Session Stock Prices response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/prices_am)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MorningSessionStockPricesPremiumPlanResponse {
    /// List of morning session stock prices.
    prices_am: Vec<MorningAmCommon>,
    /// Pagination key for fetching next set of data.
    pagination_key: Option<String>,
}
impl HasPaginationKey for MorningSessionStockPricesPremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for MorningSessionStockPricesPremiumPlanResponse {
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

/// Morning session stock prices for premium plan.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MorningAmPremiumPlan {
    /// Common fields
    pub common: MorningAmCommon,
}

/// Morning session stock prices.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MorningAmCommon {
    /// Date
    #[serde(rename = "Date")]
    pub date: String,
    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,
    /// Open price of the morning session
    #[serde(rename = "MorningOpen")]
    pub morning_open: f64,
    /// High price of the morning session
    #[serde(rename = "MorningHigh")]
    pub morning_high: f64,
    /// Low price of the morning session
    #[serde(rename = "MorningLow")]
    pub morning_low: f64,
    /// Close price of the morning session
    #[serde(rename = "MorningClose")]
    pub morning_close: f64,
    /// Trading volume of the morning session
    #[serde(rename = "MorningVolume")]
    pub morning_volume: f64,
    /// Trading value of the morning session
    #[serde(rename = "MorningTurnoverValue")]
    pub morning_turnover_value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_morning_session_stock_prices_premium_plan_response() {
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
        let response: MorningSessionStockPricesPremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response: MorningSessionStockPricesPremiumPlanResponse =
            MorningSessionStockPricesPremiumPlanResponse {
                prices_am: vec![MorningAmCommon {
                    date: "2023-03-20".to_string(),
                    code: "39400".to_string(),
                    morning_open: 232.0,
                    morning_high: 244.0,
                    morning_low: 232.0,
                    morning_close: 240.0,
                    morning_volume: 52600.0,
                    morning_turnover_value: 12518800.0,
                }],
                pagination_key: Some("value1.value2.".to_string()),
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
