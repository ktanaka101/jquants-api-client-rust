//! Cash Dividend Data (/fins/dividend) API.

use serde::{Deserialize, Serialize};

use super::{
    shared::{
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::dividend::{
            DevidendStatucCode, DividendCommemorativeSpecialCode, DividendForecastResultCode,
            DividendInterimFinalCode,
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
    pub gross_dividend_rate: String,

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
    pub payable_date: String,

    /// CA Reference Number
    #[serde(rename = "CAReferenceNumber")]
    pub ca_reference_number: String,

    /// Distribution Amount per Share
    #[serde(rename = "DistributionAmount")]
    pub distribution_amount: String,

    /// Retained Earnings per Share
    #[serde(rename = "RetainedEarnings")]
    pub retained_earnings: String,

    /// Deemed Dividend per Share
    #[serde(rename = "DeemedDividend")]
    pub deemed_dividend: String,

    /// Deemed Capital Gains per Share
    #[serde(rename = "DeemedCapitalGains")]
    pub deemed_capital_gains: String,

    /// Net Asset Decrease Ratio
    #[serde(rename = "NetAssetDecreaseRatio")]
    pub net_asset_decrease_ratio: String,

    /// Commemorative/Special Code
    #[serde(rename = "CommemorativeSpecialCode")]
    pub commemorative_special_code: DividendCommemorativeSpecialCode,

    /// Commemorative Dividend Rate per Share
    #[serde(rename = "CommemorativeDividendRate")]
    pub commemorative_dividend_rate: String,

    /// Special Dividend Rate per Share
    #[serde(rename = "SpecialDividendRate")]
    pub special_dividend_rate: String,
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
            gross_dividend_rate: "-".to_string(),
            record_date: "2014-03-10".to_string(),
            ex_date: "2014-03-06".to_string(),
            actual_record_date: "2014-03-10".to_string(),
            payable_date: "-".to_string(),
            ca_reference_number: "201402241B00002".to_string(),
            distribution_amount: "".to_string(),
            retained_earnings: "".to_string(),
            deemed_dividend: "".to_string(),
            deemed_capital_gains: "".to_string(),
            net_asset_decrease_ratio: "".to_string(),
            commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
            commemorative_dividend_rate: "".to_string(),
            special_dividend_rate: "".to_string(),
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
            gross_dividend_rate: "-".to_string(),
            record_date: "2014-03-10".to_string(),
            ex_date: "2014-03-06".to_string(),
            actual_record_date: "2014-03-10".to_string(),
            payable_date: "-".to_string(),
            ca_reference_number: "201402241B00002".to_string(),
            distribution_amount: "".to_string(),
            retained_earnings: "".to_string(),
            deemed_dividend: "".to_string(),
            deemed_capital_gains: "".to_string(),
            net_asset_decrease_ratio: "".to_string(),
            commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
            commemorative_dividend_rate: "".to_string(),
            special_dividend_rate: "".to_string(),
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
                gross_dividend_rate: "100".to_string(),
                record_date: "2023-03-10".to_string(),
                ex_date: "2023-03-05".to_string(),
                actual_record_date: "2023-03-10".to_string(),
                payable_date: "2023-03-15".to_string(),
                ca_reference_number: "1".to_string(),
                distribution_amount: "100".to_string(),
                retained_earnings: "50".to_string(),
                deemed_dividend: "0".to_string(),
                deemed_capital_gains: "0".to_string(),
                net_asset_decrease_ratio: "0.05".to_string(),
                commemorative_special_code: DividendCommemorativeSpecialCode::Normal,
                commemorative_dividend_rate: "-".to_string(),
                special_dividend_rate: "-".to_string(),
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
                gross_dividend_rate: "110".to_string(),
                record_date: "2023-03-12".to_string(),
                ex_date: "2023-03-07".to_string(),
                actual_record_date: "2023-03-12".to_string(),
                payable_date: "2023-03-17".to_string(),
                ca_reference_number: "1".to_string(),
                distribution_amount: "110".to_string(),
                retained_earnings: "55".to_string(),
                deemed_dividend: "0".to_string(),
                deemed_capital_gains: "0".to_string(),
                net_asset_decrease_ratio: "0.055".to_string(),
                commemorative_special_code: DividendCommemorativeSpecialCode::Commemorative,
                commemorative_dividend_rate: "10".to_string(),
                special_dividend_rate: "-".to_string(),
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
}
