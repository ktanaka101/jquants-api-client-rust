//! Index Option Prices(OHLC)(/option/index_option) API

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use super::{
    shared::{
        deserialize_utils::empty_string_or_null_as_none,
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::{
            emergency_margin_trigger_division::EmergencyMarginTriggerDivision,
            put_call_division::PutCallDivision,
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Index Option Prices (OHLC) Data API.
#[derive(Clone, Serialize)]
pub struct IndexOptionPricesBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Date of data (e.g., "20210901" or "2021-09-01")
    date: String,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<IndexOptionPricesResponse> for IndexOptionPricesBuilder {
    async fn send(self) -> Result<IndexOptionPricesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<IndexOptionPricesResponse, crate::JQuantsError> {
        self.client.inner.get("option/index_option", self).await
    }
}

impl Paginatable<IndexOptionPricesResponse> for IndexOptionPricesBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl IndexOptionPricesBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient, date: String) -> Self {
        Self {
            client,
            date,
            pagination_key: None,
        }
    }

    /// Set the date of data (e.g., "20210901" or "2021-09-01")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    /// Set pagination key for fetching the next set of data.
    pub fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

/// Trait for Index Option Prices (OHLC) Data API.
pub trait IndexOptionPricesApi: JQuantsPlanClient {
    /// Get API builder for Index Option Prices (OHLC) Data.
    ///
    /// Use [Index Option Prices (OHLC) (/option/index_option) API](https://jpx.gitbook.io/j-quants-en/api-reference/index_option)
    fn get_index_option_prices(&self, date: impl Into<String>) -> IndexOptionPricesBuilder {
        IndexOptionPricesBuilder::new(self.get_api_client().clone(), date.into())
    }
}

/// Index Option Prices (OHLC) Data API response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/index_option)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndexOptionPricesResponse {
    /// List of Nikkei 225 Options prices
    pub index_option: Vec<IndexOptionPriceItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for IndexOptionPricesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for IndexOptionPricesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.index_option.extend(p.index_option);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single Nikkei 225 Option price record.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IndexOptionPriceItem {
    /// Trading day (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Whole day open price
    #[serde(rename = "WholeDayOpen")]
    pub whole_day_open: f64,

    /// Whole day high price
    #[serde(rename = "WholeDayHigh")]
    pub whole_day_high: f64,

    /// Whole day low price
    #[serde(rename = "WholeDayLow")]
    pub whole_day_low: f64,

    /// Whole day close price
    #[serde(rename = "WholeDayClose")]
    pub whole_day_close: f64,

    /// Night session open price
    #[serde(
        rename = "NightSessionOpen",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub night_session_open: Option<f64>,

    /// Night session high price
    #[serde(
        rename = "NightSessionHigh",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub night_session_high: Option<f64>,

    /// Night session low price
    #[serde(
        rename = "NightSessionLow",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub night_session_low: Option<f64>,

    /// Night session close price
    #[serde(
        rename = "NightSessionClose",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub night_session_close: Option<f64>,

    /// Day session open price
    #[serde(rename = "DaySessionOpen")]
    pub day_session_open: f64,

    /// Day session high price
    #[serde(rename = "DaySessionHigh")]
    pub day_session_high: f64,

    /// Day session low price
    #[serde(rename = "DaySessionLow")]
    pub day_session_low: f64,

    /// Day session close price
    #[serde(rename = "DaySessionClose")]
    pub day_session_close: f64,

    /// Volume
    #[serde(rename = "Volume")]
    pub volume: f64,

    /// Open interest
    #[serde(rename = "OpenInterest")]
    pub open_interest: f64,

    /// Turnover value
    #[serde(rename = "TurnoverValue")]
    pub turnover_value: f64,

    /// Contract month (YYYY-MM)
    #[serde(rename = "ContractMonth")]
    pub contract_month: String,

    /// Strike price
    #[serde(rename = "StrikePrice")]
    pub strike_price: f64,

    /// Volume only auction
    #[serde(
        rename = "Volume(OnlyAuction)",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub volume_only_auction: Option<f64>,

    /// Emergency margin trigger division
    #[serde(
        rename = "EmergencyMarginTriggerDivision",
        deserialize_with = "empty_string_or_null_as_none"
    )]
    pub emergency_margin_trigger_division: Option<EmergencyMarginTriggerDivision>,

    /// Put Call division (1: Put, 2: Call)
    #[serde(rename = "PutCallDivision")]
    pub put_call_division: PutCallDivision,

    /// Last trading day (YYYY-MM-DD)
    #[serde(
        rename = "LastTradingDay",
        deserialize_with = "empty_string_or_null_as_none"
    )]
    pub last_trading_day: Option<String>,

    /// Special quotation day (YYYY-MM-DD)
    #[serde(
        rename = "SpecialQuotationDay",
        deserialize_with = "empty_string_or_null_as_none"
    )]
    pub special_quotation_day: Option<String>,

    /// Settlement price
    #[serde(
        rename = "SettlementPrice",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub settlement_price: Option<f64>,

    /// Theoretical price
    #[serde(
        rename = "TheoreticalPrice",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub theoretical_price: Option<f64>,

    /// Base volatility
    #[serde(
        rename = "BaseVolatility",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub base_volatility: Option<f64>,

    /// Underlying asset price
    #[serde(
        rename = "UnderlyingPrice",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub underlying_price: Option<f64>,

    /// Implied volatility
    #[serde(
        rename = "ImpliedVolatility",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub implied_volatility: Option<f64>,

    /// Interest rate for theoretical price calculation
    #[serde(rename = "InterestRate", deserialize_with = "deserialize_f64_or_none")]
    pub interest_rate: Option<f64>,
}

/// Helper function to deserialize fields that can be either a number or a string.
/// If the field is a number, it returns the number as `Some(f64)`.
/// If the field is a string representing a number, it parses and returns `Some(f64)`.
/// If the field is "", or any non-numeric string, it returns `None`.
fn deserialize_f64_or_none<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64OrNoneVisitor;

    impl serde::de::Visitor<'_> for F64OrNoneVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float or a string representing a float")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value.trim() {
                "" => Ok(None),
                s => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(F64OrNoneVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_index_option_prices_response() {
        let json_data = r#"
        {
            "index_option": [
                {
                    "Date": "2023-03-22",
                    "Code": "130060018",
                    "WholeDayOpen": 0.0,
                    "WholeDayHigh": 0.0,
                    "WholeDayLow": 0.0,
                    "WholeDayClose": 0.0,
                    "NightSessionOpen": 0.0,
                    "NightSessionHigh": 0.0,
                    "NightSessionLow": 0.0,
                    "NightSessionClose": 0.0,
                    "DaySessionOpen": 0.0,
                    "DaySessionHigh": 0.0,
                    "DaySessionLow": 0.0,
                    "DaySessionClose": 0.0,
                    "Volume": 0.0,
                    "OpenInterest": 330.0,
                    "TurnoverValue": 0.0,
                    "ContractMonth": "2025-06",
                    "StrikePrice": 20000.0,
                    "Volume(OnlyAuction)": 0.0,
                    "EmergencyMarginTriggerDivision": "002",
                    "PutCallDivision": "1",
                    "LastTradingDay": "2025-06-12",
                    "SpecialQuotationDay": "2025-06-13",
                    "SettlementPrice": 980.0,
                    "TheoreticalPrice": 974.641,
                    "BaseVolatility": 17.93025,
                    "UnderlyingPrice": 27466.61,
                    "ImpliedVolatility": 23.1816,
                    "InterestRate": 0.2336
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: IndexOptionPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![IndexOptionPriceItem {
            date: "2023-03-22".to_string(),
            code: "130060018".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            night_session_open: Some(0.0),
            night_session_high: Some(0.0),
            night_session_low: Some(0.0),
            night_session_close: Some(0.0),
            day_session_open: 0.0,
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 330.0,
            turnover_value: 0.0,
            contract_month: "2025-06".to_string(),
            strike_price: 20000.0,
            volume_only_auction: Some(0.0),
            emergency_margin_trigger_division: Some(EmergencyMarginTriggerDivision::Calculated),
            put_call_division: PutCallDivision::Put,
            last_trading_day: Some("2025-06-12".to_string()),
            special_quotation_day: Some("2025-06-13".to_string()),
            settlement_price: Some(980.0),
            theoretical_price: Some(974.641),
            base_volatility: Some(17.93025),
            underlying_price: Some(27466.61),
            implied_volatility: Some(23.1816),
            interest_rate: Some(0.2336),
        }];

        let expected_response = IndexOptionPricesResponse {
            index_option: expected_announcement,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_index_option_prices_response_with_missing_optional_fields() {
        let json_data = r#"
        {
            "index_option": [
                {
                    "Date": "2023-03-22",
                    "Code": "130060018",
                    "WholeDayOpen": 0.0,
                    "WholeDayHigh": 0.0,
                    "WholeDayLow": 0.0,
                    "WholeDayClose": 0.0,
                    "NightSessionOpen": "",
                    "NightSessionHigh": "",
                    "NightSessionLow": "",
                    "NightSessionClose": "",
                    "DaySessionOpen": 0.0,
                    "DaySessionHigh": 0.0,
                    "DaySessionLow": 0.0,
                    "DaySessionClose": 0.0,
                    "Volume": 0.0,
                    "OpenInterest": 0.0,
                    "TurnoverValue": 0.0,
                    "ContractMonth": "2025-06",
                    "StrikePrice": 0.0,
                    "Volume(OnlyAuction)": "",
                    "EmergencyMarginTriggerDivision": "",
                    "PutCallDivision": "1",
                    "LastTradingDay": "",
                    "SpecialQuotationDay": "",
                    "SettlementPrice": "",
                    "TheoreticalPrice": "",
                    "BaseVolatility": "",
                    "UnderlyingPrice": "",
                    "ImpliedVolatility": "",
                    "InterestRate": ""
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: IndexOptionPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![IndexOptionPriceItem {
            date: "2023-03-22".to_string(),
            code: "130060018".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            night_session_open: None,
            night_session_high: None,
            night_session_low: None,
            night_session_close: None,
            day_session_open: 0.0,
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 0.0,
            turnover_value: 0.0,
            contract_month: "2025-06".to_string(),
            strike_price: 0.0,
            volume_only_auction: None,
            emergency_margin_trigger_division: None,
            put_call_division: PutCallDivision::Put,
            last_trading_day: None,
            special_quotation_day: None,
            settlement_price: None,
            theoretical_price: None,
            base_volatility: None,
            underlying_price: None,
            implied_volatility: None,
            interest_rate: None,
        }];

        let expected_response = IndexOptionPricesResponse {
            index_option: expected_announcement,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_index_option_prices_response_multiple_items() {
        let json_data = r#"
        {
            "index_option": [
                {
                    "Date": "2023-03-22",
                    "Code": "130060018",
                    "WholeDayOpen": 1000.0,
                    "WholeDayHigh": 1050.0,
                    "WholeDayLow": 990.0,
                    "WholeDayClose": 1025.0,
                    "NightSessionOpen": 1010.0,
                    "NightSessionHigh": 1040.0,
                    "NightSessionLow": 995.0,
                    "NightSessionClose": 1030.0,
                    "DaySessionOpen": 1025.0,
                    "DaySessionHigh": 1060.0,
                    "DaySessionLow": 1000.0,
                    "DaySessionClose": 1045.0,
                    "Volume": 1500.0,
                    "OpenInterest": 330.0,
                    "TurnoverValue": 1500000.0,
                    "ContractMonth": "2025-06",
                    "StrikePrice": 20000.0,
                    "Volume(OnlyAuction)": 500.0,
                    "EmergencyMarginTriggerDivision": "002",
                    "PutCallDivision": "1",
                    "LastTradingDay": "2025-06-12",
                    "SpecialQuotationDay": "2025-06-13",
                    "SettlementPrice": 980.0,
                    "TheoreticalPrice": 974.641,
                    "BaseVolatility": 17.93025,
                    "UnderlyingPrice": 27466.61,
                    "ImpliedVolatility": 23.1816,
                    "InterestRate": 0.2336
                },
                {
                    "Date": "2023-03-22",
                    "Code": "130060019",
                    "WholeDayOpen": 2000.0,
                    "WholeDayHigh": 2050.0,
                    "WholeDayLow": 1990.0,
                    "WholeDayClose": 2025.0,
                    "NightSessionOpen": 2010.0,
                    "NightSessionHigh": 2040.0,
                    "NightSessionLow": 1995.0,
                    "NightSessionClose": 2030.0,
                    "DaySessionOpen": 2025.0,
                    "DaySessionHigh": 2060.0,
                    "DaySessionLow": 2000.0,
                    "DaySessionClose": 2045.0,
                    "Volume": 2500.0,
                    "OpenInterest": 430.0,
                    "TurnoverValue": 2500000.0,
                    "ContractMonth": "2025-07",
                    "StrikePrice": 21000.0,
                    "Volume(OnlyAuction)": 600.0,
                    "EmergencyMarginTriggerDivision": "001",
                    "PutCallDivision": "2",
                    "LastTradingDay": "2025-07-12",
                    "SpecialQuotationDay": "2025-07-13",
                    "SettlementPrice": 1980.0,
                    "TheoreticalPrice": 1974.641,
                    "BaseVolatility": 18.93025,
                    "UnderlyingPrice": 27566.61,
                    "ImpliedVolatility": 24.1816,
                    "InterestRate": 0.2436
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: IndexOptionPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![
            IndexOptionPriceItem {
                date: "2023-03-22".to_string(),
                code: "130060018".to_string(),
                whole_day_open: 1000.0,
                whole_day_high: 1050.0,
                whole_day_low: 990.0,
                whole_day_close: 1025.0,
                night_session_open: Some(1010.0),
                night_session_high: Some(1040.0),
                night_session_low: Some(995.0),
                night_session_close: Some(1030.0),
                day_session_open: 1025.0,
                day_session_high: 1060.0,
                day_session_low: 1000.0,
                day_session_close: 1045.0,
                volume: 1500.0,
                open_interest: 330.0,
                turnover_value: 1500000.0,
                contract_month: "2025-06".to_string(),
                strike_price: 20000.0,
                volume_only_auction: Some(500.0),
                emergency_margin_trigger_division: Some(EmergencyMarginTriggerDivision::Calculated),
                put_call_division: PutCallDivision::Put,
                last_trading_day: Some("2025-06-12".to_string()),
                special_quotation_day: Some("2025-06-13".to_string()),
                settlement_price: Some(980.0),
                theoretical_price: Some(974.641),
                base_volatility: Some(17.93025),
                underlying_price: Some(27466.61),
                implied_volatility: Some(23.1816),
                interest_rate: Some(0.2336),
            },
            IndexOptionPriceItem {
                date: "2023-03-22".to_string(),
                code: "130060019".to_string(),
                whole_day_open: 2000.0,
                whole_day_high: 2050.0,
                whole_day_low: 1990.0,
                whole_day_close: 2025.0,
                night_session_open: Some(2010.0),
                night_session_high: Some(2040.0),
                night_session_low: Some(1995.0),
                night_session_close: Some(2030.0),
                day_session_open: 2025.0,
                day_session_high: 2060.0,
                day_session_low: 2000.0,
                day_session_close: 2045.0,
                volume: 2500.0,
                open_interest: 430.0,
                turnover_value: 2500000.0,
                contract_month: "2025-07".to_string(),
                strike_price: 21000.0,
                volume_only_auction: Some(600.0),
                emergency_margin_trigger_division: Some(EmergencyMarginTriggerDivision::Triggered),
                put_call_division: PutCallDivision::Call,
                last_trading_day: Some("2025-07-12".to_string()),
                special_quotation_day: Some("2025-07-13".to_string()),
                settlement_price: Some(1980.0),
                theoretical_price: Some(1974.641),
                base_volatility: Some(18.93025),
                underlying_price: Some(27566.61),
                implied_volatility: Some(24.1816),
                interest_rate: Some(0.2436),
            },
        ];

        let expected_response = IndexOptionPricesResponse {
            index_option: expected_announcement,
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_index_option_prices_response_no_pagination_key() {
        let json_data = r#"
        {
            "index_option": [
                {
                    "Date": "2023-03-22",
                    "Code": "130060018",
                    "WholeDayOpen": 0.0,
                    "WholeDayHigh": 0.0,
                    "WholeDayLow": 0.0,
                    "WholeDayClose": 0.0,
                    "NightSessionOpen": 0.0,
                    "NightSessionHigh": 0.0,
                    "NightSessionLow": 0.0,
                    "NightSessionClose": 0.0,
                    "DaySessionOpen": 0.0,
                    "DaySessionHigh": 0.0,
                    "DaySessionLow": 0.0,
                    "DaySessionClose": 0.0,
                    "Volume": 0.0,
                    "OpenInterest": 330.0,
                    "TurnoverValue": 0.0,
                    "ContractMonth": "2025-06",
                    "StrikePrice": 20000.0,
                    "Volume(OnlyAuction)": 0.0,
                    "EmergencyMarginTriggerDivision": "003",
                    "PutCallDivision": "1",
                    "LastTradingDay": "2025-06-12",
                    "SpecialQuotationDay": "2025-06-13",
                    "SettlementPrice": 980.0,
                    "TheoreticalPrice": 974.641,
                    "BaseVolatility": 17.93025,
                    "UnderlyingPrice": 27466.61,
                    "ImpliedVolatility": 23.1816,
                    "InterestRate": 0.2336
                }
            ]
        }
        "#;

        let response: IndexOptionPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![IndexOptionPriceItem {
            date: "2023-03-22".to_string(),
            code: "130060018".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            night_session_open: Some(0.0),
            night_session_high: Some(0.0),
            night_session_low: Some(0.0),
            night_session_close: Some(0.0),
            day_session_open: 0.0,
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 330.0,
            turnover_value: 0.0,
            contract_month: "2025-06".to_string(),
            strike_price: 20000.0,
            volume_only_auction: Some(0.0),
            emergency_margin_trigger_division: Some(EmergencyMarginTriggerDivision::Unknown(
                "003".to_string(),
            )),
            put_call_division: PutCallDivision::Put,
            last_trading_day: Some("2025-06-12".to_string()),
            special_quotation_day: Some("2025-06-13".to_string()),
            settlement_price: Some(980.0),
            theoretical_price: Some(974.641),
            base_volatility: Some(17.93025),
            underlying_price: Some(27466.61),
            implied_volatility: Some(23.1816),
            interest_rate: Some(0.2336),
        }];

        let expected_response = IndexOptionPricesResponse {
            index_option: expected_announcement,
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_index_option_prices_response_no_data() {
        let json_data = r#"
        {
            "index_option": []
        }
        "#;

        let response: IndexOptionPricesResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = IndexOptionPricesResponse {
            index_option: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
