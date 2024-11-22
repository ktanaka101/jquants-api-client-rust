//! Options OHLC (/derivatives/options) API

use serde::{Deserialize, Serialize};

use super::{
    shared::{
        deserialize_utils::{deserialize_f64_or_none, empty_string_or_null_as_none},
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::{
            central_contract_month_flag::CentralContractMonthFlag,
            emergency_margin_trigger_division::EmergencyMarginTriggerDivision,
            options_code::OptionsCode, put_call_division::PutCallDivision,
            underlying_sso::UnderlyingSSO,
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Options (OHLC) Data API.
#[derive(Clone, Serialize)]
pub struct OptionsPricesBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Category of data
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,

    /// Security options code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<OptionsCode>,

    /// Date of data (e.g., "20210901" or "2021-09-01")
    date: String,

    /// Central contract month flag (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    contract_flag: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<OptionsPricesResponse> for OptionsPricesBuilder {
    async fn send(self) -> Result<OptionsPricesResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<OptionsPricesResponse, crate::JQuantsError> {
        self.client.inner.get("derivatives/options", self).await
    }
}

impl Paginatable<OptionsPricesResponse> for OptionsPricesBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl OptionsPricesBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient, date: String) -> Self {
        Self {
            client,
            category: None,
            code: None,
            date,
            contract_flag: None,
            pagination_key: None,
        }
    }

    /// Set the category of data.
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set the security options code.
    pub fn code(mut self, code: impl Into<OptionsCode>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set the date of data (e.g., "20210901" or "2021-09-01")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    /// Set the central contract month flag.
    pub fn contract_flag(mut self, flag: impl Into<String>) -> Self {
        self.contract_flag = Some(flag.into());
        self
    }

    /// Set pagination key for fetching the next set of data.
    pub fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

/// Trait for Options (OHLC) Data API.
pub trait OptionsPricesApi: JQuantsPlanClient {
    /// Get API builder for Options (OHLC) Data.
    ///
    /// Use [Options (OHLC) (/derivatives/options) API](https://jpx.gitbook.io/j-quants-en/api-reference/options)
    fn get_options_prices(&self, date: impl Into<String>) -> OptionsPricesBuilder {
        OptionsPricesBuilder::new(self.get_api_client().clone(), date.into())
    }
}

/// Options (OHLC) Data API response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/options)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OptionsPricesResponse {
    /// List of Options prices
    pub options: Vec<OptionsPricesItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for OptionsPricesResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for OptionsPricesResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.options.extend(p.options);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single Options price record.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OptionsPricesItem {
    /// Issue code
    #[serde(rename = "Code")]
    pub code: String,

    /// Derivative Product Category
    #[serde(rename = "DerivativesProductCategory")]
    pub derivatives_product_category: String,

    /// Underlying SSO
    #[serde(rename = "UnderlyingSSO")]
    pub underlying_sso: UnderlyingSSO,

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
    #[serde(
        rename = "DaySessionOpen",
        deserialize_with = "deserialize_f64_or_none"
    )]
    pub day_session_open: Option<f64>,

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
    #[serde(rename = "EmergencyMarginTriggerDivision")]
    pub emergency_margin_trigger_division: EmergencyMarginTriggerDivision,

    /// Put Call division (1: put, 2: call)
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

    /// Flag of the central contract month
    #[serde(
        rename = "CentralContractMonthFlag",
        deserialize_with = "empty_string_or_null_as_none"
    )]
    pub central_contract_month_flag: Option<CentralContractMonthFlag>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_options_prices_response() {
        let json_data = r#"
        {
            "options": [
                {
                    "Code": "140014505", 
                    "DerivativesProductCategory": "TOPIXE", 
                    "UnderlyingSSO": "-", 
                    "Date": "2024-07-23", 
                    "WholeDayOpen": 0.0, 
                    "WholeDayHigh": 0.0, 
                    "WholeDayLow": 0.0, 
                    "WholeDayClose": 0.0, 
                    "MorningSessionOpen": "", 
                    "MorningSessionHigh": "", 
                    "MorningSessionLow": "", 
                    "MorningSessionClose": "", 
                    "NightSessionOpen": 0.0, 
                    "NightSessionHigh": 0.0, 
                    "NightSessionLow": 0.0, 
                    "NightSessionClose": 0.0, 
                    "DaySessionOpen": 0.0, 
                    "DaySessionHigh": 0.0, 
                    "DaySessionLow": 0.0, 
                    "DaySessionClose": 0.0, 
                    "Volume": 0.0, 
                    "OpenInterest": 0.0, 
                    "TurnoverValue": 0.0, 
                    "ContractMonth": "2025-01", 
                    "StrikePrice": 2450.0, 
                    "Volume(OnlyAuction)": 0.0, 
                    "EmergencyMarginTriggerDivision": "002", 
                    "PutCallDivision": "2", 
                    "LastTradingDay": "2025-01-09", 
                    "SpecialQuotationDay": "2025-01-10", 
                    "SettlementPrice": 377.0, 
                    "TheoreticalPrice": 380.3801, 
                    "BaseVolatility": 18.115, 
                    "UnderlyingPrice": 2833.39, 
                    "ImpliedVolatility": 17.2955, 
                    "InterestRate": 0.3527, 
                    "CentralContractMonthFlag": "0"
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: OptionsPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_option = vec![OptionsPricesItem {
            code: "140014505".to_string(),
            derivatives_product_category: "TOPIXE".to_string(),
            underlying_sso: UnderlyingSSO::Other,
            date: "2024-07-23".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: Some(0.0),
            night_session_high: Some(0.0),
            night_session_low: Some(0.0),
            night_session_close: Some(0.0),
            day_session_open: Some(0.0),
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 0.0,
            turnover_value: 0.0,
            contract_month: "2025-01".to_string(),
            strike_price: 2450.0,
            volume_only_auction: Some(0.0),
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
            put_call_division: PutCallDivision::Call,
            last_trading_day: Some("2025-01-09".to_string()),
            special_quotation_day: Some("2025-01-10".to_string()),
            settlement_price: Some(377.0),
            theoretical_price: Some(380.3801),
            base_volatility: Some(18.115),
            underlying_price: Some(2833.39),
            implied_volatility: Some(17.2955),
            interest_rate: Some(0.3527),
            central_contract_month_flag: Some(CentralContractMonthFlag::Others),
        }];

        let expected_response = OptionsPricesResponse {
            options: expected_option,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_options_prices_response_with_missing_optional_fields() {
        let json_data = r#"
        {
            "options": [
                {
                    "Code": "140014505",
                    "DerivativesProductCategory": "TOPIXE",
                    "UnderlyingSSO": "-",
                    "Date": "2024-07-23",
                    "WholeDayOpen": 0.0,
                    "WholeDayHigh": 0.0,
                    "WholeDayLow": 0.0,
                    "WholeDayClose": 0.0,
                    "MorningSessionOpen": "",
                    "MorningSessionHigh": "",
                    "MorningSessionLow": "",
                    "MorningSessionClose": "",
                    "NightSessionOpen": "",
                    "NightSessionHigh": "",
                    "NightSessionLow": "",
                    "NightSessionClose": "",
                    "DaySessionOpen": "",
                    "DaySessionHigh": 0.0,
                    "DaySessionLow": 0.0,
                    "DaySessionClose": 0.0,
                    "Volume": 0.0,
                    "OpenInterest": 0.0,
                    "TurnoverValue": 0.0,
                    "ContractMonth": "2025-01",
                    "StrikePrice": 2450.0,
                    "Volume(OnlyAuction)": "",
                    "EmergencyMarginTriggerDivision": "001",
                    "PutCallDivision": "2",
                    "LastTradingDay": "2025-01-09",
                    "SpecialQuotationDay": "2025-01-10",
                    "SettlementPrice": "",
                    "TheoreticalPrice": "",
                    "BaseVolatility": "",
                    "UnderlyingPrice": "",
                    "ImpliedVolatility": "",
                    "InterestRate": "",
                    "CentralContractMonthFlag": ""
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: OptionsPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_option = vec![OptionsPricesItem {
            code: "140014505".to_string(),
            derivatives_product_category: "TOPIXE".to_string(),
            underlying_sso: UnderlyingSSO::Other,
            date: "2024-07-23".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: None,
            night_session_high: None,
            night_session_low: None,
            night_session_close: None,
            day_session_open: None,
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 0.0,
            turnover_value: 0.0,
            contract_month: "2025-01".to_string(),
            strike_price: 2450.0,
            volume_only_auction: None,
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Triggered,
            put_call_division: PutCallDivision::Call,
            last_trading_day: Some("2025-01-09".to_string()),
            special_quotation_day: Some("2025-01-10".to_string()),
            settlement_price: None,
            theoretical_price: None,
            base_volatility: None,
            underlying_price: None,
            implied_volatility: None,
            interest_rate: None,
            central_contract_month_flag: None,
        }];

        let expected_response = OptionsPricesResponse {
            options: expected_option,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_options_prices_response_multiple_items() {
        let json_data = r#"
        {
            "options": [
                {
                    "Code": "140014505",
                    "DerivativesProductCategory": "TOPIXE",
                    "UnderlyingSSO": "-",
                    "Date": "2024-07-23",
                    "WholeDayOpen": 1000.0,
                    "WholeDayHigh": 1050.0,
                    "WholeDayLow": 990.0,
                    "WholeDayClose": 1025.0,
                    "MorningSessionOpen": "1005.0",
                    "MorningSessionHigh": "1045.0",
                    "MorningSessionLow": "995.0",
                    "MorningSessionClose": "1020.0",
                    "NightSessionOpen": 1010.0,
                    "NightSessionHigh": 1040.0,
                    "NightSessionLow": 995.0,
                    "NightSessionClose": 1030.0,
                    "DaySessionOpen": 1025.0,
                    "DaySessionHigh": 1060.0,
                    "DaySessionLow": 1000.0,
                    "DaySessionClose": 1045.0,
                    "Volume": 1500.0,
                    "OpenInterest": 300.0,
                    "TurnoverValue": 1500000.0,
                    "ContractMonth": "2025-02",
                    "StrikePrice": 2500.0,
                    "Volume(OnlyAuction)": 500.0,
                    "EmergencyMarginTriggerDivision": "001",
                    "PutCallDivision": "1",
                    "LastTradingDay": "2025-02-09",
                    "SpecialQuotationDay": "2025-02-10",
                    "SettlementPrice": 1025.0,
                    "TheoreticalPrice": 1030.5001,
                    "BaseVolatility": 19.200,
                    "UnderlyingPrice": 2850.00,
                    "ImpliedVolatility": 18.5000,
                    "InterestRate": 0.3600,
                    "CentralContractMonthFlag": "1"
                },
                {
                    "Code": "140014506",
                    "DerivativesProductCategory": "TOPIXE",
                    "UnderlyingSSO": "-",
                    "Date": "2024-07-23",
                    "WholeDayOpen": 2000.0,
                    "WholeDayHigh": 2050.0,
                    "WholeDayLow": 1990.0,
                    "WholeDayClose": 2025.0,
                    "MorningSessionOpen": "2005.0",
                    "MorningSessionHigh": "2045.0",
                    "MorningSessionLow": "1995.0",
                    "MorningSessionClose": "2020.0",
                    "NightSessionOpen": 2010.0,
                    "NightSessionHigh": 2040.0,
                    "NightSessionLow": 1995.0,
                    "NightSessionClose": 2030.0,
                    "DaySessionOpen": 2025.0,
                    "DaySessionHigh": 2060.0,
                    "DaySessionLow": 2000.0,
                    "DaySessionClose": 2045.0,
                    "Volume": 2500.0,
                    "OpenInterest": 400.0,
                    "TurnoverValue": 2500000.0,
                    "ContractMonth": "2025-03",
                    "StrikePrice": 2550.0,
                    "Volume(OnlyAuction)": 600.0,
                    "EmergencyMarginTriggerDivision": "002",
                    "PutCallDivision": "2",
                    "LastTradingDay": "2025-03-09",
                    "SpecialQuotationDay": "2025-03-10",
                    "SettlementPrice": 2025.0,
                    "TheoreticalPrice": 2030.5001,
                    "BaseVolatility": 19.500,
                    "UnderlyingPrice": 2855.00,
                    "ImpliedVolatility": 18.7000,
                    "InterestRate": 0.3650,
                    "CentralContractMonthFlag": "1"
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: OptionsPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_options = vec![
            OptionsPricesItem {
                code: "140014505".to_string(),
                derivatives_product_category: "TOPIXE".to_string(),
                underlying_sso: UnderlyingSSO::Other,
                date: "2024-07-23".to_string(),
                whole_day_open: 1000.0,
                whole_day_high: 1050.0,
                whole_day_low: 990.0,
                whole_day_close: 1025.0,
                morning_session_open: Some(1005.0),
                morning_session_high: Some(1045.0),
                morning_session_low: Some(995.0),
                morning_session_close: Some(1020.0),
                night_session_open: Some(1010.0),
                night_session_high: Some(1040.0),
                night_session_low: Some(995.0),
                night_session_close: Some(1030.0),
                day_session_open: Some(1025.0),
                day_session_high: 1060.0,
                day_session_low: 1000.0,
                day_session_close: 1045.0,
                volume: 1500.0,
                open_interest: 300.0,
                turnover_value: 1500000.0,
                contract_month: "2025-02".to_string(),
                strike_price: 2500.0,
                volume_only_auction: Some(500.0),
                emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Triggered,
                put_call_division: PutCallDivision::Put,
                last_trading_day: Some("2025-02-09".to_string()),
                special_quotation_day: Some("2025-02-10".to_string()),
                settlement_price: Some(1025.0),
                theoretical_price: Some(1030.5001),
                base_volatility: Some(19.200),
                underlying_price: Some(2850.00),
                implied_volatility: Some(18.5000),
                interest_rate: Some(0.3600),
                central_contract_month_flag: Some(CentralContractMonthFlag::CentralContractMonth),
            },
            OptionsPricesItem {
                code: "140014506".to_string(),
                derivatives_product_category: "TOPIXE".to_string(),
                underlying_sso: UnderlyingSSO::Other,
                date: "2024-07-23".to_string(),
                whole_day_open: 2000.0,
                whole_day_high: 2050.0,
                whole_day_low: 1990.0,
                whole_day_close: 2025.0,
                morning_session_open: Some(2005.0),
                morning_session_high: Some(2045.0),
                morning_session_low: Some(1995.0),
                morning_session_close: Some(2020.0),
                night_session_open: Some(2010.0),
                night_session_high: Some(2040.0),
                night_session_low: Some(1995.0),
                night_session_close: Some(2030.0),
                day_session_open: Some(2025.0),
                day_session_high: 2060.0,
                day_session_low: 2000.0,
                day_session_close: 2045.0,
                volume: 2500.0,
                open_interest: 400.0,
                turnover_value: 2500000.0,
                contract_month: "2025-03".to_string(),
                strike_price: 2550.0,
                volume_only_auction: Some(600.0),
                emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
                put_call_division: PutCallDivision::Call,
                last_trading_day: Some("2025-03-09".to_string()),
                special_quotation_day: Some("2025-03-10".to_string()),
                settlement_price: Some(2025.0),
                theoretical_price: Some(2030.5001),
                base_volatility: Some(19.500),
                underlying_price: Some(2855.00),
                implied_volatility: Some(18.7000),
                interest_rate: Some(0.3650),
                central_contract_month_flag: Some(CentralContractMonthFlag::CentralContractMonth),
            },
        ];

        let expected_response = OptionsPricesResponse {
            options: expected_options,
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_options_prices_response_no_pagination_key() {
        let json_data = r#"
        {
            "options": [
                {
                    "Code": "140014505",
                    "DerivativesProductCategory": "TOPIXE",
                    "UnderlyingSSO": "-",
                    "Date": "2024-07-23",
                    "WholeDayOpen": 0.0,
                    "WholeDayHigh": 0.0,
                    "WholeDayLow": 0.0,
                    "WholeDayClose": 0.0,
                    "MorningSessionOpen": "",
                    "MorningSessionHigh": "",
                    "MorningSessionLow": "",
                    "MorningSessionClose": "",
                    "NightSessionOpen": 0.0,
                    "NightSessionHigh": 0.0,
                    "NightSessionLow": 0.0,
                    "NightSessionClose": 0.0,
                    "DaySessionOpen": 0.0,
                    "DaySessionHigh": 0.0,
                    "DaySessionLow": 0.0,
                    "DaySessionClose": 0.0,
                    "Volume": 0.0,
                    "OpenInterest": 0.0,
                    "TurnoverValue": 0.0,
                    "ContractMonth": "2025-01",
                    "StrikePrice": 2450.0,
                    "Volume(OnlyAuction)": 0.0,
                    "EmergencyMarginTriggerDivision": "002",
                    "PutCallDivision": "2",
                    "LastTradingDay": "2025-01-09",
                    "SpecialQuotationDay": "2025-01-10",
                    "SettlementPrice": 377.0,
                    "TheoreticalPrice": 380.3801,
                    "BaseVolatility": 18.115,
                    "UnderlyingPrice": 2833.39,
                    "ImpliedVolatility": 17.2955,
                    "InterestRate": 0.3527,
                    "CentralContractMonthFlag": "0"
                }
            ]
        }
        "#;

        let response: OptionsPricesResponse = serde_json::from_str(json_data).unwrap();

        let expected_option = vec![OptionsPricesItem {
            code: "140014505".to_string(),
            derivatives_product_category: "TOPIXE".to_string(),
            underlying_sso: UnderlyingSSO::Other,
            date: "2024-07-23".to_string(),
            whole_day_open: 0.0,
            whole_day_high: 0.0,
            whole_day_low: 0.0,
            whole_day_close: 0.0,
            morning_session_open: None,
            morning_session_high: None,
            morning_session_low: None,
            morning_session_close: None,
            night_session_open: Some(0.0),
            night_session_high: Some(0.0),
            night_session_low: Some(0.0),
            night_session_close: Some(0.0),
            day_session_open: Some(0.0),
            day_session_high: 0.0,
            day_session_low: 0.0,
            day_session_close: 0.0,
            volume: 0.0,
            open_interest: 0.0,
            turnover_value: 0.0,
            contract_month: "2025-01".to_string(),
            strike_price: 2450.0,
            volume_only_auction: Some(0.0),
            emergency_margin_trigger_division: EmergencyMarginTriggerDivision::Calculated,
            put_call_division: PutCallDivision::Call,
            last_trading_day: Some("2025-01-09".to_string()),
            special_quotation_day: Some("2025-01-10".to_string()),
            settlement_price: Some(377.0),
            theoretical_price: Some(380.3801),
            base_volatility: Some(18.115),
            underlying_price: Some(2833.39),
            implied_volatility: Some(17.2955),
            interest_rate: Some(0.3527),
            central_contract_month_flag: Some(CentralContractMonthFlag::Others),
        }];

        let expected_response = OptionsPricesResponse {
            options: expected_option,
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_options_prices_response_no_data() {
        let json_data = r#"
        {
            "options": []
        }
        "#;

        let response: OptionsPricesResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = OptionsPricesResponse {
            options: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
