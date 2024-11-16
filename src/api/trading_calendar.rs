//! Trading Calendar API.

use serde::{Deserialize, Serialize};

use super::{
    shared::{traits::builder::JQuantsBuilder, types::holiday_division::HolidayDivision},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Trading Calendar API.
#[derive(Clone, Serialize)]
pub struct TradingCalendarBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Holiday division.
    #[serde(skip_serializing_if = "Option::is_none", rename = "holidaydivision")]
    holiday_division: Option<HolidayDivision>,
    /// Starting point of data period (e.g., "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g., "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

impl JQuantsBuilder<TradingCalendarResponse> for TradingCalendarBuilder {
    async fn send(self) -> Result<TradingCalendarResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<TradingCalendarResponse, crate::JQuantsError> {
        self.client
            .inner
            .get("markets/trading_calendar", self)
            .await
    }
}

impl TradingCalendarBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            holiday_division: None,
            from: None,
            to: None,
        }
    }

    /// Set holiday division.
    pub fn holiday_division(mut self, holiday_division: impl Into<HolidayDivision>) -> Self {
        self.holiday_division = Some(holiday_division.into());
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
}

/// Builder for Trading Calendar API.
pub trait TradingCalendarApi: JQuantsPlanClient {
    /// Get API builder for Trading Calendar.
    ///
    /// Use [Trading Calendar (/markets/trading_calendar) API](https://jpx.gitbook.io/j-quants-en/api-reference/trading_calendar)
    fn get_trading_calendar(&self) -> TradingCalendarBuilder {
        TradingCalendarBuilder::new(self.get_api_client().clone())
    }
}

/// Trading Calendar response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/trading_calendar)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradingCalendarResponse {
    /// List of trading calendar data
    pub trading_calendar: Vec<TradingCalendarItem>,
}

/// Represents a single trading calendar data.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradingCalendarItem {
    /// Trade date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Holiday division
    #[serde(rename = "HolidayDivision")]
    pub holiday_division: HolidayDivision,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_trading_calendar_response() {
        let json = r#"
            {
                "trading_calendar": [
                    {
                        "Date": "2015-04-01",
                        "HolidayDivision": "1"
                    }
                ]
            }
        "#;

        let response: TradingCalendarResponse = serde_json::from_str(json).unwrap();
        let expected_response = TradingCalendarResponse {
            trading_calendar: vec![TradingCalendarItem {
                date: "2015-04-01".to_string(),
                holiday_division: HolidayDivision::BusinessDay,
            }],
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_trading_calendar_response_multiple_items() {
        let json = r#"
            {
                "trading_calendar": [
                    {
                        "Date": "2015-03-25",
                        "HolidayDivision": "2"
                    },
                    {
                        "Date": "2015-04-01",
                        "HolidayDivision": "1"
                    }
                ]
            }
        "#;

        let response: TradingCalendarResponse = serde_json::from_str(json).unwrap();
        let expected_response = TradingCalendarResponse {
            trading_calendar: vec![
                TradingCalendarItem {
                    date: "2015-03-25".to_string(),
                    holiday_division: HolidayDivision::HalfDayTrading,
                },
                TradingCalendarItem {
                    date: "2015-04-01".to_string(),
                    holiday_division: HolidayDivision::BusinessDay,
                },
            ],
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_trading_calendar_response_no_data() {
        let json = r#"
            {
                "trading_calendar": []
            }
        "#;

        let response: TradingCalendarResponse = serde_json::from_str(json).unwrap();
        let expected_response = TradingCalendarResponse {
            trading_calendar: vec![],
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
