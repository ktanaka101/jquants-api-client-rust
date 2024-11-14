//! Prices daily quotes API.
use std::{fmt, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    builder::JQuantsBuilder,
    pagination::{HasPaginationKey, MergePage, Paginatable},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Stock Prices (OHLC) API.
#[derive(Clone, Serialize)]
pub struct StockPricesBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// Starting point of data period (e.g. 20210901 or 2021-09-01)
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// Date of data (e.g. 20210907 or 2021-09-07)
    /// Used when `from` and `to` are not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R> for StockPricesBuilder<R> {
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("prices/daily_quotes", self).await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for StockPricesBuilder<R>
{
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> StockPricesBuilder<R> {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            from: None,
            to: None,
            date: None,
            pagination_key: None,
        }
    }

    /// Set issue code (e.g. 27800 or 2780)
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set starting point of data period (e.g. 20210901 or 2021-09-01)
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set end point of data period (e.g. 20210907 or 2021-09-07)
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set date of data (e.g. 20210907 or 2021-09-07)
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Builder for Stock Prices (OHLC) API.
pub trait StockPricesApi: JQuantsPlanClient {
    /// Response type for listed info API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get api builder for stock prices.
    ///
    /// Use [Stock Prices (OHLC)(/prices/daily_quotes) API](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
    fn get_stock_prices(&self) -> StockPricesBuilder<Self::Response> {
        StockPricesBuilder::new(self.get_api_client().clone())
    }
}

/// Stock prices (OHLC) response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
pub type StockPricesFreePlanResponse = StockPricesStandardPlanResponse;

/// Stock prices (OHLC) response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
pub type StockPricesLightPlanResponse = StockPricesStandardPlanResponse;

/// Stock prices (OHLC) response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StockPricesStandardPlanResponse {
    /// List of daily quotes
    pub daily_quotes: Vec<DailyQuoteStandardPlan>,

    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}
impl HasPaginationKey for StockPricesStandardPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for StockPricesStandardPlanResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.daily_quotes.extend(p.daily_quotes);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Stock prices (OHLC) response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StockPricesPremiumPlanResponse {
    /// List of daily quotes
    pub daily_quotes: Vec<DailyQuotePremiumPlan>,

    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}
impl HasPaginationKey for StockPricesPremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for StockPricesPremiumPlanResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.daily_quotes.extend(p.daily_quotes);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Daily Quote for standard plan.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyQuoteStandardPlan {
    /// The common structure for daily quote
    #[serde(flatten)]
    pub common: DailyQuoteCommon,
}

/// Daily Quote for standard plan.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyQuotePremiumPlan {
    /// The common structure for daily quote
    #[serde(flatten)]
    pub common: DailyQuoteCommon,

    /// Open price of the morning session (before Adjustment)
    #[serde(rename = "MorningOpen")]
    pub morning_open: f64,

    /// High price of the morning session (before Adjustment)
    #[serde(rename = "MorningHigh")]
    pub morning_high: f64,

    /// Low price of the morning session (before Adjustment)
    #[serde(rename = "MorningLow")]
    pub morning_low: f64,

    /// Close price of the morning session (before Adjustment)
    #[serde(rename = "MorningClose")]
    pub morning_close: f64,

    /// Flag of hitting the upper price limit of the day in morning session
    #[serde(rename = "MorningUpperLimit")]
    pub morning_upper_limit: String,

    /// Flag of hitting the lower price limit of the day in morning session
    #[serde(rename = "MorningLowerLimit")]
    pub morning_lower_limit: String,

    /// Trading volume of the morning session (before Adjustment)
    #[serde(rename = "MorningVolume")]
    pub morning_volume: f64,

    /// Trading value of the morning session
    #[serde(rename = "MorningTurnoverValue")]
    pub morning_turnover_value: f64,

    /// Adjusted open price of the morning session
    #[serde(rename = "MorningAdjustmentOpen")]
    pub morning_adjustment_open: f64,

    /// Adjusted high price of the morning session
    #[serde(rename = "MorningAdjustmentHigh")]
    pub morning_adjustment_high: f64,

    /// Adjusted low price of the morning session
    #[serde(rename = "MorningAdjustmentLow")]
    pub morning_adjustment_low: f64,

    /// Adjusted close price of the morning session
    #[serde(rename = "MorningAdjustmentClose")]
    pub morning_adjustment_close: f64,

    /// Adjusted trading volume of the morning session
    #[serde(rename = "MorningAdjustmentVolume")]
    pub morning_adjustment_volume: f64,

    /// Open price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonOpen")]
    pub afternoon_open: f64,

    /// High price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonHigh")]
    pub afternoon_high: f64,

    /// Low price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonLow")]
    pub afternoon_low: f64,

    /// Close price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonClose")]
    pub afternoon_close: f64,

    /// Flag of hitting the upper price limit of the day in afternoon session
    #[serde(rename = "AfternoonUpperLimit")]
    pub afternoon_upper_limit: String,

    /// Flag of hitting the lower price limit of the day in afternoon session
    #[serde(rename = "AfternoonLowerLimit")]
    pub afternoon_lower_limit: String,

    /// Trading volume of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonVolume")]
    pub afternoon_volume: f64,

    /// Trading value of the afternoon session
    #[serde(rename = "AfternoonTurnoverValue")]
    pub afternoon_turnover_value: f64,

    /// Adjusted open price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentOpen")]
    pub afternoon_adjustment_open: f64,

    /// Adjusted high price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentHigh")]
    pub afternoon_adjustment_high: f64,

    /// Adjusted low price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentLow")]
    pub afternoon_adjustment_low: f64,

    /// Adjusted close price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentClose")]
    pub afternoon_adjustment_close: f64,

    /// Adjusted trading volume of the afternoon session
    #[serde(rename = "AfternoonAdjustmentVolume")]
    pub afternoon_adjustment_volume: f64,
}

/// Represents a single daily quote
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyQuoteCommon {
    /// The date in YYYY-MM-DD format
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Open Price (before adjustment)
    #[serde(rename = "Open")]
    pub open: f64,

    /// High price (before adjustment)
    #[serde(rename = "High")]
    pub high: f64,

    /// Low price (before adjustment)
    #[serde(rename = "Low")]
    pub low: f64,

    /// Close price (before adjustment)
    #[serde(rename = "Close")]
    pub close: f64,

    /// Flag of hitting the upper price limit of the day
    #[serde(rename = "UpperLimit")]
    pub upper_limit: String,

    /// Flag of hitting the lower price limit of the day
    #[serde(rename = "LowerLimit")]
    pub lower_limit: String,

    /// Trading volume (before Adjustment)
    #[serde(rename = "Volume")]
    pub volume: f64,

    /// Trading value
    #[serde(rename = "TurnoverValue")]
    pub turnover_value: f64,

    /// Adjustment factor
    #[serde(rename = "AdjustmentFactor")]
    pub adjustment_factor: f64,

    /// Adjusted open price
    #[serde(rename = "AdjustmentOpen")]
    pub adjustment_open: f64,

    /// Adjusted high price
    #[serde(rename = "AdjustmentHigh")]
    pub adjustment_high: f64,

    /// Adjusted low price
    #[serde(rename = "AdjustmentLow")]
    pub adjustment_low: f64,

    /// Adjusted close price
    #[serde(rename = "AdjustmentClose")]
    pub adjustment_close: f64,

    /// Adjusted volume
    #[serde(rename = "AdjustmentVolume")]
    pub adjustment_volume: f64,
}

#[cfg(test)]
mod tests {
    use crate::api::stock_prices::{
        DailyQuoteCommon, DailyQuotePremiumPlan, DailyQuoteStandardPlan,
        StockPricesPremiumPlanResponse, StockPricesStandardPlanResponse,
    };

    #[test]
    fn test_deserialize_stock_prices_standard_plan_response() {
        let json = r#"
            {
                "daily_quotes": [
                    {
                        "Date": "2023-03-24",
                        "Code": "86970",
                        "Open": 2047.0,
                        "High": 2069.0,
                        "Low": 2035.0,
                        "Close": 2045.0,
                        "UpperLimit": "0",
                        "LowerLimit": "0",
                        "Volume": 2202500.0,
                        "TurnoverValue": 4507051850.0,
                        "AdjustmentFactor": 1.0,
                        "AdjustmentOpen": 2047.0,
                        "AdjustmentHigh": 2069.0,
                        "AdjustmentLow": 2035.0,
                        "AdjustmentClose": 2045.0,
                        "AdjustmentVolume": 2202500.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: StockPricesStandardPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = StockPricesStandardPlanResponse {
            daily_quotes: vec![DailyQuoteStandardPlan {
                common: DailyQuoteCommon {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: 2047.0,
                    high: 2069.0,
                    low: 2035.0,
                    close: 2045.0,
                    upper_limit: "0".to_string(),
                    lower_limit: "0".to_string(),
                    volume: 2202500.0,
                    turnover_value: 4507051850.0,
                    adjustment_factor: 1.0,
                    adjustment_open: 2047.0,
                    adjustment_high: 2069.0,
                    adjustment_low: 2035.0,
                    adjustment_close: 2045.0,
                    adjustment_volume: 2202500.0,
                },
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_stock_prices_standard_plan_response_no_pagination_key() {
        let json = r#"
            {
                "daily_quotes": [
                    {
                        "Date": "2023-03-24",
                        "Code": "86970",
                        "Open": 2047.0,
                        "High": 2069.0,
                        "Low": 2035.0,
                        "Close": 2045.0,
                        "UpperLimit": "0",
                        "LowerLimit": "0",
                        "Volume": 2202500.0,
                        "TurnoverValue": 4507051850.0,
                        "AdjustmentFactor": 1.0,
                        "AdjustmentOpen": 2047.0,
                        "AdjustmentHigh": 2069.0,
                        "AdjustmentLow": 2035.0,
                        "AdjustmentClose": 2045.0,
                        "AdjustmentVolume": 2202500.0
                    }
                ]
            }
        "#;

        let response: StockPricesStandardPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = StockPricesStandardPlanResponse {
            daily_quotes: vec![DailyQuoteStandardPlan {
                common: DailyQuoteCommon {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: 2047.0,
                    high: 2069.0,
                    low: 2035.0,
                    close: 2045.0,
                    upper_limit: "0".to_string(),
                    lower_limit: "0".to_string(),
                    volume: 2202500.0,
                    turnover_value: 4507051850.0,
                    adjustment_factor: 1.0,
                    adjustment_open: 2047.0,
                    adjustment_high: 2069.0,
                    adjustment_low: 2035.0,
                    adjustment_close: 2045.0,
                    adjustment_volume: 2202500.0,
                },
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_stock_prices_premium_plan_response() {
        let json = r#"
            {
                "daily_quotes": [
                    {
                        "Date": "2023-03-24",
                        "Code": "86970",
                        "Open": 2047.0,
                        "High": 2069.0,
                        "Low": 2035.0,
                        "Close": 2045.0,
                        "UpperLimit": "0",
                        "LowerLimit": "0",
                        "Volume": 2202500.0,
                        "TurnoverValue": 4507051850.0,
                        "AdjustmentFactor": 1.0,
                        "AdjustmentOpen": 2047.0,
                        "AdjustmentHigh": 2069.0,
                        "AdjustmentLow": 2035.0,
                        "AdjustmentClose": 2045.0,
                        "AdjustmentVolume": 2202500.0,
                        "MorningOpen": 2047.0,
                        "MorningHigh": 2069.0,
                        "MorningLow": 2040.0,
                        "MorningClose": 2045.5,
                        "MorningUpperLimit": "0",
                        "MorningLowerLimit": "0",
                        "MorningVolume": 1121200.0,
                        "MorningTurnoverValue": 2297525850.0,
                        "MorningAdjustmentOpen": 2047.0,
                        "MorningAdjustmentHigh": 2069.0,
                        "MorningAdjustmentLow": 2040.0,
                        "MorningAdjustmentClose": 2045.5,
                        "MorningAdjustmentVolume": 1121200.0,
                        "AfternoonOpen": 2047.0,
                        "AfternoonHigh": 2047.0,
                        "AfternoonLow": 2035.0,
                        "AfternoonClose": 2045.0,
                        "AfternoonUpperLimit": "0",
                        "AfternoonLowerLimit": "0",
                        "AfternoonVolume": 1081300.0,
                        "AfternoonTurnoverValue": 2209526000.0,
                        "AfternoonAdjustmentOpen": 2047.0,
                        "AfternoonAdjustmentHigh": 2047.0,
                        "AfternoonAdjustmentLow": 2035.0,
                        "AfternoonAdjustmentClose": 2045.0,
                        "AfternoonAdjustmentVolume": 1081300.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: StockPricesPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = StockPricesPremiumPlanResponse {
            daily_quotes: vec![DailyQuotePremiumPlan {
                common: DailyQuoteCommon {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: 2047.0,
                    high: 2069.0,
                    low: 2035.0,
                    close: 2045.0,
                    upper_limit: "0".to_string(),
                    lower_limit: "0".to_string(),
                    volume: 2202500.0,
                    turnover_value: 4507051850.0,
                    adjustment_factor: 1.0,
                    adjustment_open: 2047.0,
                    adjustment_high: 2069.0,
                    adjustment_low: 2035.0,
                    adjustment_close: 2045.0,
                    adjustment_volume: 2202500.0,
                },
                morning_open: 2047.0,
                morning_high: 2069.0,
                morning_low: 2040.0,
                morning_close: 2045.5,
                morning_upper_limit: "0".to_string(),
                morning_lower_limit: "0".to_string(),
                morning_volume: 1121200.0,
                morning_turnover_value: 2297525850.0,
                morning_adjustment_open: 2047.0,
                morning_adjustment_high: 2069.0,
                morning_adjustment_low: 2040.0,
                morning_adjustment_close: 2045.5,
                morning_adjustment_volume: 1121200.0,
                afternoon_open: 2047.0,
                afternoon_high: 2047.0,
                afternoon_low: 2035.0,
                afternoon_close: 2045.0,
                afternoon_upper_limit: "0".to_string(),
                afternoon_lower_limit: "0".to_string(),
                afternoon_volume: 1081300.0,
                afternoon_turnover_value: 2209526000.0,
                afternoon_adjustment_open: 2047.0,
                afternoon_adjustment_high: 2047.0,
                afternoon_adjustment_low: 2035.0,
                afternoon_adjustment_close: 2045.0,
                afternoon_adjustment_volume: 1081300.0,
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_stock_prices_premium_plan_response_no_pagination_key() {
        let json = r#"
            {
                "daily_quotes": [
                    {
                        "Date": "2023-03-24",
                        "Code": "86970",
                        "Open": 2047.0,
                        "High": 2069.0,
                        "Low": 2035.0,
                        "Close": 2045.0,
                        "UpperLimit": "0",
                        "LowerLimit": "0",
                        "Volume": 2202500.0,
                        "TurnoverValue": 4507051850.0,
                        "AdjustmentFactor": 1.0,
                        "AdjustmentOpen": 2047.0,
                        "AdjustmentHigh": 2069.0,
                        "AdjustmentLow": 2035.0,
                        "AdjustmentClose": 2045.0,
                        "AdjustmentVolume": 2202500.0,
                        "MorningOpen": 2047.0,
                        "MorningHigh": 2069.0,
                        "MorningLow": 2040.0,
                        "MorningClose": 2045.5,
                        "MorningUpperLimit": "0",
                        "MorningLowerLimit": "0",
                        "MorningVolume": 1121200.0,
                        "MorningTurnoverValue": 2297525850.0,
                        "MorningAdjustmentOpen": 2047.0,
                        "MorningAdjustmentHigh": 2069.0,
                        "MorningAdjustmentLow": 2040.0,
                        "MorningAdjustmentClose": 2045.5,
                        "MorningAdjustmentVolume": 1121200.0,
                        "AfternoonOpen": 2047.0,
                        "AfternoonHigh": 2047.0,
                        "AfternoonLow": 2035.0,
                        "AfternoonClose": 2045.0,
                        "AfternoonUpperLimit": "0",
                        "AfternoonLowerLimit": "0",
                        "AfternoonVolume": 1081300.0,
                        "AfternoonTurnoverValue": 2209526000.0,
                        "AfternoonAdjustmentOpen": 2047.0,
                        "AfternoonAdjustmentHigh": 2047.0,
                        "AfternoonAdjustmentLow": 2035.0,
                        "AfternoonAdjustmentClose": 2045.0,
                        "AfternoonAdjustmentVolume": 1081300.0
                    }
                ]
            }
        "#;

        let response: StockPricesPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = StockPricesPremiumPlanResponse {
            daily_quotes: vec![DailyQuotePremiumPlan {
                common: DailyQuoteCommon {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: 2047.0,
                    high: 2069.0,
                    low: 2035.0,
                    close: 2045.0,
                    upper_limit: "0".to_string(),
                    lower_limit: "0".to_string(),
                    volume: 2202500.0,
                    turnover_value: 4507051850.0,
                    adjustment_factor: 1.0,
                    adjustment_open: 2047.0,
                    adjustment_high: 2069.0,
                    adjustment_low: 2035.0,
                    adjustment_close: 2045.0,
                    adjustment_volume: 2202500.0,
                },
                morning_open: 2047.0,
                morning_high: 2069.0,
                morning_low: 2040.0,
                morning_close: 2045.5,
                morning_upper_limit: "0".to_string(),
                morning_lower_limit: "0".to_string(),
                morning_volume: 1121200.0,
                morning_turnover_value: 2297525850.0,
                morning_adjustment_open: 2047.0,
                morning_adjustment_high: 2069.0,
                morning_adjustment_low: 2040.0,
                morning_adjustment_close: 2045.5,
                morning_adjustment_volume: 1121200.0,
                afternoon_open: 2047.0,
                afternoon_high: 2047.0,
                afternoon_low: 2035.0,
                afternoon_close: 2045.0,
                afternoon_upper_limit: "0".to_string(),
                afternoon_lower_limit: "0".to_string(),
                afternoon_volume: 1081300.0,
                afternoon_turnover_value: 2209526000.0,
                afternoon_adjustment_open: 2047.0,
                afternoon_adjustment_high: 2047.0,
                afternoon_adjustment_low: 2035.0,
                afternoon_adjustment_close: 2045.0,
                afternoon_adjustment_volume: 1081300.0,
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
