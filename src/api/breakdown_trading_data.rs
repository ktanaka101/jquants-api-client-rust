//! Breakdown Trading Data API.

use serde::{Deserialize, Serialize};

use super::{
    shared::traits::{
        builder::JQuantsBuilder,
        pagination::{HasPaginationKey, MergePage, Paginatable},
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Breakdown Trading Data API.
#[derive(Clone, Serialize)]
pub struct BreakdownTradingDataBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Issue code (e.g. "27890" or "2789")
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// Starting point of data period (e.g. "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g. "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// Date of data (e.g. "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<BreakdownTradingDataResponse> for BreakdownTradingDataBuilder {
    async fn send(self) -> Result<BreakdownTradingDataResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<BreakdownTradingDataResponse, crate::JQuantsError> {
        self.client.inner.get("markets/breakdown", self).await
    }
}

impl Paginatable<BreakdownTradingDataResponse> for BreakdownTradingDataBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl BreakdownTradingDataBuilder {
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

    /// Set issue code (e.g. "27890" or "2789")
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set starting point of data period (e.g. "20210901" or "2021-09-01")
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set end point of data period (e.g. "20210907" or "2021-09-07")
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set date of data (e.g. "20210907" or "2021-09-07")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Builder for Breakdown Trading Data API.
pub trait BreakdownTradingDataApi: JQuantsPlanClient {
    /// Get API builder for Breakdown Trading Data.
    ///
    /// Use [Breakdown Trading Data (/markets/breakdown) API](https://jpx.gitbook.io/j-quants-en/api-reference/breakdown)
    fn get_breakdown_trading_data(&self) -> BreakdownTradingDataBuilder {
        BreakdownTradingDataBuilder::new(self.get_api_client().clone())
    }
}

/// Breakdown Trading Data response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/breakdown)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BreakdownTradingDataResponse {
    /// List of breakdown trading data
    pub breakdown: Vec<BreakdownTradingDataItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for BreakdownTradingDataResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for BreakdownTradingDataResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.breakdown.extend(p.breakdown);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single breakdown trading data item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BreakdownTradingDataItem {
    /// Trade date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Long selling trading value
    #[serde(rename = "LongSellValue")]
    pub long_sell_value: f64,

    /// Trading value of short selling (excluding new margin sell)
    #[serde(rename = "ShortSellWithoutMarginValue")]
    pub short_sell_without_margin_value: f64,

    /// Trading value of new margin selling (sell orders that create new margin sell positions)
    #[serde(rename = "MarginSellNewValue")]
    pub margin_sell_new_value: f64,

    /// Trading value of closing margin selling (sell orders that close existing margin buy positions)
    #[serde(rename = "MarginSellCloseValue")]
    pub margin_sell_close_value: f64,

    /// Long buying trading value
    #[serde(rename = "LongBuyValue")]
    pub long_buy_value: f64,

    /// Trading value of new margin buying (buy orders that create new margin buy positions)
    #[serde(rename = "MarginBuyNewValue")]
    pub margin_buy_new_value: f64,

    /// Trading value of closing margin buying (buy orders that close existing margin sell positions)
    #[serde(rename = "MarginBuyCloseValue")]
    pub margin_buy_close_value: f64,

    /// Long selling trading volume
    #[serde(rename = "LongSellVolume")]
    pub long_sell_volume: f64,

    /// Trading volume of short selling (excluding new margin selling)
    #[serde(rename = "ShortSellWithoutMarginVolume")]
    pub short_sell_without_margin_volume: f64,

    /// Trading volume of new margin selling (sell orders that create new margin sell positions)
    #[serde(rename = "MarginSellNewVolume")]
    pub margin_sell_new_volume: f64,

    /// Trading volume of closing margin selling (sell orders that close existing margin buy positions)
    #[serde(rename = "MarginSellCloseVolume")]
    pub margin_sell_close_volume: f64,

    /// Long buying trading volume
    #[serde(rename = "LongBuyVolume")]
    pub long_buy_volume: f64,

    /// Trading volume of new margin buying (buy orders that create new margin buy positions)
    #[serde(rename = "MarginBuyNewVolume")]
    pub margin_buy_new_volume: f64,

    /// Trading volume of closing margin buying (buy orders that close existing margin sell positions)
    #[serde(rename = "MarginBuyCloseVolume")]
    pub margin_buy_close_volume: f64,
}

#[cfg(feature = "polars")]
impl BreakdownTradingDataResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_categorical_column;
        use polars::prelude::*;

        let data = self.breakdown;

        let mut dates = Vec::with_capacity(data.len());
        let mut codes = Vec::with_capacity(data.len());
        let mut long_sell_values = Vec::with_capacity(data.len());
        let mut short_sell_without_margin_values = Vec::with_capacity(data.len());
        let mut margin_sell_new_values = Vec::with_capacity(data.len());
        let mut margin_sell_close_values = Vec::with_capacity(data.len());
        let mut long_buy_values = Vec::with_capacity(data.len());
        let mut margin_buy_new_values = Vec::with_capacity(data.len());
        let mut margin_buy_close_values = Vec::with_capacity(data.len());
        let mut long_sell_volumes = Vec::with_capacity(data.len());
        let mut short_sell_without_margin_volumes = Vec::with_capacity(data.len());
        let mut margin_sell_new_volumes = Vec::with_capacity(data.len());
        let mut margin_sell_close_volumes = Vec::with_capacity(data.len());
        let mut long_buy_volumes = Vec::with_capacity(data.len());
        let mut margin_buy_new_volumes = Vec::with_capacity(data.len());
        let mut margin_buy_close_volumes = Vec::with_capacity(data.len());

        for item in data {
            dates.push(item.date);
            codes.push(item.code);
            long_sell_values.push(item.long_sell_value);
            short_sell_without_margin_values.push(item.short_sell_without_margin_value);
            margin_sell_new_values.push(item.margin_sell_new_value);
            margin_sell_close_values.push(item.margin_sell_close_value);
            long_buy_values.push(item.long_buy_value);
            margin_buy_new_values.push(item.margin_buy_new_value);
            margin_buy_close_values.push(item.margin_buy_close_value);
            long_sell_volumes.push(item.long_sell_volume);
            short_sell_without_margin_volumes.push(item.short_sell_without_margin_volume);
            margin_sell_new_volumes.push(item.margin_sell_new_volume);
            margin_sell_close_volumes.push(item.margin_sell_close_volume);
            long_buy_volumes.push(item.long_buy_volume);
            margin_buy_new_volumes.push(item.margin_buy_new_volume);
            margin_buy_close_volumes.push(item.margin_buy_close_volume);
        }

        let df = polars::frame::DataFrame::new(vec![
            Column::new("Date".into(), dates).cast(&DataType::Date)?,
            build_categorical_column("Code", codes)?,
            Column::new("LongSellValue".into(), long_sell_values),
            Column::new(
                "ShortSellWithoutMarginValue".into(),
                short_sell_without_margin_values,
            ),
            Column::new("MarginSellNewValue".into(), margin_sell_new_values),
            Column::new("MarginSellCloseValue".into(), margin_sell_close_values),
            Column::new("LongBuyValue".into(), long_buy_values),
            Column::new("MarginBuyNewValue".into(), margin_buy_new_values),
            Column::new("MarginBuyCloseValue".into(), margin_buy_close_values),
            Column::new("LongSellVolume".into(), long_sell_volumes),
            Column::new(
                "ShortSellWithoutMarginVolume".into(),
                short_sell_without_margin_volumes,
            ),
            Column::new("MarginSellNewVolume".into(), margin_sell_new_volumes),
            Column::new("MarginSellCloseVolume".into(), margin_sell_close_volumes),
            Column::new("LongBuyVolume".into(), long_buy_volumes),
            Column::new("MarginBuyNewVolume".into(), margin_buy_new_volumes),
            Column::new("MarginBuyCloseVolume".into(), margin_buy_close_volumes),
        ])?;

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_breakdown_trading_data_response() {
        let json = r#"
            {
                "breakdown": [
                    {
                        "Date": "2015-04-01", 
                        "Code": "13010", 
                        "LongSellValue": 115164000.0,
                        "ShortSellWithoutMarginValue": 93561000.0, 
                        "MarginSellNewValue": 6412000.0, 
                        "MarginSellCloseValue": 23009000.0, 
                        "LongBuyValue": 185114000.0, 
                        "MarginBuyNewValue": 35568000.0, 
                        "MarginBuyCloseValue": 17464000.0, 
                        "LongSellVolume": 415000.0, 
                        "ShortSellWithoutMarginVolume": 337000.0, 
                        "MarginSellNewVolume": 23000.0, 
                        "MarginSellCloseVolume": 83000.0, 
                        "LongBuyVolume": 667000.0, 
                        "MarginBuyNewVolume": 128000.0, 
                        "MarginBuyCloseVolume": 63000.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: BreakdownTradingDataResponse = serde_json::from_str(json).unwrap();
        let expected_response = BreakdownTradingDataResponse {
            breakdown: vec![BreakdownTradingDataItem {
                date: "2015-04-01".to_string(),
                code: "13010".to_string(),
                long_sell_value: 115164000.0,
                short_sell_without_margin_value: 93561000.0,
                margin_sell_new_value: 6412000.0,
                margin_sell_close_value: 23009000.0,
                long_buy_value: 185114000.0,
                margin_buy_new_value: 35568000.0,
                margin_buy_close_value: 17464000.0,
                long_sell_volume: 415000.0,
                short_sell_without_margin_volume: 337000.0,
                margin_sell_new_volume: 23000.0,
                margin_sell_close_volume: 83000.0,
                long_buy_volume: 667000.0,
                margin_buy_new_volume: 128000.0,
                margin_buy_close_volume: 63000.0,
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_breakdown_trading_data_response_no_pagination_key() {
        let json = r#"
            {
                "breakdown": [
                    {
                        "Date": "2015-04-01", 
                        "Code": "13010", 
                        "LongSellValue": 115164000.0,
                        "ShortSellWithoutMarginValue": 93561000.0, 
                        "MarginSellNewValue": 6412000.0, 
                        "MarginSellCloseValue": 23009000.0, 
                        "LongBuyValue": 185114000.0, 
                        "MarginBuyNewValue": 35568000.0, 
                        "MarginBuyCloseValue": 17464000.0, 
                        "LongSellVolume": 415000.0, 
                        "ShortSellWithoutMarginVolume": 337000.0, 
                        "MarginSellNewVolume": 23000.0, 
                        "MarginSellCloseVolume": 83000.0, 
                        "LongBuyVolume": 667000.0, 
                        "MarginBuyNewVolume": 128000.0, 
                        "MarginBuyCloseVolume": 63000.0
                    }
                ]
            }
        "#;

        let response: BreakdownTradingDataResponse = serde_json::from_str(json).unwrap();
        let expected_response = BreakdownTradingDataResponse {
            breakdown: vec![BreakdownTradingDataItem {
                date: "2015-04-01".to_string(),
                code: "13010".to_string(),
                long_sell_value: 115164000.0,
                short_sell_without_margin_value: 93561000.0,
                margin_sell_new_value: 6412000.0,
                margin_sell_close_value: 23009000.0,
                long_buy_value: 185114000.0,
                margin_buy_new_value: 35568000.0,
                margin_buy_close_value: 17464000.0,
                long_sell_volume: 415000.0,
                short_sell_without_margin_volume: 337000.0,
                margin_sell_new_volume: 23000.0,
                margin_sell_close_volume: 83000.0,
                long_buy_volume: 667000.0,
                margin_buy_new_volume: 128000.0,
                margin_buy_close_volume: 63000.0,
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_breakdown_trading_data_response_multiple_items() {
        let json = r#"
            {
                "breakdown": [
                    {
                        "Date": "2015-03-25",
                        "Code": "13010",
                        "LongSellValue": 110000000.0,
                        "ShortSellWithoutMarginValue": 90000000.0,
                        "MarginSellNewValue": 6000000.0,
                        "MarginSellCloseValue": 22000000.0,
                        "LongBuyValue": 180000000.0,
                        "MarginBuyNewValue": 35000000.0,
                        "MarginBuyCloseValue": 17000000.0,
                        "LongSellVolume": 400000.0,
                        "ShortSellWithoutMarginVolume": 330000.0,
                        "MarginSellNewVolume": 22000.0,
                        "MarginSellCloseVolume": 82000.0,
                        "LongBuyVolume": 660000.0,
                        "MarginBuyNewVolume": 125000.0,
                        "MarginBuyCloseVolume": 62000.0
                    },
                    {
                        "Date": "2015-04-01", 
                        "Code": "13010", 
                        "LongSellValue": 115164000.0,
                        "ShortSellWithoutMarginValue": 93561000.0, 
                        "MarginSellNewValue": 6412000.0, 
                        "MarginSellCloseValue": 23009000.0, 
                        "LongBuyValue": 185114000.0, 
                        "MarginBuyNewValue": 35568000.0, 
                        "MarginBuyCloseValue": 17464000.0, 
                        "LongSellVolume": 415000.0, 
                        "ShortSellWithoutMarginVolume": 337000.0, 
                        "MarginSellNewVolume": 23000.0, 
                        "MarginSellCloseVolume": 83000.0, 
                        "LongBuyVolume": 667000.0, 
                        "MarginBuyNewVolume": 128000.0, 
                        "MarginBuyCloseVolume": 63000.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: BreakdownTradingDataResponse = serde_json::from_str(json).unwrap();
        let expected_response = BreakdownTradingDataResponse {
            breakdown: vec![
                BreakdownTradingDataItem {
                    date: "2015-03-25".to_string(),
                    code: "13010".to_string(),
                    long_sell_value: 110000000.0,
                    short_sell_without_margin_value: 90000000.0,
                    margin_sell_new_value: 6000000.0,
                    margin_sell_close_value: 22000000.0,
                    long_buy_value: 180000000.0,
                    margin_buy_new_value: 35000000.0,
                    margin_buy_close_value: 17000000.0,
                    long_sell_volume: 400000.0,
                    short_sell_without_margin_volume: 330000.0,
                    margin_sell_new_volume: 22000.0,
                    margin_sell_close_volume: 82000.0,
                    long_buy_volume: 660000.0,
                    margin_buy_new_volume: 125000.0,
                    margin_buy_close_volume: 62000.0,
                },
                BreakdownTradingDataItem {
                    date: "2015-04-01".to_string(),
                    code: "13010".to_string(),
                    long_sell_value: 115164000.0,
                    short_sell_without_margin_value: 93561000.0,
                    margin_sell_new_value: 6412000.0,
                    margin_sell_close_value: 23009000.0,
                    long_buy_value: 185114000.0,
                    margin_buy_new_value: 35568000.0,
                    margin_buy_close_value: 17464000.0,
                    long_sell_volume: 415000.0,
                    short_sell_without_margin_volume: 337000.0,
                    margin_sell_new_volume: 23000.0,
                    margin_sell_close_volume: 83000.0,
                    long_buy_volume: 667000.0,
                    margin_buy_new_volume: 128000.0,
                    margin_buy_close_volume: 63000.0,
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_breakdown_trading_data_response_no_data() {
        let json = r#"
            {
                "breakdown": []
            }
        "#;

        let response: BreakdownTradingDataResponse = serde_json::from_str(json).unwrap();
        let expected_response = BreakdownTradingDataResponse {
            breakdown: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = BreakdownTradingDataResponse {
            breakdown: vec![
                BreakdownTradingDataItem {
                    date: "2015-04-01".to_string(),
                    code: "13010".to_string(),
                    long_sell_value: 100.0,
                    short_sell_without_margin_value: 200.0,
                    margin_sell_new_value: 300.0,
                    margin_sell_close_value: 400.0,
                    long_buy_value: 500.0,
                    margin_buy_new_value: 600.0,
                    margin_buy_close_value: 700.0,
                    long_sell_volume: 800.0,
                    short_sell_without_margin_volume: 900.0,
                    margin_sell_new_volume: 1000.0,
                    margin_sell_close_volume: 1100.0,
                    long_buy_volume: 1200.0,
                    margin_buy_new_volume: 1300.0,
                    margin_buy_close_volume: 1400.0,
                },
                BreakdownTradingDataItem {
                    date: "2015-04-02".to_string(),
                    code: "13010".to_string(),
                    long_sell_value: 10000.0,
                    short_sell_without_margin_value: 11000.0,
                    margin_sell_new_value: 12000.0,
                    margin_sell_close_value: 13000.0,
                    long_buy_value: 14000.0,
                    margin_buy_new_value: 15000.0,
                    margin_buy_close_value: 16000.0,
                    long_sell_volume: 17000.0,
                    short_sell_without_margin_volume: 18000.0,
                    margin_sell_new_volume: 19000.0,
                    margin_sell_close_volume: 20000.0,
                    long_buy_volume: 21000.0,
                    margin_buy_new_volume: 22000.0,
                    margin_buy_close_volume: 23000.0,
                },
            ],
            pagination_key: None,
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 16)
            ┌────────────┬───────┬───────────────┬─────────────────────────────┬────────────────────┬──────────────────────┬──────────────┬───────────────────┬─────────────────────┬────────────────┬──────────────────────────────┬─────────────────────┬───────────────────────┬───────────────┬────────────────────┬──────────────────────┐
            │ Date       ┆ Code  ┆ LongSellValue ┆ ShortSellWithoutMarginValue ┆ MarginSellNewValue ┆ MarginSellCloseValue ┆ LongBuyValue ┆ MarginBuyNewValue ┆ MarginBuyCloseValue ┆ LongSellVolume ┆ ShortSellWithoutMarginVolume ┆ MarginSellNewVolume ┆ MarginSellCloseVolume ┆ LongBuyVolume ┆ MarginBuyNewVolume ┆ MarginBuyCloseVolume │
            │ ---        ┆ ---   ┆ ---           ┆ ---                         ┆ ---                ┆ ---                  ┆ ---          ┆ ---               ┆ ---                 ┆ ---            ┆ ---                          ┆ ---                 ┆ ---                   ┆ ---           ┆ ---                ┆ ---                  │
            │ date       ┆ cat   ┆ f64           ┆ f64                         ┆ f64                ┆ f64                  ┆ f64          ┆ f64               ┆ f64                 ┆ f64            ┆ f64                          ┆ f64                 ┆ f64                   ┆ f64           ┆ f64                ┆ f64                  │
            ╞════════════╪═══════╪═══════════════╪═════════════════════════════╪════════════════════╪══════════════════════╪══════════════╪═══════════════════╪═════════════════════╪════════════════╪══════════════════════════════╪═════════════════════╪═══════════════════════╪═══════════════╪════════════════════╪══════════════════════╡
            │ 2015-04-01 ┆ 13010 ┆ 100.0         ┆ 200.0                       ┆ 300.0              ┆ 400.0                ┆ 500.0        ┆ 600.0             ┆ 700.0               ┆ 800.0          ┆ 900.0                        ┆ 1000.0              ┆ 1100.0                ┆ 1200.0        ┆ 1300.0             ┆ 1400.0               │
            │ 2015-04-02 ┆ 13010 ┆ 10000.0       ┆ 11000.0                     ┆ 12000.0            ┆ 13000.0              ┆ 14000.0      ┆ 15000.0           ┆ 16000.0             ┆ 17000.0        ┆ 18000.0                      ┆ 19000.0             ┆ 20000.0               ┆ 21000.0       ┆ 22000.0            ┆ 23000.0              │
            └────────────┴───────┴───────────────┴─────────────────────────────┴────────────────────┴──────────────────────┴──────────────┴───────────────────┴─────────────────────┴────────────────┴──────────────────────────────┴─────────────────────┴───────────────────────┴───────────────┴────────────────────┴──────────────────────┘"#]]
        .assert_eq(&df.to_string());
    }
}
