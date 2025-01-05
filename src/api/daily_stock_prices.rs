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

#[cfg(feature = "polars")]
fn build_common_columns(
    data: Vec<DailyQuoteCommonItem>,
) -> Result<Vec<polars::prelude::Column>, crate::polars_utils::IntoPolarsError> {
    use crate::polars_utils::build_categorical_column;
    use polars::prelude::*;

    let mut dates = Vec::with_capacity(data.len());
    let mut codes = Vec::with_capacity(data.len());
    let mut opens = Vec::with_capacity(data.len());
    let mut highs = Vec::with_capacity(data.len());
    let mut lows = Vec::with_capacity(data.len());
    let mut closes = Vec::with_capacity(data.len());
    let mut upper_limits = Vec::with_capacity(data.len());
    let mut lower_limits = Vec::with_capacity(data.len());
    let mut volumes = Vec::with_capacity(data.len());
    let mut turnover_values = Vec::with_capacity(data.len());
    let mut adjustment_factors = Vec::with_capacity(data.len());
    let mut adjustment_opens = Vec::with_capacity(data.len());
    let mut adjustment_highs = Vec::with_capacity(data.len());
    let mut adjustment_lows = Vec::with_capacity(data.len());
    let mut adjustment_closes = Vec::with_capacity(data.len());
    let mut adjustment_volumes = Vec::with_capacity(data.len());

    for common_item in data {
        let DailyQuoteCommonItem {
            date,
            code,
            open,
            high,
            low,
            close,
            upper_limit,
            lower_limit,
            volume,
            turnover_value,
            adjustment_factor,
            adjustment_open,
            adjustment_high,
            adjustment_low,
            adjustment_close,
            adjustment_volume,
        } = common_item;

        dates.push(date);
        codes.push(code);
        opens.push(open);
        highs.push(high);
        lows.push(low);
        closes.push(close);
        upper_limits.push(upper_limit);
        lower_limits.push(lower_limit);
        volumes.push(volume);
        turnover_values.push(turnover_value);
        adjustment_factors.push(adjustment_factor);
        adjustment_opens.push(adjustment_open);
        adjustment_highs.push(adjustment_high);
        adjustment_lows.push(adjustment_low);
        adjustment_closes.push(adjustment_close);
        adjustment_volumes.push(adjustment_volume);
    }

    let columns = vec![
        Column::new("Date".into(), dates).cast(&DataType::Date)?,
        build_categorical_column("Code", codes)?,
        Series::new("Open".into(), opens).into(),
        Series::new("High".into(), highs).into(),
        Series::new("Low".into(), lows).into(),
        Series::new("Close".into(), closes).into(),
        build_categorical_column("UpperLimit", upper_limits)?,
        build_categorical_column("LowerLimit", lower_limits)?,
        Series::new("Volume".into(), volumes).into(),
        Series::new("TurnoverValue".into(), turnover_values).into(),
        Series::new("AdjustmentFactor".into(), adjustment_factors).into(),
        Series::new("AdjustmentOpen".into(), adjustment_opens).into(),
        Series::new("AdjustmentHigh".into(), adjustment_highs).into(),
        Series::new("AdjustmentLow".into(), adjustment_lows).into(),
        Series::new("AdjustmentClose".into(), adjustment_closes).into(),
        Series::new("AdjustmentVolume".into(), adjustment_volumes).into(),
    ];

    Ok(columns)
}

#[cfg(feature = "polars")]
impl DailyStockPricesStandardPlanResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        let data = self.daily_quotes;
        let columns = build_common_columns(data.into_iter().map(|d| d.common).collect::<Vec<_>>())?;
        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
}

#[cfg(feature = "polars")]
impl DailyStockPricesPremiumPlanResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_categorical_column;
        use polars::prelude::*;

        let data = self.daily_quotes;

        let mut commons = Vec::with_capacity(data.len());
        let mut morning_opens = Vec::with_capacity(data.len());
        let mut morning_highs = Vec::with_capacity(data.len());
        let mut morning_lows = Vec::with_capacity(data.len());
        let mut morning_closes = Vec::with_capacity(data.len());
        let mut morning_upper_limits = Vec::with_capacity(data.len());
        let mut morning_lower_limits = Vec::with_capacity(data.len());
        let mut morning_volumes = Vec::with_capacity(data.len());
        let mut morning_turnover_values = Vec::with_capacity(data.len());
        let mut morning_adjustment_opens = Vec::with_capacity(data.len());
        let mut morning_adjustment_highs = Vec::with_capacity(data.len());
        let mut morning_adjustment_lows = Vec::with_capacity(data.len());
        let mut morning_adjustment_closes = Vec::with_capacity(data.len());
        let mut morning_adjustment_volumes = Vec::with_capacity(data.len());
        let mut afternoon_opens = Vec::with_capacity(data.len());
        let mut afternoon_highs = Vec::with_capacity(data.len());
        let mut afternoon_lows = Vec::with_capacity(data.len());
        let mut afternoon_closes = Vec::with_capacity(data.len());
        let mut afternoon_upper_limits = Vec::with_capacity(data.len());
        let mut afternoon_lower_limits = Vec::with_capacity(data.len());
        let mut afternoon_volumes = Vec::with_capacity(data.len());
        let mut afternoon_turnover_values = Vec::with_capacity(data.len());
        let mut afternoon_adjustment_opens = Vec::with_capacity(data.len());
        let mut afternoon_adjustment_highs = Vec::with_capacity(data.len());
        let mut afternoon_adjustment_lows = Vec::with_capacity(data.len());
        let mut afternoon_adjustment_closes = Vec::with_capacity(data.len());
        let mut afternoon_adjustment_volumes = Vec::with_capacity(data.len());

        for item in data {
            let DailyQuotePremiumPlanItem {
                common,
                morning_open,
                morning_high,
                morning_low,
                morning_close,
                morning_upper_limit,
                morning_lower_limit,
                morning_volume,
                morning_turnover_value,
                morning_adjustment_open,
                morning_adjustment_high,
                morning_adjustment_low,
                morning_adjustment_close,
                morning_adjustment_volume,
                afternoon_open,
                afternoon_high,
                afternoon_low,
                afternoon_close,
                afternoon_upper_limit,
                afternoon_lower_limit,
                afternoon_volume,
                afternoon_turnover_value,
                afternoon_adjustment_open,
                afternoon_adjustment_high,
                afternoon_adjustment_low,
                afternoon_adjustment_close,
                afternoon_adjustment_volume,
            } = item;

            commons.push(common);
            morning_opens.push(morning_open);
            morning_highs.push(morning_high);
            morning_lows.push(morning_low);
            morning_closes.push(morning_close);
            morning_upper_limits.push(morning_upper_limit);
            morning_lower_limits.push(morning_lower_limit);
            morning_volumes.push(morning_volume);
            morning_turnover_values.push(morning_turnover_value);
            morning_adjustment_opens.push(morning_adjustment_open);
            morning_adjustment_highs.push(morning_adjustment_high);
            morning_adjustment_lows.push(morning_adjustment_low);
            morning_adjustment_closes.push(morning_adjustment_close);
            morning_adjustment_volumes.push(morning_adjustment_volume);
            afternoon_opens.push(afternoon_open);
            afternoon_highs.push(afternoon_high);
            afternoon_lows.push(afternoon_low);
            afternoon_closes.push(afternoon_close);
            afternoon_upper_limits.push(afternoon_upper_limit);
            afternoon_lower_limits.push(afternoon_lower_limit);
            afternoon_volumes.push(afternoon_volume);
            afternoon_turnover_values.push(afternoon_turnover_value);
            afternoon_adjustment_opens.push(afternoon_adjustment_open);
            afternoon_adjustment_highs.push(afternoon_adjustment_high);
            afternoon_adjustment_lows.push(afternoon_adjustment_low);
            afternoon_adjustment_closes.push(afternoon_adjustment_close);
            afternoon_adjustment_volumes.push(afternoon_adjustment_volume);
        }

        let mut columns = build_common_columns(commons)?;
        columns.extend(vec![
            Series::new("MorningOpen".into(), morning_opens).into(),
            Series::new("MorningHigh".into(), morning_highs).into(),
            Series::new("MorningLow".into(), morning_lows).into(),
            Series::new("MorningClose".into(), morning_closes).into(),
            build_categorical_column("MorningUpperLimit", morning_upper_limits)?,
            build_categorical_column("MorningLowerLimit", morning_lower_limits)?,
            Series::new("MorningVolume".into(), morning_volumes).into(),
            Series::new("MorningTurnoverValue".into(), morning_turnover_values).into(),
            Series::new("MorningAdjustmentOpen".into(), morning_adjustment_opens).into(),
            Series::new("MorningAdjustmentHigh".into(), morning_adjustment_highs).into(),
            Series::new("MorningAdjustmentLow".into(), morning_adjustment_lows).into(),
            Series::new("MorningAdjustmentClose".into(), morning_adjustment_closes).into(),
            Series::new("MorningAdjustmentVolume".into(), morning_adjustment_volumes).into(),
            Series::new("AfternoonOpen".into(), afternoon_opens).into(),
            Series::new("AfternoonHigh".into(), afternoon_highs).into(),
            Series::new("AfternoonLow".into(), afternoon_lows).into(),
            Series::new("AfternoonClose".into(), afternoon_closes).into(),
            build_categorical_column("AfternoonUpperLimit", afternoon_upper_limits)?,
            build_categorical_column("AfternoonLowerLimit", afternoon_lower_limits)?,
            Series::new("AfternoonVolume".into(), afternoon_volumes).into(),
            Series::new("AfternoonTurnoverValue".into(), afternoon_turnover_values).into(),
            Series::new("AfternoonAdjustmentOpen".into(), afternoon_adjustment_opens).into(),
            Series::new("AfternoonAdjustmentHigh".into(), afternoon_adjustment_highs).into(),
            Series::new("AfternoonAdjustmentLow".into(), afternoon_adjustment_lows).into(),
            Series::new(
                "AfternoonAdjustmentClose".into(),
                afternoon_adjustment_closes,
            )
            .into(),
            Series::new(
                "AfternoonAdjustmentVolume".into(),
                afternoon_adjustment_volumes,
            )
            .into(),
        ]);

        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
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

    #[cfg(feature = "polars")]
    #[test]
    fn test_some_light_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = DailyStockPricesStandardPlanResponse {
            daily_quotes: vec![
                DailyQuoteStandardPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-11".to_string(),
                        code: "86970".to_string(),
                        open: Some(100.0),
                        high: Some(200.0),
                        low: Some(300.0),
                        close: Some(400.0),
                        upper_limit: PriceLimit::NotHit,
                        lower_limit: PriceLimit::Hit,
                        volume: Some(500.0),
                        turnover_value: Some(600.0),
                        adjustment_factor: 1.0,
                        adjustment_open: Some(700.0),
                        adjustment_high: Some(800.0),
                        adjustment_low: Some(900.0),
                        adjustment_close: Some(1000.0),
                        adjustment_volume: Some(1100.0),
                    },
                },
                DailyQuoteStandardPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-12".to_string(),
                        code: "86971".to_string(),
                        open: Some(10000.0),
                        high: Some(11000.0),
                        low: Some(12000.0),
                        close: Some(13000.0),
                        upper_limit: PriceLimit::NotHit,
                        lower_limit: PriceLimit::NotHit,
                        volume: Some(14000.0),
                        turnover_value: Some(15000.0),
                        adjustment_factor: 1.0,
                        adjustment_open: Some(16000.0),
                        adjustment_high: Some(17000.0),
                        adjustment_low: Some(18000.0),
                        adjustment_close: Some(19000.0),
                        adjustment_volume: Some(20000.0),
                    },
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 16)
            ┌────────────┬───────┬─────────┬─────────┬─────────┬─────────┬────────────┬────────────┬─────────┬───────────────┬──────────────────┬────────────────┬────────────────┬───────────────┬─────────────────┬──────────────────┐
            │ Date       ┆ Code  ┆ Open    ┆ High    ┆ Low     ┆ Close   ┆ UpperLimit ┆ LowerLimit ┆ Volume  ┆ TurnoverValue ┆ AdjustmentFactor ┆ AdjustmentOpen ┆ AdjustmentHigh ┆ AdjustmentLow ┆ AdjustmentClose ┆ AdjustmentVolume │
            │ ---        ┆ ---   ┆ ---     ┆ ---     ┆ ---     ┆ ---     ┆ ---        ┆ ---        ┆ ---     ┆ ---           ┆ ---              ┆ ---            ┆ ---            ┆ ---           ┆ ---             ┆ ---              │
            │ date       ┆ cat   ┆ f64     ┆ f64     ┆ f64     ┆ f64     ┆ cat        ┆ cat        ┆ f64     ┆ f64           ┆ f64              ┆ f64            ┆ f64            ┆ f64           ┆ f64             ┆ f64              │
            ╞════════════╪═══════╪═════════╪═════════╪═════════╪═════════╪════════════╪════════════╪═════════╪═══════════════╪══════════════════╪════════════════╪════════════════╪═══════════════╪═════════════════╪══════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ 100.0   ┆ 200.0   ┆ 300.0   ┆ 400.0   ┆ 0          ┆ 1          ┆ 500.0   ┆ 600.0         ┆ 1.0              ┆ 700.0          ┆ 800.0          ┆ 900.0         ┆ 1000.0          ┆ 1100.0           │
            │ 2022-11-12 ┆ 86971 ┆ 10000.0 ┆ 11000.0 ┆ 12000.0 ┆ 13000.0 ┆ 0          ┆ 0          ┆ 14000.0 ┆ 15000.0       ┆ 1.0              ┆ 16000.0        ┆ 17000.0        ┆ 18000.0       ┆ 19000.0         ┆ 20000.0          │
            └────────────┴───────┴─────────┴─────────┴─────────┴─────────┴────────────┴────────────┴─────────┴───────────────┴──────────────────┴────────────────┴────────────────┴───────────────┴─────────────────┴──────────────────┘"#]]
        .assert_eq(&df.to_string());
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_none_light_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = DailyStockPricesStandardPlanResponse {
            daily_quotes: vec![
                DailyQuoteStandardPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-11".to_string(),
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
                },
                DailyQuoteStandardPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-12".to_string(),
                        code: "86971".to_string(),
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
                },
            ],
            pagination_key: None,
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 16)
            ┌────────────┬───────┬──────┬──────┬──────┬───────┬────────────┬────────────┬────────┬───────────────┬──────────────────┬────────────────┬────────────────┬───────────────┬─────────────────┬──────────────────┐
            │ Date       ┆ Code  ┆ Open ┆ High ┆ Low  ┆ Close ┆ UpperLimit ┆ LowerLimit ┆ Volume ┆ TurnoverValue ┆ AdjustmentFactor ┆ AdjustmentOpen ┆ AdjustmentHigh ┆ AdjustmentLow ┆ AdjustmentClose ┆ AdjustmentVolume │
            │ ---        ┆ ---   ┆ ---  ┆ ---  ┆ ---  ┆ ---   ┆ ---        ┆ ---        ┆ ---    ┆ ---           ┆ ---              ┆ ---            ┆ ---            ┆ ---           ┆ ---             ┆ ---              │
            │ date       ┆ cat   ┆ f64  ┆ f64  ┆ f64  ┆ f64   ┆ cat        ┆ cat        ┆ f64    ┆ f64           ┆ f64              ┆ f64            ┆ f64            ┆ f64           ┆ f64             ┆ f64              │
            ╞════════════╪═══════╪══════╪══════╪══════╪═══════╪════════════╪════════════╪════════╪═══════════════╪══════════════════╪════════════════╪════════════════╪═══════════════╪═════════════════╪══════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ null ┆ null ┆ null ┆ null  ┆ 0          ┆ 0          ┆ null   ┆ null          ┆ 1.0              ┆ null           ┆ null           ┆ null          ┆ null            ┆ null             │
            │ 2022-11-12 ┆ 86971 ┆ null ┆ null ┆ null ┆ null  ┆ 0          ┆ 0          ┆ null   ┆ null          ┆ 1.0              ┆ null           ┆ null           ┆ null          ┆ null            ┆ null             │
            └────────────┴───────┴──────┴──────┴──────┴───────┴────────────┴────────────┴────────┴───────────────┴──────────────────┴────────────────┴────────────────┴───────────────┴─────────────────┴──────────────────┘"#]].assert_eq(&df.to_string());
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_some_premium_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = DailyStockPricesPremiumPlanResponse {
            daily_quotes: vec![
                DailyQuotePremiumPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-11".to_string(),
                        code: "86970".to_string(),
                        open: Some(100.0),
                        high: Some(200.0),
                        low: Some(300.0),
                        close: Some(400.0),
                        upper_limit: PriceLimit::NotHit,
                        lower_limit: PriceLimit::Hit,
                        volume: Some(500.0),
                        turnover_value: Some(600.0),
                        adjustment_factor: 1.0,
                        adjustment_open: Some(700.0),
                        adjustment_high: Some(800.0),
                        adjustment_low: Some(900.0),
                        adjustment_close: Some(1000.0),
                        adjustment_volume: Some(1100.0),
                    },
                    morning_open: Some(1200.0),
                    morning_high: Some(1300.0),
                    morning_low: Some(1400.0),
                    morning_close: Some(1500.0),
                    morning_upper_limit: PriceLimit::Hit,
                    morning_lower_limit: PriceLimit::Hit,
                    morning_volume: Some(1600.0),
                    morning_turnover_value: Some(1700.0),
                    morning_adjustment_open: Some(1800.0),
                    morning_adjustment_high: Some(1900.0),
                    morning_adjustment_low: Some(2000.0),
                    morning_adjustment_close: Some(2100.0),
                    morning_adjustment_volume: Some(2200.0),
                    afternoon_open: Some(2300.0),
                    afternoon_high: Some(2400.0),
                    afternoon_low: Some(2500.0),
                    afternoon_close: Some(2600.0),
                    afternoon_upper_limit: PriceLimit::Hit,
                    afternoon_lower_limit: PriceLimit::Hit,
                    afternoon_volume: Some(2700.0),
                    afternoon_turnover_value: Some(2800.0),
                    afternoon_adjustment_open: Some(2900.0),
                    afternoon_adjustment_high: Some(3000.0),
                    afternoon_adjustment_low: Some(3100.0),
                    afternoon_adjustment_close: Some(3200.0),
                    afternoon_adjustment_volume: Some(3300.0),
                },
                DailyQuotePremiumPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-12".to_string(),
                        code: "86971".to_string(),
                        open: Some(10000.0),
                        high: Some(11000.0),
                        low: Some(12000.0),
                        close: Some(13000.0),
                        upper_limit: PriceLimit::NotHit,
                        lower_limit: PriceLimit::NotHit,
                        volume: Some(14000.0),
                        turnover_value: Some(15000.0),
                        adjustment_factor: 1.0,
                        adjustment_open: Some(16000.0),
                        adjustment_high: Some(17000.0),
                        adjustment_low: Some(18000.0),
                        adjustment_close: Some(19000.0),
                        adjustment_volume: Some(20000.0),
                    },
                    morning_open: Some(21000.0),
                    morning_high: Some(22000.0),
                    morning_low: Some(23000.0),
                    morning_close: Some(24000.0),
                    morning_upper_limit: PriceLimit::NotHit,
                    morning_lower_limit: PriceLimit::NotHit,
                    morning_volume: Some(25000.0),
                    morning_turnover_value: Some(26000.0),
                    morning_adjustment_open: Some(27000.0),
                    morning_adjustment_high: Some(28000.0),
                    morning_adjustment_low: Some(29000.0),
                    morning_adjustment_close: Some(30000.0),
                    morning_adjustment_volume: Some(31000.0),
                    afternoon_open: Some(32000.0),
                    afternoon_high: Some(33000.0),
                    afternoon_low: Some(34000.0),
                    afternoon_close: Some(35000.0),
                    afternoon_upper_limit: PriceLimit::NotHit,
                    afternoon_lower_limit: PriceLimit::NotHit,
                    afternoon_volume: Some(36000.0),
                    afternoon_turnover_value: Some(37000.0),
                    afternoon_adjustment_open: Some(38000.0),
                    afternoon_adjustment_high: Some(39000.0),
                    afternoon_adjustment_low: Some(40000.0),
                    afternoon_adjustment_close: Some(41000.0),
                    afternoon_adjustment_volume: Some(42000.0),
                },
            ],
            pagination_key: Some(
                "value
            1.value2."
                    .to_string(),
            ),
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 42)
            ┌────────────┬───────┬─────────┬─────────┬─────────┬─────────┬────────────┬────────────┬─────────┬───────────────┬──────────────────┬────────────────┬────────────────┬───────────────┬─────────────────┬──────────────────┬─────────────┬─────────────┬────────────┬──────────────┬───────────────────┬───────────────────┬───────────────┬──────────────────────┬───────────────────────┬───────────────────────┬──────────────────────┬────────────────────────┬─────────────────────────┬───────────────┬───────────────┬──────────────┬────────────────┬─────────────────────┬─────────────────────┬─────────────────┬────────────────────────┬─────────────────────────┬─────────────────────────┬────────────────────────┬──────────────────────────┬───────────────────────────┐
            │ Date       ┆ Code  ┆ Open    ┆ High    ┆ Low     ┆ Close   ┆ UpperLimit ┆ LowerLimit ┆ Volume  ┆ TurnoverValue ┆ AdjustmentFactor ┆ AdjustmentOpen ┆ AdjustmentHigh ┆ AdjustmentLow ┆ AdjustmentClose ┆ AdjustmentVolume ┆ MorningOpen ┆ MorningHigh ┆ MorningLow ┆ MorningClose ┆ MorningUpperLimit ┆ MorningLowerLimit ┆ MorningVolume ┆ MorningTurnoverValue ┆ MorningAdjustmentOpen ┆ MorningAdjustmentHigh ┆ MorningAdjustmentLow ┆ MorningAdjustmentClose ┆ MorningAdjustmentVolume ┆ AfternoonOpen ┆ AfternoonHigh ┆ AfternoonLow ┆ AfternoonClose ┆ AfternoonUpperLimit ┆ AfternoonLowerLimit ┆ AfternoonVolume ┆ AfternoonTurnoverValue ┆ AfternoonAdjustmentOpen ┆ AfternoonAdjustmentHigh ┆ AfternoonAdjustmentLow ┆ AfternoonAdjustmentClose ┆ AfternoonAdjustmentVolume │
            │ ---        ┆ ---   ┆ ---     ┆ ---     ┆ ---     ┆ ---     ┆ ---        ┆ ---        ┆ ---     ┆ ---           ┆ ---              ┆ ---            ┆ ---            ┆ ---           ┆ ---             ┆ ---              ┆ ---         ┆ ---         ┆ ---        ┆ ---          ┆ ---               ┆ ---               ┆ ---           ┆ ---                  ┆ ---                   ┆ ---                   ┆ ---                  ┆ ---                    ┆ ---                     ┆ ---           ┆ ---           ┆ ---          ┆ ---            ┆ ---                 ┆ ---                 ┆ ---             ┆ ---                    ┆ ---                     ┆ ---                     ┆ ---                    ┆ ---                      ┆ ---                       │
            │ date       ┆ cat   ┆ f64     ┆ f64     ┆ f64     ┆ f64     ┆ cat        ┆ cat        ┆ f64     ┆ f64           ┆ f64              ┆ f64            ┆ f64            ┆ f64           ┆ f64             ┆ f64              ┆ f64         ┆ f64         ┆ f64        ┆ f64          ┆ cat               ┆ cat               ┆ f64           ┆ f64                  ┆ f64                   ┆ f64                   ┆ f64                  ┆ f64                    ┆ f64                     ┆ f64           ┆ f64           ┆ f64          ┆ f64            ┆ cat                 ┆ cat                 ┆ f64             ┆ f64                    ┆ f64                     ┆ f64                     ┆ f64                    ┆ f64                      ┆ f64                       │
            ╞════════════╪═══════╪═════════╪═════════╪═════════╪═════════╪════════════╪════════════╪═════════╪═══════════════╪══════════════════╪════════════════╪════════════════╪═══════════════╪═════════════════╪══════════════════╪═════════════╪═════════════╪════════════╪══════════════╪═══════════════════╪═══════════════════╪═══════════════╪══════════════════════╪═══════════════════════╪═══════════════════════╪══════════════════════╪════════════════════════╪═════════════════════════╪═══════════════╪═══════════════╪══════════════╪════════════════╪═════════════════════╪═════════════════════╪═════════════════╪════════════════════════╪═════════════════════════╪═════════════════════════╪════════════════════════╪══════════════════════════╪═══════════════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ 100.0   ┆ 200.0   ┆ 300.0   ┆ 400.0   ┆ 0          ┆ 1          ┆ 500.0   ┆ 600.0         ┆ 1.0              ┆ 700.0          ┆ 800.0          ┆ 900.0         ┆ 1000.0          ┆ 1100.0           ┆ 1200.0      ┆ 1300.0      ┆ 1400.0     ┆ 1500.0       ┆ 1                 ┆ 1                 ┆ 1600.0        ┆ 1700.0               ┆ 1800.0                ┆ 1900.0                ┆ 2000.0               ┆ 2100.0                 ┆ 2200.0                  ┆ 2300.0        ┆ 2400.0        ┆ 2500.0       ┆ 2600.0         ┆ 1                   ┆ 1                   ┆ 2700.0          ┆ 2800.0                 ┆ 2900.0                  ┆ 3000.0                  ┆ 3100.0                 ┆ 3200.0                   ┆ 3300.0                    │
            │ 2022-11-12 ┆ 86971 ┆ 10000.0 ┆ 11000.0 ┆ 12000.0 ┆ 13000.0 ┆ 0          ┆ 0          ┆ 14000.0 ┆ 15000.0       ┆ 1.0              ┆ 16000.0        ┆ 17000.0        ┆ 18000.0       ┆ 19000.0         ┆ 20000.0          ┆ 21000.0     ┆ 22000.0     ┆ 23000.0    ┆ 24000.0      ┆ 0                 ┆ 0                 ┆ 25000.0       ┆ 26000.0              ┆ 27000.0               ┆ 28000.0               ┆ 29000.0              ┆ 30000.0                ┆ 31000.0                 ┆ 32000.0       ┆ 33000.0       ┆ 34000.0      ┆ 35000.0        ┆ 0                   ┆ 0                   ┆ 36000.0         ┆ 37000.0                ┆ 38000.0                 ┆ 39000.0                 ┆ 40000.0                ┆ 41000.0                  ┆ 42000.0                   │
            └────────────┴───────┴─────────┴─────────┴─────────┴─────────┴────────────┴────────────┴─────────┴───────────────┴──────────────────┴────────────────┴────────────────┴───────────────┴─────────────────┴──────────────────┴─────────────┴─────────────┴────────────┴──────────────┴───────────────────┴───────────────────┴───────────────┴──────────────────────┴───────────────────────┴───────────────────────┴──────────────────────┴────────────────────────┴─────────────────────────┴───────────────┴───────────────┴──────────────┴────────────────┴─────────────────────┴─────────────────────┴─────────────────┴────────────────────────┴─────────────────────────┴─────────────────────────┴────────────────────────┴──────────────────────────┴───────────────────────────┘"#]].assert_eq(&df.to_string());
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_none_premium_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = DailyStockPricesPremiumPlanResponse {
            daily_quotes: vec![
                DailyQuotePremiumPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-11".to_string(),
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
                },
                DailyQuotePremiumPlanItem {
                    common: DailyQuoteCommonItem {
                        date: "2022-11-12".to_string(),
                        code: "86971".to_string(),
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
                },
            ],
            pagination_key: None,
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 42)
            ┌────────────┬───────┬──────┬──────┬──────┬───────┬────────────┬────────────┬────────┬───────────────┬──────────────────┬────────────────┬────────────────┬───────────────┬─────────────────┬──────────────────┬─────────────┬─────────────┬────────────┬──────────────┬───────────────────┬───────────────────┬───────────────┬──────────────────────┬───────────────────────┬───────────────────────┬──────────────────────┬────────────────────────┬─────────────────────────┬───────────────┬───────────────┬──────────────┬────────────────┬─────────────────────┬─────────────────────┬─────────────────┬────────────────────────┬─────────────────────────┬─────────────────────────┬────────────────────────┬──────────────────────────┬───────────────────────────┐
            │ Date       ┆ Code  ┆ Open ┆ High ┆ Low  ┆ Close ┆ UpperLimit ┆ LowerLimit ┆ Volume ┆ TurnoverValue ┆ AdjustmentFactor ┆ AdjustmentOpen ┆ AdjustmentHigh ┆ AdjustmentLow ┆ AdjustmentClose ┆ AdjustmentVolume ┆ MorningOpen ┆ MorningHigh ┆ MorningLow ┆ MorningClose ┆ MorningUpperLimit ┆ MorningLowerLimit ┆ MorningVolume ┆ MorningTurnoverValue ┆ MorningAdjustmentOpen ┆ MorningAdjustmentHigh ┆ MorningAdjustmentLow ┆ MorningAdjustmentClose ┆ MorningAdjustmentVolume ┆ AfternoonOpen ┆ AfternoonHigh ┆ AfternoonLow ┆ AfternoonClose ┆ AfternoonUpperLimit ┆ AfternoonLowerLimit ┆ AfternoonVolume ┆ AfternoonTurnoverValue ┆ AfternoonAdjustmentOpen ┆ AfternoonAdjustmentHigh ┆ AfternoonAdjustmentLow ┆ AfternoonAdjustmentClose ┆ AfternoonAdjustmentVolume │
            │ ---        ┆ ---   ┆ ---  ┆ ---  ┆ ---  ┆ ---   ┆ ---        ┆ ---        ┆ ---    ┆ ---           ┆ ---              ┆ ---            ┆ ---            ┆ ---           ┆ ---             ┆ ---              ┆ ---         ┆ ---         ┆ ---        ┆ ---          ┆ ---               ┆ ---               ┆ ---           ┆ ---                  ┆ ---                   ┆ ---                   ┆ ---                  ┆ ---                    ┆ ---                     ┆ ---           ┆ ---           ┆ ---          ┆ ---            ┆ ---                 ┆ ---                 ┆ ---             ┆ ---                    ┆ ---                     ┆ ---                     ┆ ---                    ┆ ---                      ┆ ---                       │
            │ date       ┆ cat   ┆ f64  ┆ f64  ┆ f64  ┆ f64   ┆ cat        ┆ cat        ┆ f64    ┆ f64           ┆ f64              ┆ f64            ┆ f64            ┆ f64           ┆ f64             ┆ f64              ┆ f64         ┆ f64         ┆ f64        ┆ f64          ┆ cat               ┆ cat               ┆ f64           ┆ f64                  ┆ f64                   ┆ f64                   ┆ f64                  ┆ f64                    ┆ f64                     ┆ f64           ┆ f64           ┆ f64          ┆ f64            ┆ cat                 ┆ cat                 ┆ f64             ┆ f64                    ┆ f64                     ┆ f64                     ┆ f64                    ┆ f64                      ┆ f64                       │
            ╞════════════╪═══════╪══════╪══════╪══════╪═══════╪════════════╪════════════╪════════╪═══════════════╪══════════════════╪════════════════╪════════════════╪═══════════════╪═════════════════╪══════════════════╪═════════════╪═════════════╪════════════╪══════════════╪═══════════════════╪═══════════════════╪═══════════════╪══════════════════════╪═══════════════════════╪═══════════════════════╪══════════════════════╪════════════════════════╪═════════════════════════╪═══════════════╪═══════════════╪══════════════╪════════════════╪═════════════════════╪═════════════════════╪═════════════════╪════════════════════════╪═════════════════════════╪═════════════════════════╪════════════════════════╪══════════════════════════╪═══════════════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ null ┆ null ┆ null ┆ null  ┆ 0          ┆ 0          ┆ null   ┆ null          ┆ 1.0              ┆ null           ┆ null           ┆ null          ┆ null            ┆ null             ┆ null        ┆ null        ┆ null       ┆ null         ┆ 0                 ┆ 0                 ┆ null          ┆ null                 ┆ null                  ┆ null                  ┆ null                 ┆ null                   ┆ null                    ┆ null          ┆ null          ┆ null         ┆ null           ┆ 0                   ┆ 0                   ┆ null            ┆ null                   ┆ null                    ┆ null                    ┆ null                   ┆ null                     ┆ null                      │
            │ 2022-11-12 ┆ 86971 ┆ null ┆ null ┆ null ┆ null  ┆ 0          ┆ 0          ┆ null   ┆ null          ┆ 1.0              ┆ null           ┆ null           ┆ null          ┆ null            ┆ null             ┆ null        ┆ null        ┆ null       ┆ null         ┆ 0                 ┆ 0                 ┆ null          ┆ null                 ┆ null                  ┆ null                  ┆ null                 ┆ null                   ┆ null                    ┆ null          ┆ null          ┆ null         ┆ null           ┆ 0                   ┆ 0                   ┆ null            ┆ null                   ┆ null                    ┆ null                    ┆ null                   ┆ null                     ┆ null                      │
            └────────────┴───────┴──────┴──────┴──────┴───────┴────────────┴────────────┴────────┴───────────────┴──────────────────┴────────────────┴────────────────┴───────────────┴─────────────────┴──────────────────┴─────────────┴─────────────┴────────────┴──────────────┴───────────────────┴───────────────────┴───────────────┴──────────────────────┴───────────────────────┴───────────────────────┴──────────────────────┴────────────────────────┴─────────────────────────┴───────────────┴───────────────┴──────────────┴────────────────┴─────────────────────┴─────────────────────┴─────────────────┴────────────────────────┴─────────────────────────┴─────────────────────────┴────────────────────────┴──────────────────────────┴───────────────────────────┘"#]].assert_eq(&df.to_string());
    }
}
