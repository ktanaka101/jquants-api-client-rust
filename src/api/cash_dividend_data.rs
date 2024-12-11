//! Cash Dividend Data (/fins/dividend) API.

use serde::{Deserialize, Serialize};

use super::{
    shared::{
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::{
            amount_per_share::AmountPerShare,
            dividend::{
                DevidendStatucCode, DividendCommemorativeSpecialCode, DividendForecastResultCode,
                DividendInterimFinalCode,
            },
            payable_date::PayableDate,
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Cash Dividend Data API.
#[derive(Clone, Serialize)]
pub struct CashDividendDataBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Issue code (e.g., "27800" or "2780")
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,

    /// Disclosure date (e.g., "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Starting point of data period (e.g., "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,

    /// End point of data period (e.g., "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<CashDividendDataResponse> for CashDividendDataBuilder {
    async fn send(self) -> Result<CashDividendDataResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<CashDividendDataResponse, crate::JQuantsError> {
        self.client.inner.get("fins/dividend", self).await
    }
}

impl Paginatable<CashDividendDataResponse> for CashDividendDataBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl CashDividendDataBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            code: None,
            date: None,
            from: None,
            to: None,
            pagination_key: None,
        }
    }

    /// Set issue code (e.g., "27800" or "2780")
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set disclosure date (e.g., "20210901" or "2021-09-01")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
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

/// Trait for Cash Dividend Data API.
pub trait CashDividendDataApi: JQuantsPlanClient {
    /// Get API builder for Cash Dividend Data.
    ///
    /// Use [Cash Dividend Data (/fins/dividend) API](https://jpx.gitbook.io/j-quants-en/api-reference/dividend)
    fn get_cash_dividend_data(&self) -> CashDividendDataBuilder {
        CashDividendDataBuilder::new(self.get_api_client().clone())
    }
}

/// Cash Dividend Data API response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/dividend)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CashDividendDataResponse {
    /// List of cash dividend data
    pub dividend: Vec<CashDividendItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for CashDividendDataResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for CashDividendDataResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.dividend.extend(p.dividend);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single cash dividend data item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CashDividendItem {
    /// Announcement Date (YYYY-MM-DD)
    #[serde(rename = "AnnouncementDate")]
    pub announcement_date: String,

    /// Announcement Time (HH:MM)
    #[serde(rename = "AnnouncementTime")]
    pub announcement_time: String,

    /// Issue Code (5-character)
    #[serde(rename = "Code")]
    pub code: String,

    /// Reference Number
    #[serde(rename = "ReferenceNumber")]
    pub reference_number: String,

    /// Status Code
    #[serde(rename = "StatusCode")]
    pub status_code: DevidendStatucCode,

    /// Board Meeting Date (YYYY-MM-DD)
    #[serde(rename = "BoardMeetingDate")]
    pub board_meeting_date: String,

    /// Interim/Final Code
    #[serde(rename = "InterimFinalCode")]
    pub interim_final_code: DividendInterimFinalCode,

    /// Forecast/Result Code
    #[serde(rename = "ForecastResultCode")]
    pub forecast_result_code: DividendForecastResultCode,

    /// Interim Final Term
    #[serde(rename = "InterimFinalTerm")]
    pub interim_final_term: String,

    /// Gross Dividend Rate
    #[serde(rename = "GrossDividendRate")]
    pub gross_dividend_rate: AmountPerShare,

    /// Record Date (YYYY-MM-DD)
    #[serde(rename = "RecordDate")]
    pub record_date: String,

    /// Ex-Rights Date (YYYY-MM-DD)
    #[serde(rename = "ExDate")]
    pub ex_date: String,

    /// Actual Record Date (YYYY-MM-DD)
    #[serde(rename = "ActualRecordDate")]
    pub actual_record_date: String,

    /// Payable Date (YYYY-MM-DD)
    #[serde(rename = "PayableDate")]
    pub payable_date: PayableDate,

    /// CA Reference Number
    #[serde(rename = "CAReferenceNumber")]
    pub ca_reference_number: String,

    /// Distribution Amount per Share
    #[serde(rename = "DistributionAmount")]
    pub distribution_amount: AmountPerShare,

    /// Retained Earnings per Share
    #[serde(rename = "RetainedEarnings")]
    pub retained_earnings: AmountPerShare,

    /// Deemed Dividend per Share
    #[serde(rename = "DeemedDividend")]
    pub deemed_dividend: AmountPerShare,

    /// Deemed Capital Gains per Share
    #[serde(rename = "DeemedCapitalGains")]
    pub deemed_capital_gains: AmountPerShare,

    /// Net Asset Decrease Ratio
    #[serde(rename = "NetAssetDecreaseRatio")]
    pub net_asset_decrease_ratio: AmountPerShare,

    /// Commemorative/Special Code
    #[serde(rename = "CommemorativeSpecialCode")]
    pub commemorative_special_code: DividendCommemorativeSpecialCode,

    /// Commemorative Dividend Rate per Share
    #[serde(rename = "CommemorativeDividendRate")]
    pub commemorative_dividend_rate: AmountPerShare,

    /// Special Dividend Rate per Share
    #[serde(rename = "SpecialDividendRate")]
    pub special_dividend_rate: AmountPerShare,
}

#[cfg(feature = "polars")]
impl CashDividendDataResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_column;
        use polars::prelude::*;

        let data = self.dividend;

        let mut announcement_date = Vec::with_capacity(data.len());
        let mut announcement_time = Vec::with_capacity(data.len());
        let mut code = Vec::with_capacity(data.len());
        let mut reference_number = Vec::with_capacity(data.len());
        let mut status_code = Vec::with_capacity(data.len());
        let mut board_meeting_date = Vec::with_capacity(data.len());
        let mut interim_final_code = Vec::with_capacity(data.len());
        let mut forecast_result_code = Vec::with_capacity(data.len());
        let mut interim_final_term = Vec::with_capacity(data.len());
        let mut gross_dividend_rate_variant = Vec::with_capacity(data.len());
        let mut gross_dividend_rate = Vec::with_capacity(data.len());
        let mut record_date = Vec::with_capacity(data.len());
        let mut ex_date = Vec::with_capacity(data.len());
        let mut actual_record_date = Vec::with_capacity(data.len());
        let mut payable_date_variant = Vec::with_capacity(data.len());
        let mut payable_date = Vec::with_capacity(data.len());
        let mut ca_reference_number = Vec::with_capacity(data.len());
        let mut distribution_amount_variant = Vec::with_capacity(data.len());
        let mut distribution_amount = Vec::with_capacity(data.len());
        let mut retained_earnings_variant = Vec::with_capacity(data.len());
        let mut retained_earnings = Vec::with_capacity(data.len());
        let mut deemed_dividend_variant = Vec::with_capacity(data.len());
        let mut deemed_dividend = Vec::with_capacity(data.len());
        let mut deemed_capital_gains_variant = Vec::with_capacity(data.len());
        let mut deemed_capital_gains = Vec::with_capacity(data.len());
        let mut net_asset_decrease_ratio_variant = Vec::with_capacity(data.len());
        let mut net_asset_decrease_ratio = Vec::with_capacity(data.len());
        let mut commemorative_special_code = Vec::with_capacity(data.len());
        let mut commemorative_dividend_rate_variant = Vec::with_capacity(data.len());
        let mut commemorative_dividend_rate = Vec::with_capacity(data.len());
        let mut special_dividend_rate_variant = Vec::with_capacity(data.len());
        let mut special_dividend_rate = Vec::with_capacity(data.len());

        for item in data {
            announcement_date.push(item.announcement_date);
            announcement_time.push(item.announcement_time);
            code.push(item.code);
            reference_number.push(item.reference_number);
            status_code.push(item.status_code);
            board_meeting_date.push(item.board_meeting_date);
            interim_final_code.push(item.interim_final_code);
            forecast_result_code.push(item.forecast_result_code);
            interim_final_term.push(item.interim_final_term);
            gross_dividend_rate_variant.push(item.gross_dividend_rate.variant());
            gross_dividend_rate.push(item.gross_dividend_rate.into_number());
            record_date.push(item.record_date);
            ex_date.push(item.ex_date);
            actual_record_date.push(item.actual_record_date);
            payable_date_variant.push(item.payable_date.variant());
            payable_date.push(item.payable_date.into_date());
            ca_reference_number.push(item.ca_reference_number);
            distribution_amount_variant.push(item.distribution_amount.variant());
            distribution_amount.push(item.distribution_amount.into_number());
            retained_earnings_variant.push(item.retained_earnings.variant());
            retained_earnings.push(item.retained_earnings.into_number());
            deemed_dividend_variant.push(item.deemed_dividend.variant());
            deemed_dividend.push(item.deemed_dividend.into_number());
            deemed_capital_gains_variant.push(item.deemed_capital_gains.variant());
            deemed_capital_gains.push(item.deemed_capital_gains.into_number());
            net_asset_decrease_ratio_variant.push(item.net_asset_decrease_ratio.variant());
            net_asset_decrease_ratio.push(item.net_asset_decrease_ratio.into_number());
            commemorative_special_code.push(item.commemorative_special_code);
            commemorative_dividend_rate_variant.push(item.commemorative_dividend_rate.variant());
            commemorative_dividend_rate.push(item.commemorative_dividend_rate.into_number());
            special_dividend_rate_variant.push(item.special_dividend_rate.variant());
            special_dividend_rate.push(item.special_dividend_rate.into_number());
        }

        let df = polars::frame::DataFrame::new(vec![
            Column::new("AnnouncementDate".into(), announcement_date).cast(&DataType::Date)?,
            Column::new("AnnouncementTime".into(), announcement_time),
            build_column("Code", code)?,
            Column::new("ReferenceNumber".into(), reference_number),
            build_column("StatusCode", status_code)?,
            Column::new("BoardMeetingDate".into(), board_meeting_date).cast(&DataType::Date)?,
            build_column("InterimFinalCode", interim_final_code)?,
            build_column("ForecastResultCode", forecast_result_code)?,
            Column::new("InterimFinalTerm".into(), interim_final_term),
            build_column("GrossDividendRateVariant", gross_dividend_rate_variant)?,
            Column::new("GrossDividendRate".into(), gross_dividend_rate),
            Column::new("RecordDate".into(), record_date).cast(&DataType::Date)?,
            Column::new("ExDate".into(), ex_date).cast(&DataType::Date)?,
            Column::new("ActualRecordDate".into(), actual_record_date).cast(&DataType::Date)?,
            build_column("PayableDateVariant", payable_date_variant)?,
            Column::new("PayableDate".into(), payable_date).cast(&DataType::Date)?,
            Column::new("CAReferenceNumber".into(), ca_reference_number),
            build_column("DistributionAmountVariant", distribution_amount_variant)?,
            Column::new("DistributionAmount".into(), distribution_amount),
            build_column("RetainedEarningsVariant", retained_earnings_variant)?,
            Column::new("RetainedEarnings".into(), retained_earnings),
            build_column("DeemedDividendVariant", deemed_dividend_variant)?,
            Column::new("DeemedDividend".into(), deemed_dividend),
            build_column("DeemedCapitalGainsVariant", deemed_capital_gains_variant)?,
            Column::new("DeemedCapitalGains".into(), deemed_capital_gains),
            build_column(
                "NetAssetDecreaseRatioVariant",
                net_asset_decrease_ratio_variant,
            )?,
            Column::new("NetAssetDecreaseRatio".into(), net_asset_decrease_ratio),
            build_column("CommemorativeSpecialCode", commemorative_special_code)?,
            build_column(
                "CommemorativeDividendRateVariant",
                commemorative_dividend_rate_variant,
            )?,
            Column::new(
                "CommemorativeDividendRate".into(),
                commemorative_dividend_rate,
            ),
            build_column("SpecialDividendRateVariant", special_dividend_rate_variant)?,
            Column::new("SpecialDividendRate".into(), special_dividend_rate),
        ])?;

        let df = df
            .lazy()
            .with_columns(vec![
                col("AnnouncementTime").str().to_time(StrptimeOptions {
                    format: Some("%H:%M".into()),
                    strict: true,
                    exact: true,
                    ..Default::default()
                }),
                col("InterimFinalTerm").str().to_date(StrptimeOptions {
                    format: Some("%Y-%m".into()),
                    strict: true,
                    exact: true,
                    ..Default::default()
                }),
            ])
            .collect()?;

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_cash_dividend_data_response() {
        let json_data = r#"
        {
            "dividend": [
                {
                    "AnnouncementDate": "2014-02-24",
                    "AnnouncementTime": "09:21",
                    "Code": "15550",
                    "ReferenceNumber": "201402241B00002",
                    "StatusCode": "1",
                    "BoardMeetingDate": "2014-02-24",
                    "InterimFinalCode": "2",
                    "ForecastResultCode": "2",
                    "InterimFinalTerm": "2014-03",
                    "GrossDividendRate": "-",
                    "RecordDate": "2014-03-10",
                    "ExDate": "2014-03-06",
                    "ActualRecordDate": "2014-03-10",
                    "PayableDate": "-",
                    "CAReferenceNumber": "201402241B00002",
                    "DistributionAmount": "",
                    "RetainedEarnings": "",
                    "DeemedDividend": "",
                    "DeemedCapitalGains": "",
                    "NetAssetDecreaseRatio": "",
                    "CommemorativeSpecialCode": "0",
                    "CommemorativeDividendRate": "",
                    "SpecialDividendRate": ""
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: CashDividendDataResponse = serde_json::from_str(json_data).unwrap();

        let expected_dividend = vec![CashDividendItem {
            announcement_date: "2014-02-24".to_string(),
            announcement_time: "09:21".to_string(),
            code: "15550".to_string(),
            reference_number: "201402241B00002".to_string(),
            status_code: DevidendStatucCode::New,
            board_meeting_date: "2014-02-24".to_string(),
            interim_final_code: DividendInterimFinalCode::Final,
            forecast_result_code: DividendForecastResultCode::Forecast,
            interim_final_term: "2014-03".to_string(),
            gross_dividend_rate: AmountPerShare::Undetermined,
            record_date: "2014-03-10".to_string(),
            ex_date: "2014-03-06".to_string(),
            actual_record_date: "2014-03-10".to_string(),
            payable_date: PayableDate::Undetermined,
            ca_reference_number: "201402241B00002".to_string(),
            distribution_amount: AmountPerShare::NotApplicable,
            retained_earnings: AmountPerShare::NotApplicable,
            deemed_dividend: AmountPerShare::NotApplicable,
            deemed_capital_gains: AmountPerShare::NotApplicable,
            net_asset_decrease_ratio: AmountPerShare::NotApplicable,
            commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
            commemorative_dividend_rate: AmountPerShare::NotApplicable,
            special_dividend_rate: AmountPerShare::NotApplicable,
        }];

        let expected_response = CashDividendDataResponse {
            dividend: expected_dividend,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_cash_dividend_data_response_no_pagination_key() {
        let json_data = r#"
        {
            "dividend": [
                {
                    "AnnouncementDate": "2014-02-24",
                    "AnnouncementTime": "09:21",
                    "Code": "15550",
                    "ReferenceNumber": "201402241B00002",
                    "StatusCode": "1",
                    "BoardMeetingDate": "2014-02-24",
                    "InterimFinalCode": "2",
                    "ForecastResultCode": "2",
                    "InterimFinalTerm": "2014-03",
                    "GrossDividendRate": "-",
                    "RecordDate": "2014-03-10",
                    "ExDate": "2014-03-06",
                    "ActualRecordDate": "2014-03-10",
                    "PayableDate": "-",
                    "CAReferenceNumber": "201402241B00002",
                    "DistributionAmount": "",
                    "RetainedEarnings": "",
                    "DeemedDividend": "",
                    "DeemedCapitalGains": "",
                    "NetAssetDecreaseRatio": "",
                    "CommemorativeSpecialCode": "0",
                    "CommemorativeDividendRate": "",
                    "SpecialDividendRate": ""
                }
            ]
        }
        "#;

        let response: CashDividendDataResponse = serde_json::from_str(json_data).unwrap();

        let expected_dividend = vec![CashDividendItem {
            announcement_date: "2014-02-24".to_string(),
            announcement_time: "09:21".to_string(),
            code: "15550".to_string(),
            reference_number: "201402241B00002".to_string(),
            status_code: DevidendStatucCode::New,
            board_meeting_date: "2014-02-24".to_string(),
            interim_final_code: DividendInterimFinalCode::Final,
            forecast_result_code: DividendForecastResultCode::Forecast,
            interim_final_term: "2014-03".to_string(),
            gross_dividend_rate: AmountPerShare::Undetermined,
            record_date: "2014-03-10".to_string(),
            ex_date: "2014-03-06".to_string(),
            actual_record_date: "2014-03-10".to_string(),
            payable_date: PayableDate::Undetermined,
            ca_reference_number: "201402241B00002".to_string(),
            distribution_amount: AmountPerShare::NotApplicable,
            retained_earnings: AmountPerShare::NotApplicable,
            deemed_dividend: AmountPerShare::NotApplicable,
            deemed_capital_gains: AmountPerShare::NotApplicable,
            net_asset_decrease_ratio: AmountPerShare::NotApplicable,
            commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
            commemorative_dividend_rate: AmountPerShare::NotApplicable,
            special_dividend_rate: AmountPerShare::NotApplicable,
        }];

        let expected_response = CashDividendDataResponse {
            dividend: expected_dividend,
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_cash_dividend_data_response_multiple_items() {
        let json_data = r#"
        {
            "dividend": [
                {
                    "AnnouncementDate": "2023-03-06",
                    "AnnouncementTime": "10:00",
                    "Code": "86970",
                    "ReferenceNumber": "1",
                    "StatusCode": "1",
                    "BoardMeetingDate": "2023-03-06",
                    "InterimFinalCode": "1",
                    "ForecastResultCode": "1",
                    "InterimFinalTerm": "2023-04",
                    "GrossDividendRate": "100",
                    "RecordDate": "2023-03-10",
                    "ExDate": "2023-03-05",
                    "ActualRecordDate": "2023-03-10",
                    "PayableDate": "2023-03-15",
                    "CAReferenceNumber": "1",
                    "DistributionAmount": "100",
                    "RetainedEarnings": "50",
                    "DeemedDividend": "0",
                    "DeemedCapitalGains": "0",
                    "NetAssetDecreaseRatio": "0.05",
                    "CommemorativeSpecialCode": "0",
                    "CommemorativeDividendRate": "-",
                    "SpecialDividendRate": "-"
                },
                {
                    "AnnouncementDate": "2023-03-07",
                    "AnnouncementTime": "11:00",
                    "Code": "86970",
                    "ReferenceNumber": "2",
                    "StatusCode": "2",
                    "BoardMeetingDate": "2023-03-07",
                    "InterimFinalCode": "2",
                    "ForecastResultCode": "1",
                    "InterimFinalTerm": "2023-04",
                    "GrossDividendRate": "110",
                    "RecordDate": "2023-03-12",
                    "ExDate": "2023-03-07",
                    "ActualRecordDate": "2023-03-12",
                    "PayableDate": "2023-03-17",
                    "CAReferenceNumber": "1",
                    "DistributionAmount": "110",
                    "RetainedEarnings": "55",
                    "DeemedDividend": "0",
                    "DeemedCapitalGains": "0",
                    "NetAssetDecreaseRatio": "0.055",
                    "CommemorativeSpecialCode": "1",
                    "CommemorativeDividendRate": "10",
                    "SpecialDividendRate": "-"
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: CashDividendDataResponse = serde_json::from_str(json_data).unwrap();

        let expected_dividend = vec![
            CashDividendItem {
                announcement_date: "2023-03-06".to_string(),
                announcement_time: "10:00".to_string(),
                code: "86970".to_string(),
                reference_number: "1".to_string(),
                status_code: DevidendStatucCode::New,
                board_meeting_date: "2023-03-06".to_string(),
                interim_final_code: DividendInterimFinalCode::Interim,
                forecast_result_code: DividendForecastResultCode::Determined,
                interim_final_term: "2023-04".to_string(),
                gross_dividend_rate: AmountPerShare::Number(100.0),
                record_date: "2023-03-10".to_string(),
                ex_date: "2023-03-05".to_string(),
                actual_record_date: "2023-03-10".to_string(),
                payable_date: PayableDate::Date("2023-03-15".to_string()),
                ca_reference_number: "1".to_string(),
                distribution_amount: AmountPerShare::Number(100.0),
                retained_earnings: AmountPerShare::Number(50.0),
                deemed_dividend: AmountPerShare::Number(0.0),
                deemed_capital_gains: AmountPerShare::Number(0.0),
                net_asset_decrease_ratio: AmountPerShare::Number(0.05),
                commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
                commemorative_dividend_rate: AmountPerShare::Undetermined,
                special_dividend_rate: AmountPerShare::Undetermined,
            },
            CashDividendItem {
                announcement_date: "2023-03-07".to_string(),
                announcement_time: "11:00".to_string(),
                code: "86970".to_string(),
                reference_number: "2".to_string(),
                status_code: DevidendStatucCode::Revised,
                board_meeting_date: "2023-03-07".to_string(),
                interim_final_code: DividendInterimFinalCode::Final,
                forecast_result_code: DividendForecastResultCode::Determined,
                interim_final_term: "2023-04".to_string(),
                gross_dividend_rate: AmountPerShare::Number(110.0),
                record_date: "2023-03-12".to_string(),
                ex_date: "2023-03-07".to_string(),
                actual_record_date: "2023-03-12".to_string(),
                payable_date: PayableDate::Date("2023-03-17".to_string()),
                ca_reference_number: "1".to_string(),
                distribution_amount: AmountPerShare::Number(110.0),
                retained_earnings: AmountPerShare::Number(55.0),
                deemed_dividend: AmountPerShare::Number(0.0),
                deemed_capital_gains: AmountPerShare::Number(0.0),
                net_asset_decrease_ratio: AmountPerShare::Number(0.055),
                commemorative_special_code: DividendCommemorativeSpecialCode::Commemorative,
                commemorative_dividend_rate: AmountPerShare::Number(10.0),
                special_dividend_rate: AmountPerShare::Undetermined,
            },
        ];

        let expected_response = CashDividendDataResponse {
            dividend: expected_dividend,
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_cash_dividend_data_response_no_data() {
        let json_data = r#"
        {
            "dividend": []
        }
        "#;

        let response: CashDividendDataResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = CashDividendDataResponse {
            dividend: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");
        std::env::set_var("POLARS_TABLE_WIDTH", "999");

        let response = CashDividendDataResponse {
            dividend: vec![
                CashDividendItem {
                    announcement_date: "2023-03-06".to_string(),
                    announcement_time: "10:00".to_string(),
                    code: "86970".to_string(),
                    reference_number: "1".to_string(),
                    status_code: DevidendStatucCode::New,
                    board_meeting_date: "2023-03-07".to_string(),
                    interim_final_code: DividendInterimFinalCode::Interim,
                    forecast_result_code: DividendForecastResultCode::Determined,
                    interim_final_term: "2023-04".to_string(),
                    gross_dividend_rate: AmountPerShare::Number(100.0),
                    record_date: "2023-03-08".to_string(),
                    ex_date: "2023-03-09".to_string(),
                    actual_record_date: "2023-03-10".to_string(),
                    payable_date: PayableDate::Date("2023-03-11".to_string()),
                    ca_reference_number: "1".to_string(),
                    distribution_amount: AmountPerShare::Number(100.0),
                    retained_earnings: AmountPerShare::Number(200.0),
                    deemed_dividend: AmountPerShare::Number(300.0),
                    deemed_capital_gains: AmountPerShare::Number(400.0),
                    net_asset_decrease_ratio: AmountPerShare::Number(500.0),
                    commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
                    commemorative_dividend_rate: AmountPerShare::Number(600.0),
                    special_dividend_rate: AmountPerShare::Number(700.0),
                },
                CashDividendItem {
                    announcement_date: "2023-03-07".to_string(),
                    announcement_time: "11:00".to_string(),
                    code: "86970".to_string(),
                    reference_number: "2".to_string(),
                    status_code: DevidendStatucCode::Revised,
                    board_meeting_date: "2023-03-07".to_string(),
                    interim_final_code: DividendInterimFinalCode::Final,
                    forecast_result_code: DividendForecastResultCode::Determined,
                    interim_final_term: "2023-04".to_string(),
                    gross_dividend_rate: AmountPerShare::Undetermined,
                    record_date: "2023-03-12".to_string(),
                    ex_date: "2023-03-13".to_string(),
                    actual_record_date: "2023-03-14".to_string(),
                    payable_date: PayableDate::Undetermined,
                    ca_reference_number: "2".to_string(),
                    distribution_amount: AmountPerShare::Undetermined,
                    retained_earnings: AmountPerShare::Undetermined,
                    deemed_dividend: AmountPerShare::Undetermined,
                    deemed_capital_gains: AmountPerShare::Undetermined,
                    net_asset_decrease_ratio: AmountPerShare::Undetermined,
                    commemorative_special_code: DividendCommemorativeSpecialCode::Commemorative,
                    commemorative_dividend_rate: AmountPerShare::Undetermined,
                    special_dividend_rate: AmountPerShare::Undetermined,
                },
                CashDividendItem {
                    announcement_date: "2023-03-08".to_string(),
                    announcement_time: "12:00".to_string(),
                    code: "86970".to_string(),
                    reference_number: "3".to_string(),
                    status_code: DevidendStatucCode::Delete,
                    board_meeting_date: "2023-03-08".to_string(),
                    interim_final_code: DividendInterimFinalCode::Unknown("aaa".to_string()),
                    forecast_result_code: DividendForecastResultCode::Unknown("bbb".to_string()),
                    interim_final_term: "2023-05".to_string(),
                    gross_dividend_rate: AmountPerShare::NotApplicable,
                    record_date: "2023-03-16".to_string(),
                    ex_date: "2023-03-17".to_string(),
                    actual_record_date: "2023-03-18".to_string(),
                    payable_date: PayableDate::NotApplicable,
                    ca_reference_number: "3".to_string(),
                    distribution_amount: AmountPerShare::NotApplicable,
                    retained_earnings: AmountPerShare::NotApplicable,
                    deemed_dividend: AmountPerShare::NotApplicable,
                    deemed_capital_gains: AmountPerShare::NotApplicable,
                    net_asset_decrease_ratio: AmountPerShare::NotApplicable,
                    commemorative_special_code: DividendCommemorativeSpecialCode::Unknown(
                        "ccc".to_string(),
                    ),
                    commemorative_dividend_rate: AmountPerShare::NotApplicable,
                    special_dividend_rate: AmountPerShare::NotApplicable,
                },
            ],
            pagination_key: Some("value3.value4.".to_string()),
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (3, 32)
            ┌──────────────────┬──────────────────┬───────┬─────────────────┬────────────┬──────────────────┬──────────────────┬────────────────────┬──────────────────┬──────────────────────────┬───────────────────┬────────────┬────────────┬──────────────────┬────────────────────┬─────────────┬───────────────────┬───────────────────────────┬────────────────────┬─────────────────────────┬──────────────────┬───────────────────────┬────────────────┬───────────────────────────┬────────────────────┬──────────────────────────────┬───────────────────────┬──────────────────────────┬─────────────────────────────────┬───────────────────────────┬────────────────────────────┬─────────────────────┐
            │ AnnouncementDate ┆ AnnouncementTime ┆ Code  ┆ ReferenceNumber ┆ StatusCode ┆ BoardMeetingDate ┆ InterimFinalCode ┆ ForecastResultCode ┆ InterimFinalTerm ┆ GrossDividendRateVariant ┆ GrossDividendRate ┆ RecordDate ┆ ExDate     ┆ ActualRecordDate ┆ PayableDateVariant ┆ PayableDate ┆ CAReferenceNumber ┆ DistributionAmountVariant ┆ DistributionAmount ┆ RetainedEarningsVariant ┆ RetainedEarnings ┆ DeemedDividendVariant ┆ DeemedDividend ┆ DeemedCapitalGainsVariant ┆ DeemedCapitalGains ┆ NetAssetDecreaseRatioVariant ┆ NetAssetDecreaseRatio ┆ CommemorativeSpecialCode ┆ CommemorativeDividendRateVaria… ┆ CommemorativeDividendRate ┆ SpecialDividendRateVariant ┆ SpecialDividendRate │
            │ ---              ┆ ---              ┆ ---   ┆ ---             ┆ ---        ┆ ---              ┆ ---              ┆ ---                ┆ ---              ┆ ---                      ┆ ---               ┆ ---        ┆ ---        ┆ ---              ┆ ---                ┆ ---         ┆ ---               ┆ ---                       ┆ ---                ┆ ---                     ┆ ---              ┆ ---                   ┆ ---            ┆ ---                       ┆ ---                ┆ ---                          ┆ ---                   ┆ ---                      ┆ ---                             ┆ ---                       ┆ ---                        ┆ ---                 │
            │ date             ┆ time             ┆ cat   ┆ str             ┆ cat        ┆ date             ┆ cat              ┆ cat                ┆ date             ┆ cat                      ┆ f64               ┆ date       ┆ date       ┆ date             ┆ cat                ┆ date        ┆ str               ┆ cat                       ┆ f64                ┆ cat                     ┆ f64              ┆ cat                   ┆ f64            ┆ cat                       ┆ f64                ┆ cat                          ┆ f64                   ┆ cat                      ┆ cat                             ┆ f64                       ┆ cat                        ┆ f64                 │
            ╞══════════════════╪══════════════════╪═══════╪═════════════════╪════════════╪══════════════════╪══════════════════╪════════════════════╪══════════════════╪══════════════════════════╪═══════════════════╪════════════╪════════════╪══════════════════╪════════════════════╪═════════════╪═══════════════════╪═══════════════════════════╪════════════════════╪═════════════════════════╪══════════════════╪═══════════════════════╪════════════════╪═══════════════════════════╪════════════════════╪══════════════════════════════╪═══════════════════════╪══════════════════════════╪═════════════════════════════════╪═══════════════════════════╪════════════════════════════╪═════════════════════╡
            │ 2023-03-06       ┆ 10:00:00         ┆ 86970 ┆ 1               ┆ 1          ┆ 2023-03-07       ┆ 1                ┆ 1                  ┆ 2023-04-01       ┆ Number                   ┆ 100.0             ┆ 2023-03-08 ┆ 2023-03-09 ┆ 2023-03-10       ┆ Date               ┆ 2023-03-11  ┆ 1                 ┆ Number                    ┆ 100.0              ┆ Number                  ┆ 200.0            ┆ Number                ┆ 300.0          ┆ Number                    ┆ 400.0              ┆ Number                       ┆ 500.0                 ┆ 0                        ┆ Number                          ┆ 600.0                     ┆ Number                     ┆ 700.0               │
            │ 2023-03-07       ┆ 11:00:00         ┆ 86970 ┆ 2               ┆ 2          ┆ 2023-03-07       ┆ 2                ┆ 1                  ┆ 2023-04-01       ┆ Undetermined             ┆ null              ┆ 2023-03-12 ┆ 2023-03-13 ┆ 2023-03-14       ┆ Undetermined       ┆ null        ┆ 2                 ┆ Undetermined              ┆ null               ┆ Undetermined            ┆ null             ┆ Undetermined          ┆ null           ┆ Undetermined              ┆ null               ┆ Undetermined                 ┆ null                  ┆ 1                        ┆ Undetermined                    ┆ null                      ┆ Undetermined               ┆ null                │
            │ 2023-03-08       ┆ 12:00:00         ┆ 86970 ┆ 3               ┆ 3          ┆ 2023-03-08       ┆ aaa              ┆ bbb                ┆ 2023-05-01       ┆ NotApplicable            ┆ null              ┆ 2023-03-16 ┆ 2023-03-17 ┆ 2023-03-18       ┆ NotApplicable      ┆ null        ┆ 3                 ┆ NotApplicable             ┆ null               ┆ NotApplicable           ┆ null             ┆ NotApplicable         ┆ null           ┆ NotApplicable             ┆ null               ┆ NotApplicable                ┆ null                  ┆ ccc                      ┆ NotApplicable                   ┆ null                      ┆ NotApplicable              ┆ null                │
            └──────────────────┴──────────────────┴───────┴─────────────────┴────────────┴──────────────────┴──────────────────┴────────────────────┴──────────────────┴──────────────────────────┴───────────────────┴────────────┴────────────┴──────────────────┴────────────────────┴─────────────┴───────────────────┴───────────────────────────┴────────────────────┴─────────────────────────┴──────────────────┴───────────────────────┴────────────────┴───────────────────────────┴────────────────────┴──────────────────────────────┴───────────────────────┴──────────────────────────┴─────────────────────────────────┴───────────────────────────┴────────────────────────────┴─────────────────────┘"#]]
        .assert_eq(&df.to_string());
    }
}
