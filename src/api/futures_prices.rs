//! Futures OHLC (/derivatives/futures) API

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
            central_contract_month_flag::CentralContractMonthFlag,
            emergency_margin_trigger_division::EmergencyMarginTriggerDivision,
            futures_code::FuturesCode,
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Futures (OHLC) Data API.
#[derive(Clone, Serialize)]
pub struct FuturesPricesBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Category of data
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<FuturesCode>,

    /// Date of data (e.g., "20210901" or "2021-09-01")
    date: String,

    /// Central contract month flag
    #[serde(skip_serializing_if = "Option::is_none")]
    central_contract_month_flag: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<FuturesPricesResponse> for FuturesPricesBuilder {
    async fn send(self) -> Result<FuturesPricesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<FuturesPricesResponse, crate::JQuantsError> {
        self.client.inner.get("derivatives/futures", self).await
    }
}

impl Paginatable<FuturesPricesResponse> for FuturesPricesBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl FuturesPricesBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient, date: String) -> Self {
        Self {
            client,
            category: None,
            date,
            central_contract_month_flag: None,
            pagination_key: None,
        }
    }

    /// Set the category of data.
    pub fn category(mut self, category: impl Into<FuturesCode>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set the date of data (e.g., "20210901" or "2021-09-01")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    /// Set the central contract month flag.
    pub fn central_contract_month_flag(mut self, flag: impl Into<String>) -> Self {
        self.central_contract_month_flag = Some(flag.into());
        self
    }

    /// Set pagination key for fetching the next set of data.
    pub fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

/// Trait for Futures (OHLC) Data API.
pub trait FuturesPricesApi: JQuantsPlanClient {
    /// Get API builder for Futures (OHLC) Data.
    ///
    /// Use [Futures (OHLC) (/derivatives/futures) API](https://jpx.gitbook.io/j-quants-en/api-reference/futures)
    fn get_futures_prices(&self, date: impl Into<String>) -> FuturesPricesBuilder {
        FuturesPricesBuilder::new(self.get_api_client().clone(), date.into())
    }
}

/// Futures (OHLC) Data API response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/futures)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FuturesPricesResponse {
    /// List of Futures prices
    pub futures: Vec<FuturesPricesItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for FuturesPricesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for FuturesPricesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.futures.extend(p.futures);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single Futures price record.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FuturesPricesItem {
    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Derivative Product Category
    #[serde(rename = "DerivativesProductCategory")]
    pub derivatives_product_category: String,

    /// Trading day (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

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

    /// Morning session open price
    #[serde(
        rename = "MorningSessionOpen",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub morning_session_open: Option<f64>,

    /// Morning session high price
    #[serde(
        rename = "MorningSessionHigh",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub morning_session_high: Option<f64>,

    /// Morning session low price
    #[serde(
        rename = "MorningSessionLow",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub morning_session_low: Option<f64>,

    /// Morning session close price
    #[serde(
        rename = "MorningSessionClose",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub morning_session_close: Option<f64>,

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

    /// Volume only auction
    #[serde(
        rename = "Volume(OnlyAuction)",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub volume_only_auction: Option<f64>,

    /// Emergency margin trigger division
    #[serde(rename = "EmergencyMarginTriggerDivision")]
    pub emergency_margin_trigger_division: EmergencyMarginTriggerDivision,

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

    /// Central contract month flag
    #[serde(
        rename = "CentralContractMonthFlag",
        deserialize_with = "empty_string_or_null_as_none"
    )]
    pub central_contract_month_flag: Option<CentralContractMonthFlag>,
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

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value as f64))
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
    fn test_deserialize_futures_prices_response() {
        let json_data = r#"
        {
            "futures": [
                {
                    "Code": "169090005",
                    "DerivativesProductCategory": "TOPIXF",
                    "Date": "2024-07-23", 
                    "WholeDayOpen": 2825.5, 
                    "WholeDayHigh": 2853.0, 
                    "WholeDayLow": 2825.5, 
                    "WholeDayClose": 2829.0, 
                    "MorningSessionOpen": "", 
                    "MorningSessionHigh": "", 
                    "MorningSessionLow": "", 
                    "MorningSessionClose": "", 
                    "NightSessionOpen": 2825.5, 
                    "NightSessionHigh": 2850.0, 
                    "NightSessionLow": 2825.5, 
                    "NightSessionClose": 2845.0, 
                    "DaySessionOpen": 2850.5, 
                    "DaySessionHigh": 2853.0, 
                    "DaySessionLow": 2826.0, 
                    "DaySessionClose": 2829.0, 
                    "Volume": 42910.0, 
                    "OpenInterest": 479812.0, 
                    "TurnoverValue": 1217918971856.0, 
                    "ContractMonth": "2024-09", 
                    "Volume(OnlyAuction)": 40405.0, 
                    "EmergencyMarginTriggerDivision": "002", 
                    "LastTradingDay": "2024-09-12", 
                    "SpecialQuotationDay": "2024-09-13", 
                    "SettlementPrice": 2829.0, 
                    "CentralContractMonthFlag": "1"
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: FuturesPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_futures = vec![FuturesPricesItem {
            code: "169090005".to_string(),
            derivatives_product_category: "TOPIXF".to_string(),
            date: "2024-07-23".to_string(),
            whole_day_open: 2825.5,
            whole_day_high: 2853.0,
            whole_day_low: 2825.5,
            whole_day_close: 2829.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: Some(2825.5),
            night_session_high: Some(2850.0),
            night_session_low: Some(2825.5),
            night_session_close: Some(2845.0),
            day_session_open: 2850.5,
            day_session_high: 2853.0,
            day_session_low: 2826.0,
            day_session_close: 2829.0,
            volume: 42910.0,
            open_interest: 479812.0,
            turnover_value: 1217918971856.0,
            contract_month: "2024-09".to_string(),
            volume_only_auction: Some(40405.0),
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
            last_trading_day: Some("2024-09-12".to_string()),
            special_quotation_day: Some("2024-09-13".to_string()),
            settlement_price: Some(2829.0),
            central_contract_month_flag: Some(CentralContractMonthFlag::CentralContractMonth),
        }];

        let expected_response = FuturesPricesResponse {
            futures: expected_futures,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_futures_prices_response_with_missing_optional_fields() {
        let json_data = r#"
        {
            "futures": [
                {
                    "Code": "169090005",
                    "DerivativesProductCategory": "TOPIXF",
                    "Date": "2024-07-23", 
                    "WholeDayOpen": 2825.5, 
                    "WholeDayHigh": 2853.0, 
                    "WholeDayLow": 2825.5, 
                    "WholeDayClose": 2829.0, 
                    "MorningSessionOpen": "", 
                    "MorningSessionHigh": "", 
                    "MorningSessionLow": "", 
                    "MorningSessionClose": "", 
                    "NightSessionOpen": "",
                    "NightSessionHigh": "", 
                    "NightSessionLow": "", 
                    "NightSessionClose": "",
                    "DaySessionOpen": 2850.5, 
                    "DaySessionHigh": 2853.0, 
                    "DaySessionLow": 2826.0, 
                    "DaySessionClose": 2829.0,
                    "Volume": 42910.0,
                    "OpenInterest": 479812.0,
                    "TurnoverValue": 1217918971856.0,
                    "ContractMonth": "2024-09",
                    "Volume(OnlyAuction)": "",
                    "EmergencyMarginTriggerDivision": "002",
                    "LastTradingDay": "",
                    "SpecialQuotationDay": "",
                    "SettlementPrice": "",
                    "CentralContractMonthFlag": ""
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: FuturesPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_futures = vec![FuturesPricesItem {
            code: "169090005".to_string(),
            derivatives_product_category: "TOPIXF".to_string(),
            date: "2024-07-23".to_string(),
            whole_day_open: 2825.5,
            whole_day_high: 2853.0,
            whole_day_low: 2825.5,
            whole_day_close: 2829.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: None,
            night_session_high: None,
            night_session_low: None,
            night_session_close: None,
            day_session_open: 2850.5,
            day_session_high: 2853.0,
            day_session_low: 2826.0,
            day_session_close: 2829.0,
            volume: 42910.0,
            open_interest: 479812.0,
            turnover_value: 1217918971856.0,
            contract_month: "2024-09".to_string(),
            volume_only_auction: None,
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
            last_trading_day: None,
            special_quotation_day: None,
            settlement_price: None,
            central_contract_month_flag: None,
        }];

        let expected_response = FuturesPricesResponse {
            futures: expected_futures,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_futures_prices_response_multiple_items() {
        let json_data = r#"
        {
            "futures": [
                {
                    "Code": "169090005",
                    "DerivativesProductCategory": "TOPIXF",
                    "Date": "2024-07-23", 
                    "WholeDayOpen": 2825.5, 
                    "WholeDayHigh": 2853.0, 
                    "WholeDayLow": 2825.5, 
                    "WholeDayClose": 2829.0, 
                    "MorningSessionOpen": "", 
                    "MorningSessionHigh": "", 
                    "MorningSessionLow": "", 
                    "MorningSessionClose": "", 
                    "NightSessionOpen": 2825.5, 
                    "NightSessionHigh": 2850.0, 
                    "NightSessionLow": 2825.5, 
                    "NightSessionClose": 2845.0, 
                    "DaySessionOpen": 2850.5, 
                    "DaySessionHigh": 2853.0, 
                    "DaySessionLow": 2826.0, 
                    "DaySessionClose": 2829.0, 
                    "Volume": 42910.0, 
                    "OpenInterest": 479812.0, 
                    "TurnoverValue": 1217918971856.0, 
                    "ContractMonth": "2024-09", 
                    "Volume(OnlyAuction)": 40405.0, 
                    "EmergencyMarginTriggerDivision": "002", 
                    "LastTradingDay": "2024-09-12", 
                    "SpecialQuotationDay": "2024-09-13", 
                    "SettlementPrice": 2829.0, 
                    "CentralContractMonthFlag": "1"
                },
                {
                    "Code": "169090006",
                    "DerivativesProductCategory": "NK225F",
                    "Date": "2024-07-24",
                    "WholeDayOpen": 3000.0,
                    "WholeDayHigh": 3050.0,
                    "WholeDayLow": 2950.0,
                    "WholeDayClose": 3025.0,
                    "MorningSessionOpen": 3010.0,
                    "MorningSessionHigh": 3040.0,
                    "MorningSessionLow": 2955.0,
                    "MorningSessionClose": 3030.0,
                    "NightSessionOpen": 3025.5,
                    "NightSessionHigh": 3050.0,
                    "NightSessionLow": 3000.0,
                    "NightSessionClose": 3045.0,
                    "DaySessionOpen": 3050.5,
                    "DaySessionHigh": 3053.0,
                    "DaySessionLow": 3006.0,
                    "DaySessionClose": 3029.0,
                    "Volume": 52910.0,
                    "OpenInterest": 579812.0,
                    "TurnoverValue": 1317918971856.0,
                    "ContractMonth": "2024-10",
                    "Volume(OnlyAuction)": 50405.0,
                    "EmergencyMarginTriggerDivision": "001",
                    "LastTradingDay": "2024-10-12",
                    "SpecialQuotationDay": "2024-10-13",
                    "SettlementPrice": 3029.0,
                    "CentralContractMonthFlag": "0"
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: FuturesPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_futures = vec![
            FuturesPricesItem {
                code: "169090005".to_string(),
                derivatives_product_category: "TOPIXF".to_string(),
                date: "2024-07-23".to_string(),
                whole_day_open: 2825.5,
                whole_day_high: 2853.0,
                whole_day_low: 2825.5,
                whole_day_close: 2829.0,
                morning_session_open: None,
                morning_session_high: None,
                morning_session_low: None,
                morning_session_close: None,
                night_session_open: Some(2825.5),
                night_session_high: Some(2850.0),
                night_session_low: Some(2825.5),
                night_session_close: Some(2845.0),
                day_session_open: 2850.5,
                day_session_high: 2853.0,
                day_session_low: 2826.0,
                day_session_close: 2829.0,
                volume: 42910.0,
                open_interest: 479812.0,
                turnover_value: 1217918971856.0,
                contract_month: "2024-09".to_string(),
                volume_only_auction: Some(40405.0),
                emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
                last_trading_day: Some("2024-09-12".to_string()),
                special_quotation_day: Some("2024-09-13".to_string()),
                settlement_price: Some(2829.0),
                central_contract_month_flag: Some(CentralContractMonthFlag::CentralContractMonth),
            },
            FuturesPricesItem {
                code: "169090006".to_string(),
                derivatives_product_category: "NK225F".to_string(),
                date: "2024-07-24".to_string(),
                whole_day_open: 3000.0,
                whole_day_high: 3050.0,
                whole_day_low: 2950.0,
                whole_day_close: 3025.0,
                morning_session_open: Some(3010.0),
                morning_session_high: Some(3040.0),
                morning_session_low: Some(2955.0),
                morning_session_close: Some(3030.0),
                night_session_open: Some(3025.5),
                night_session_high: Some(3050.0),
                night_session_low: Some(3000.0),
                night_session_close: Some(3045.0),
                day_session_open: 3050.5,
                day_session_high: 3053.0,
                day_session_low: 3006.0,
                day_session_close: 3029.0,
                volume: 52910.0,
                open_interest: 579812.0,
                turnover_value: 1317918971856.0,
                contract_month: "2024-10".to_string(),
                volume_only_auction: Some(50405.0),
                emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Triggered,
                last_trading_day: Some("2024-10-12".to_string()),
                special_quotation_day: Some("2024-10-13".to_string()),
                settlement_price: Some(3029.0),
                central_contract_month_flag: Some(CentralContractMonthFlag::Others),
            },
        ];

        let expected_response = FuturesPricesResponse {
            futures: expected_futures,
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_futures_prices_response_no_pagination_key() {
        let json_data = r#"
        {
            "futures": [
                {
                    "Code": "169090005",
                    "DerivativesProductCategory": "TOPIXF",
                    "Date": "2024-07-23", 
                    "WholeDayOpen": 2825.5, 
                    "WholeDayHigh": 2853.0, 
                    "WholeDayLow": 2825.5, 
                    "WholeDayClose": 2829.0, 
                    "MorningSessionOpen": "", 
                    "MorningSessionHigh": "", 
                    "MorningSessionLow": "", 
                    "MorningSessionClose": "", 
                    "NightSessionOpen": 2825.5, 
                    "NightSessionHigh": 2850.0, 
                    "NightSessionLow": 2825.5, 
                    "NightSessionClose": 2845.0, 
                    "DaySessionOpen": 2850.5, 
                    "DaySessionHigh": 2853.0, 
                    "DaySessionLow": 2826.0, 
                    "DaySessionClose": 2829.0, 
                    "Volume": 42910.0, 
                    "OpenInterest": 479812.0, 
                    "TurnoverValue": 1217918971856.0, 
                    "ContractMonth": "2024-09", 
                    "Volume(OnlyAuction)": 40405.0, 
                    "EmergencyMarginTriggerDivision": "002", 
                    "LastTradingDay": "2024-09-12", 
                    "SpecialQuotationDay": "2024-09-13", 
                    "SettlementPrice": 2829.0, 
                    "CentralContractMonthFlag": "1"
                }
            ]
        }
        "#;

        let response: FuturesPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_futures = vec![FuturesPricesItem {
            code: "169090005".to_string(),
            derivatives_product_category: "TOPIXF".to_string(),
            date: "2024-07-23".to_string(),
            whole_day_open: 2825.5,
            whole_day_high: 2853.0,
            whole_day_low: 2825.5,
            whole_day_close: 2829.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: Some(2825.5),
            night_session_high: Some(2850.0),
            night_session_low: Some(2825.5),
            night_session_close: Some(2845.0),
            day_session_open: 2850.5,
            day_session_high: 2853.0,
            day_session_low: 2826.0,
            day_session_close: 2829.0,
            volume: 42910.0,
            open_interest: 479812.0,
            turnover_value: 1217918971856.0,
            contract_month: "2024-09".to_string(),
            volume_only_auction: Some(40405.0),
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
            last_trading_day: Some("2024-09-12".to_string()),
            special_quotation_day: Some("2024-09-13".to_string()),
            settlement_price: Some(2829.0),
            central_contract_month_flag: Some(CentralContractMonthFlag::CentralContractMonth),
        }];

        let expected_response = FuturesPricesResponse {
            futures: expected_futures,
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_futures_prices_response_no_data() {
        let json_data = r#"
        {
            "futures": []
        }
        "#;

        let response: FuturesPricesResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = FuturesPricesResponse {
            futures: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
