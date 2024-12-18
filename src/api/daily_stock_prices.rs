//! Prices daily quotes API.
use std::{fmt, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::PriceLimit;

use super::{
    shared::traits::{
        builder::JQuantsBuilder,
        pagination::{HasPaginationKey, MergePage, Paginatable},
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Daily Stock Prices (OHLC) API.
#[derive(Clone, Serialize)]
pub struct DailyStockPricesBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    ///
    /// If a 4-character issue code is specified,  
    /// only the data of common stock will be obtained for the issue on which both common and preferred stocks are listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// Starting point of data period (e.g. 20210901 or 2021-09-01)
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// Date of data (e.g. 20210907 or 2021-09-07)
    ///
    /// Used when `from` and `to` are not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R> for DailyStockPricesBuilder<R> {
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("prices/daily_quotes", self).await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for DailyStockPricesBuilder<R>
{
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> DailyStockPricesBuilder<R> {
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

/// Builder for Daily Stock Prices (OHLC) API.
pub trait DailyStockPricesApi: JQuantsPlanClient {
    /// Response type for listed info API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get api builder for daily stock prices.
    ///
    /// Use [Daily Stock Prices (OHLC)(/prices/daily_quotes) API](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
    fn get_daily_stock_prices(&self) -> DailyStockPricesBuilder<Self::Response> {
        DailyStockPricesBuilder::new(self.get_api_client().clone())
    }
}

/// Daily Stock prices (OHLC) response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
pub type DailyStockPricesFreePlanResponse = DailyStockPricesStandardPlanResponse;

/// Daily Stock prices (OHLC) response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
pub type DailyStockPricesLightPlanResponse = DailyStockPricesStandardPlanResponse;

/// Daily Stock prices (OHLC) response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyStockPricesStandardPlanResponse {
    /// List of daily quotes
    pub daily_quotes: Vec<DailyQuoteStandardPlanItem>,

    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}
impl HasPaginationKey for DailyStockPricesStandardPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for DailyStockPricesStandardPlanResponse {
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

/// Daily Stock prices (OHLC) response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyStockPricesPremiumPlanResponse {
    /// List of daily quotes
    pub daily_quotes: Vec<DailyQuotePremiumPlanItem>,

    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}
impl HasPaginationKey for DailyStockPricesPremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}
impl MergePage for DailyStockPricesPremiumPlanResponse {
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
pub struct DailyQuoteStandardPlanItem {
    /// The common structure for daily quote
    #[serde(flatten)]
    pub common: DailyQuoteCommonItem,
}

/// Daily Quote for premium plan.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyQuotePremiumPlanItem {
    /// The common structure for daily quote
    #[serde(flatten)]
    pub common: DailyQuoteCommonItem,

    /// Open price of the morning session (before Adjustment)
    #[serde(rename = "MorningOpen")]
    pub morning_open: Option<f64>,

    /// High price of the morning session (before Adjustment)
    #[serde(rename = "MorningHigh")]
    pub morning_high: Option<f64>,

    /// Low price of the morning session (before Adjustment)
    #[serde(rename = "MorningLow")]
    pub morning_low: Option<f64>,

    /// Close price of the morning session (before Adjustment)
    #[serde(rename = "MorningClose")]
    pub morning_close: Option<f64>,

    /// Flag of hitting the upper price limit of the day in morning session
    #[serde(rename = "MorningUpperLimit")]
    pub morning_upper_limit: PriceLimit,

    /// Flag of hitting the lower price limit of the day in morning session
    #[serde(rename = "MorningLowerLimit")]
    pub morning_lower_limit: PriceLimit,

    /// Trading volume of the morning session (before Adjustment)
    #[serde(rename = "MorningVolume")]
    pub morning_volume: Option<f64>,

    /// Trading value of the morning session
    #[serde(rename = "MorningTurnoverValue")]
    pub morning_turnover_value: Option<f64>,

    /// Adjusted open price of the morning session
    #[serde(rename = "MorningAdjustmentOpen")]
    pub morning_adjustment_open: Option<f64>,

    /// Adjusted high price of the morning session
    #[serde(rename = "MorningAdjustmentHigh")]
    pub morning_adjustment_high: Option<f64>,

    /// Adjusted low price of the morning session
    #[serde(rename = "MorningAdjustmentLow")]
    pub morning_adjustment_low: Option<f64>,

    /// Adjusted close price of the morning session
    #[serde(rename = "MorningAdjustmentClose")]
    pub morning_adjustment_close: Option<f64>,

    /// Adjusted trading volume of the morning session
    #[serde(rename = "MorningAdjustmentVolume")]
    pub morning_adjustment_volume: Option<f64>,

    /// Open price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonOpen")]
    pub afternoon_open: Option<f64>,

    /// High price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonHigh")]
    pub afternoon_high: Option<f64>,

    /// Low price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonLow")]
    pub afternoon_low: Option<f64>,

    /// Close price of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonClose")]
    pub afternoon_close: Option<f64>,

    /// Flag of hitting the upper price limit of the day in afternoon session
    #[serde(rename = "AfternoonUpperLimit")]
    pub afternoon_upper_limit: PriceLimit,

    /// Flag of hitting the lower price limit of the day in afternoon session
    #[serde(rename = "AfternoonLowerLimit")]
    pub afternoon_lower_limit: PriceLimit,

    /// Trading volume of the afternoon session (before Adjustment)
    #[serde(rename = "AfternoonVolume")]
    pub afternoon_volume: Option<f64>,

    /// Trading value of the afternoon session
    #[serde(rename = "AfternoonTurnoverValue")]
    pub afternoon_turnover_value: Option<f64>,

    /// Adjusted open price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentOpen")]
    pub afternoon_adjustment_open: Option<f64>,

    /// Adjusted high price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentHigh")]
    pub afternoon_adjustment_high: Option<f64>,

    /// Adjusted low price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentLow")]
    pub afternoon_adjustment_low: Option<f64>,

    /// Adjusted close price of the afternoon session
    #[serde(rename = "AfternoonAdjustmentClose")]
    pub afternoon_adjustment_close: Option<f64>,

    /// Adjusted trading volume of the afternoon session
    #[serde(rename = "AfternoonAdjustmentVolume")]
    pub afternoon_adjustment_volume: Option<f64>,
}

/// Represents a single daily quote
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DailyQuoteCommonItem {
    /// Date (YYYY-MM-DD).
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Open Price (before adjustment)
    #[serde(rename = "Open")]
    pub open: Option<f64>,

    /// High price (before adjustment)
    #[serde(rename = "High")]
    pub high: Option<f64>,

    /// Low price (before adjustment)
    #[serde(rename = "Low")]
    pub low: Option<f64>,

    /// Close price (before adjustment)
    #[serde(rename = "Close")]
    pub close: Option<f64>,

    /// Flag of hitting the upper price limit of the day
    #[serde(rename = "UpperLimit")]
    pub upper_limit: PriceLimit,

    /// Flag of hitting the lower price limit of the day
    #[serde(rename = "LowerLimit")]
    pub lower_limit: PriceLimit,

    /// Trading volume (before Adjustment)
    #[serde(rename = "Volume")]
    pub volume: Option<f64>,

    /// Trading value
    #[serde(rename = "TurnoverValue")]
    pub turnover_value: Option<f64>,

    /// Adjustment factor
    #[serde(rename = "AdjustmentFactor")]
    pub adjustment_factor: f64,

    /// Adjusted open price
    #[serde(rename = "AdjustmentOpen")]
    pub adjustment_open: Option<f64>,

    /// Adjusted high price
    #[serde(rename = "AdjustmentHigh")]
    pub adjustment_high: Option<f64>,

    /// Adjusted low price
    #[serde(rename = "AdjustmentLow")]
    pub adjustment_low: Option<f64>,

    /// Adjusted close price
    #[serde(rename = "AdjustmentClose")]
    pub adjustment_close: Option<f64>,

    /// Adjusted volume
    #[serde(rename = "AdjustmentVolume")]
    pub adjustment_volume: Option<f64>,
}

#[cfg(test)]
mod tests {
    use crate::{
        api::daily_stock_prices::{
            DailyQuoteCommonItem, DailyQuotePremiumPlanItem, DailyQuoteStandardPlanItem,
            DailyStockPricesPremiumPlanResponse, DailyStockPricesStandardPlanResponse,
        },
        PriceLimit,
    };

    #[test]
    fn test_deserialize_daily_stock_prices_standard_plan_response() {
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

        let response: DailyStockPricesStandardPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = DailyStockPricesStandardPlanResponse {
            daily_quotes: vec![DailyQuoteStandardPlanItem {
                common: DailyQuoteCommonItem {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: Some(2047.0),
                    high: Some(2069.0),
                    low: Some(2035.0),
                    close: Some(2045.0),
                    upper_limit: PriceLimit::NotHit,
                    lower_limit: PriceLimit::NotHit,
                    volume: Some(2202500.0),
                    turnover_value: Some(4507051850.0),
                    adjustment_factor: 1.0,
                    adjustment_open: Some(2047.0),
                    adjustment_high: Some(2069.0),
                    adjustment_low: Some(2035.0),
                    adjustment_close: Some(2045.0),
                    adjustment_volume: Some(2202500.0),
                },
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_daily_stock_prices_standard_plan_response_no_pagination_key() {
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

        let response: DailyStockPricesStandardPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = DailyStockPricesStandardPlanResponse {
            daily_quotes: vec![DailyQuoteStandardPlanItem {
                common: DailyQuoteCommonItem {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: Some(2047.0),
                    high: Some(2069.0),
                    low: Some(2035.0),
                    close: Some(2045.0),
                    upper_limit: PriceLimit::NotHit,
                    lower_limit: PriceLimit::NotHit,
                    volume: Some(2202500.0),
                    turnover_value: Some(4507051850.0),
                    adjustment_factor: 1.0,
                    adjustment_open: Some(2047.0),
                    adjustment_high: Some(2069.0),
                    adjustment_low: Some(2035.0),
                    adjustment_close: Some(2045.0),
                    adjustment_volume: Some(2202500.0),
                },
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_daily_stock_prices_premium_plan_response() {
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
                        "UpperLimit": "1",
                        "LowerLimit": "1",
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
                        "MorningUpperLimit": "1",
                        "MorningLowerLimit": "1",
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
                        "AfternoonUpperLimit": "1",
                        "AfternoonLowerLimit": "1",
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

        let response: DailyStockPricesPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = DailyStockPricesPremiumPlanResponse {
            daily_quotes: vec![DailyQuotePremiumPlanItem {
                common: DailyQuoteCommonItem {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: Some(2047.0),
                    high: Some(2069.0),
                    low: Some(2035.0),
                    close: Some(2045.0),
                    upper_limit: PriceLimit::Hit,
                    lower_limit: PriceLimit::Hit,
                    volume: Some(2202500.0),
                    turnover_value: Some(4507051850.0),
                    adjustment_factor: 1.0,
                    adjustment_open: Some(2047.0),
                    adjustment_high: Some(2069.0),
                    adjustment_low: Some(2035.0),
                    adjustment_close: Some(2045.0),
                    adjustment_volume: Some(2202500.0),
                },
                morning_open: Some(2047.0),
                morning_high: Some(2069.0),
                morning_low: Some(2040.0),
                morning_close: Some(2045.5),
                morning_upper_limit: PriceLimit::Hit,
                morning_lower_limit: PriceLimit::Hit,
                morning_volume: Some(1121200.0),
                morning_turnover_value: Some(2297525850.0),
                morning_adjustment_open: Some(2047.0),
                morning_adjustment_high: Some(2069.0),
                morning_adjustment_low: Some(2040.0),
                morning_adjustment_close: Some(2045.5),
                morning_adjustment_volume: Some(1121200.0),
                afternoon_open: Some(2047.0),
                afternoon_high: Some(2047.0),
                afternoon_low: Some(2035.0),
                afternoon_close: Some(2045.0),
                afternoon_upper_limit: PriceLimit::Hit,
                afternoon_lower_limit: PriceLimit::Hit,
                afternoon_volume: Some(1081300.0),
                afternoon_turnover_value: Some(2209526000.0),
                afternoon_adjustment_open: Some(2047.0),
                afternoon_adjustment_high: Some(2047.0),
                afternoon_adjustment_low: Some(2035.0),
                afternoon_adjustment_close: Some(2045.0),
                afternoon_adjustment_volume: Some(1081300.0),
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_daily_stock_prices_premium_plan_response_no_data() {
        let json = r#"
            {
                "daily_quotes": [
                    {
                        "Date": "2023-03-24",
                        "Code": "86970",
                        "Open": null,
                        "High": null,
                        "Low": null,
                        "Close": null,
                        "UpperLimit": "0",
                        "LowerLimit": "0",
                        "Volume": null,
                        "TurnoverValue": null,
                        "AdjustmentFactor": 1.0,
                        "AdjustmentOpen": null,
                        "AdjustmentHigh": null,
                        "AdjustmentLow": null,
                        "AdjustmentClose": null,
                        "AdjustmentVolume": null,
                        "MorningOpen": null,
                        "MorningHigh": null,
                        "MorningLow": null,
                        "MorningClose": null,
                        "MorningUpperLimit": "0",
                        "MorningLowerLimit": "0",
                        "MorningVolume": null,
                        "MorningTurnoverValue": null,
                        "MorningAdjustmentOpen": null,
                        "MorningAdjustmentHigh": null,
                        "MorningAdjustmentLow": null,
                        "MorningAdjustmentClose": null,
                        "MorningAdjustmentVolume": null,
                        "AfternoonOpen": null,
                        "AfternoonHigh": null,
                        "AfternoonLow": null,
                        "AfternoonClose": null,
                        "AfternoonUpperLimit": "0",
                        "AfternoonLowerLimit": "0",
                        "AfternoonVolume": null,
                        "AfternoonTurnoverValue": null,
                        "AfternoonAdjustmentOpen": null,
                        "AfternoonAdjustmentHigh": null,
                        "AfternoonAdjustmentLow": null,
                        "AfternoonAdjustmentClose": null,
                        "AfternoonAdjustmentVolume": null
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: DailyStockPricesPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = DailyStockPricesPremiumPlanResponse {
            daily_quotes: vec![DailyQuotePremiumPlanItem {
                common: DailyQuoteCommonItem {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: None,
                    high: None,
                    low: None,
                    close: None,
                    upper_limit: PriceLimit::NotHit,
                    lower_limit: PriceLimit::NotHit,
                    volume: None,
                    turnover_value: None,
                    adjustment_factor: 1.0,
                    adjustment_open: None,
                    adjustment_high: None,
                    adjustment_low: None,
                    adjustment_close: None,
                    adjustment_volume: None,
                },
                morning_open: None,
                morning_high: None,
                morning_low: None,
                morning_close: None,
                morning_upper_limit: PriceLimit::NotHit,
                morning_lower_limit: PriceLimit::NotHit,
                morning_volume: None,
                morning_turnover_value: None,
                morning_adjustment_open: None,
                morning_adjustment_high: None,
                morning_adjustment_low: None,
                morning_adjustment_close: None,
                morning_adjustment_volume: None,
                afternoon_open: None,
                afternoon_high: None,
                afternoon_low: None,
                afternoon_close: None,
                afternoon_upper_limit: PriceLimit::NotHit,
                afternoon_lower_limit: PriceLimit::NotHit,
                afternoon_volume: None,
                afternoon_turnover_value: None,
                afternoon_adjustment_open: None,
                afternoon_adjustment_high: None,
                afternoon_adjustment_low: None,
                afternoon_adjustment_close: None,
                afternoon_adjustment_volume: None,
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_daily_stock_prices_premium_plan_response_no_pagination_key() {
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

        let response: DailyStockPricesPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = DailyStockPricesPremiumPlanResponse {
            daily_quotes: vec![DailyQuotePremiumPlanItem {
                common: DailyQuoteCommonItem {
                    date: "2023-03-24".to_string(),
                    code: "86970".to_string(),
                    open: Some(2047.0),
                    high: Some(2069.0),
                    low: Some(2035.0),
                    close: Some(2045.0),
                    upper_limit: PriceLimit::NotHit,
                    lower_limit: PriceLimit::NotHit,
                    volume: Some(2202500.0),
                    turnover_value: Some(4507051850.0),
                    adjustment_factor: 1.0,
                    adjustment_open: Some(2047.0),
                    adjustment_high: Some(2069.0),
                    adjustment_low: Some(2035.0),
                    adjustment_close: Some(2045.0),
                    adjustment_volume: Some(2202500.0),
                },
                morning_open: Some(2047.0),
                morning_high: Some(2069.0),
                morning_low: Some(2040.0),
                morning_close: Some(2045.5),
                morning_upper_limit: PriceLimit::NotHit,
                morning_lower_limit: PriceLimit::NotHit,
                morning_volume: Some(1121200.0),
                morning_turnover_value: Some(2297525850.0),
                morning_adjustment_open: Some(2047.0),
                morning_adjustment_high: Some(2069.0),
                morning_adjustment_low: Some(2040.0),
                morning_adjustment_close: Some(2045.5),
                morning_adjustment_volume: Some(1121200.0),
                afternoon_open: Some(2047.0),
                afternoon_high: Some(2047.0),
                afternoon_low: Some(2035.0),
                afternoon_close: Some(2045.0),
                afternoon_upper_limit: PriceLimit::NotHit,
                afternoon_lower_limit: PriceLimit::NotHit,
                afternoon_volume: Some(1081300.0),
                afternoon_turnover_value: Some(2209526000.0),
                afternoon_adjustment_open: Some(2047.0),
                afternoon_adjustment_high: Some(2047.0),
                afternoon_adjustment_low: Some(2035.0),
                afternoon_adjustment_close: Some(2045.0),
                afternoon_adjustment_volume: Some(1081300.0),
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
