//! Margin Trading Outstandings API.

pub mod issue_type;

use std::{fmt, marker::PhantomData};

use issue_type::IssueType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    builder::JQuantsBuilder,
    pagination::{HasPaginationKey, MergePage, Paginatable},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Margin Trading Outstandings API.
#[derive(Clone, Serialize)]
pub struct WeeklyMarginTradingOutstandingsBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Date of data (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Starting point of data period (e.g. 20210901 or 2021-09-01)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End point of data period (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R>
    for WeeklyMarginTradingOutstandingsBuilder<R>
{
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client
            .inner
            .get("markets/weekly_margin_interest", self)
            .await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for WeeklyMarginTradingOutstandingsBuilder<R>
{
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> WeeklyMarginTradingOutstandingsBuilder<R> {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            date: None,
            from: None,
            to: None,
            pagination_key: None,
        }
    }

    /// Set issue code (e.g. 27800 or 2780)
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set date of data (e.g. 20210907 or 2021-09-07)
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
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
}

/// Builder for Margin Trading Outstandings API.
pub trait WeeklyMarginTradingOutstandingsApi: JQuantsPlanClient {
    /// Response type for Margin Trading Outstandings API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get API builder for Margin Trading Outstandings.
    ///
    /// Use [Margin Trading Outstandings (/markets/weekly_margin_interest) API](https://jpx.gitbook.io/j-quants-en/api-reference/weekly_margin_interest)
    fn get_weekly_margin_trading_outstandings(
        &self,
    ) -> WeeklyMarginTradingOutstandingsBuilder<Self::Response> {
        WeeklyMarginTradingOutstandingsBuilder::new(self.get_api_client().clone())
    }
}

/// Margin Trading Outstandings response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/weekly_margin_interest)
pub type WeeklyMarginTradingOutstandingsFreePlanResponse =
    WeeklyMarginTradingOutstandingsPremiumPlanResponse;

/// Margin Trading Outstandings response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/weekly_margin_interest)
pub type WeeklyMarginTradingOutstandingsLightPlanResponse =
    WeeklyMarginTradingOutstandingsPremiumPlanResponse;

/// Margin Trading Outstandings response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/weekly_margin_interest)
pub type WeeklyMarginTradingOutstandingsStandardPlanResponse =
    WeeklyMarginTradingOutstandingsPremiumPlanResponse;

/// Margin Trading Outstandings response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/weekly_margin_interest)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WeeklyMarginTradingOutstandingsPremiumPlanResponse {
    /// List of weekly margin trading outstandings
    pub weekly_margin_interest: Vec<WeeklyMarginTradingOutstandingsItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for WeeklyMarginTradingOutstandingsPremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for WeeklyMarginTradingOutstandingsPremiumPlanResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged
                .weekly_margin_interest
                .extend(p.weekly_margin_interest);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single weekly margin trading outstandings item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WeeklyMarginTradingOutstandingsItem {
    /// Record Date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Total margin trading (negotiable and standardized) weekend short positions
    #[serde(rename = "ShortMarginTradeVolume")]
    pub short_margin_trade_volume: f64,

    /// Total margin trading (negotiable and standardized) weekend long positions
    #[serde(rename = "LongMarginTradeVolume")]
    pub long_margin_trade_volume: f64,

    /// Negotiable margin trading weekend short positions
    #[serde(rename = "ShortNegotiableMarginTradeVolume")]
    pub short_negotiable_margin_trade_volume: f64,

    /// Negotiable margin trading weekend long positions
    #[serde(rename = "LongNegotiableMarginTradeVolume")]
    pub long_negotiable_margin_trade_volume: f64,

    /// Standardized margin trading weekend short positions
    #[serde(rename = "ShortStandardizedMarginTradeVolume")]
    pub short_standardized_margin_trade_volume: f64,

    /// Standardized margin trading weekend long positions
    #[serde(rename = "LongStandardizedMarginTradeVolume")]
    pub long_standardized_margin_trade_volume: f64,

    /// Issue Classifications
    #[serde(rename = "IssueType")]
    pub issue_type: IssueType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_weekly_margin_trading_outstandings_premium_plan_response() {
        let json = r#"
            {
                "weekly_margin_interest": [
                    {
                        "Date": "2023-02-17",
                        "Code": "13010",
                        "ShortMarginTradeVolume": 4100.0,
                        "LongMarginTradeVolume": 27600.0,
                        "ShortNegotiableMarginTradeVolume": 1300.0,
                        "LongNegotiableMarginTradeVolume": 7600.0,
                        "ShortStandardizedMarginTradeVolume": 2800.0,
                        "LongStandardizedMarginTradeVolume": 20000.0,
                        "IssueType": "2"
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: WeeklyMarginTradingOutstandingsPremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = WeeklyMarginTradingOutstandingsPremiumPlanResponse {
            weekly_margin_interest: vec![WeeklyMarginTradingOutstandingsItem {
                date: "2023-02-17".to_string(),
                code: "13010".to_string(),
                short_margin_trade_volume: 4100.0,
                long_margin_trade_volume: 27600.0,
                short_negotiable_margin_trade_volume: 1300.0,
                long_negotiable_margin_trade_volume: 7600.0,
                short_standardized_margin_trade_volume: 2800.0,
                long_standardized_margin_trade_volume: 20000.0,
                issue_type: IssueType::Loan, // Assuming "2" corresponds to Loan
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_weekly_margin_trading_outstandings_premium_plan_response_no_pagination_key()
    {
        let json = r#"
            {
                "weekly_margin_interest": [
                    {
                        "Date": "2023-02-17",
                        "Code": "13010",
                        "ShortMarginTradeVolume": 4100.0,
                        "LongMarginTradeVolume": 27600.0,
                        "ShortNegotiableMarginTradeVolume": 1300.0,
                        "LongNegotiableMarginTradeVolume": 7600.0,
                        "ShortStandardizedMarginTradeVolume": 2800.0,
                        "LongStandardizedMarginTradeVolume": 20000.0,
                        "IssueType": "2"
                    }
                ]
            }
        "#;

        let response: WeeklyMarginTradingOutstandingsPremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = WeeklyMarginTradingOutstandingsPremiumPlanResponse {
            weekly_margin_interest: vec![WeeklyMarginTradingOutstandingsItem {
                date: "2023-02-17".to_string(),
                code: "13010".to_string(),
                short_margin_trade_volume: 4100.0,
                long_margin_trade_volume: 27600.0,
                short_negotiable_margin_trade_volume: 1300.0,
                long_negotiable_margin_trade_volume: 7600.0,
                short_standardized_margin_trade_volume: 2800.0,
                long_standardized_margin_trade_volume: 20000.0,
                issue_type: IssueType::Loan, // Assuming "2" corresponds to Loan
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_weekly_margin_trading_outstandings_premium_plan_response_multiple_items() {
        let json = r#"
            {
                "weekly_margin_interest": [
                    {
                        "Date": "2023-02-10",
                        "Code": "13010",
                        "ShortMarginTradeVolume": 4000.0,
                        "LongMarginTradeVolume": 27000.0,
                        "ShortNegotiableMarginTradeVolume": 1200.0,
                        "LongNegotiableMarginTradeVolume": 7500.0,
                        "ShortStandardizedMarginTradeVolume": 2800.0,
                        "LongStandardizedMarginTradeVolume": 19500.0,
                        "IssueType": "2"
                    },
                    {
                        "Date": "2023-02-17",
                        "Code": "13010",
                        "ShortMarginTradeVolume": 4100.0,
                        "LongMarginTradeVolume": 27600.0,
                        "ShortNegotiableMarginTradeVolume": 1300.0,
                        "LongNegotiableMarginTradeVolume": 7600.0,
                        "ShortStandardizedMarginTradeVolume": 2800.0,
                        "LongStandardizedMarginTradeVolume": 20000.0,
                        "IssueType": "2"
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: WeeklyMarginTradingOutstandingsPremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = WeeklyMarginTradingOutstandingsPremiumPlanResponse {
            weekly_margin_interest: vec![
                WeeklyMarginTradingOutstandingsItem {
                    date: "2023-02-10".to_string(),
                    code: "13010".to_string(),
                    short_margin_trade_volume: 4000.0,
                    long_margin_trade_volume: 27000.0,
                    short_negotiable_margin_trade_volume: 1200.0,
                    long_negotiable_margin_trade_volume: 7500.0,
                    short_standardized_margin_trade_volume: 2800.0,
                    long_standardized_margin_trade_volume: 19500.0,
                    issue_type: IssueType::Loan, // Assuming "2" corresponds to Loan
                },
                WeeklyMarginTradingOutstandingsItem {
                    date: "2023-02-17".to_string(),
                    code: "13010".to_string(),
                    short_margin_trade_volume: 4100.0,
                    long_margin_trade_volume: 27600.0,
                    short_negotiable_margin_trade_volume: 1300.0,
                    long_negotiable_margin_trade_volume: 7600.0,
                    short_standardized_margin_trade_volume: 2800.0,
                    long_standardized_margin_trade_volume: 20000.0,
                    issue_type: IssueType::Loan, // Assuming "2" corresponds to Loan
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_weekly_margin_trading_outstandings_premium_plan_response_no_data() {
        let json = r#"
            {
                "weekly_margin_interest": []
            }
        "#;

        let response: WeeklyMarginTradingOutstandingsPremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = WeeklyMarginTradingOutstandingsPremiumPlanResponse {
            weekly_margin_interest: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
