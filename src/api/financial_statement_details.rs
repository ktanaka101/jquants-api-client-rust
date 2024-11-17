//! Financial Statement Data(BS/PL) (/fins/fs_details) API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    shared::{
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
        types::type_of_document::TypeOfDocument,
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Financial Statement Details Data API.
#[derive(Clone, Serialize)]
pub struct FinancialStatementDetailsBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Issue code (e.g. "27890" or "2789")
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// Disclosure date (e.g. "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<FinancialStatementDetailsResponse> for FinancialStatementDetailsBuilder {
    async fn send(self) -> Result<FinancialStatementDetailsResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<FinancialStatementDetailsResponse, crate::JQuantsError> {
        self.client.inner.get("fins/fs_details", self).await
    }
}

impl Paginatable<FinancialStatementDetailsResponse> for FinancialStatementDetailsBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl FinancialStatementDetailsBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            code: None,
            date: None,
            pagination_key: None,
        }
    }

    /// Set issue code (e.g. "27890" or "2789")
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set disclosure date (e.g. "20210901" or "2021-09-01")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Trait for Financial Statement Details Data API.
pub trait FinancialStatementDetailsApi: JQuantsPlanClient {
    /// Get API builder for Financial Statement Details Data.
    ///
    /// Use [Financial Statement Details Data (/fins/fs_details) API](https://jpx.gitbook.io/j-quants-en/api-reference/statements-1)
    fn get_financial_statement_details(&self) -> FinancialStatementDetailsBuilder {
        FinancialStatementDetailsBuilder::new(self.get_api_client().clone())
    }
}

/// Financial Statement Details Data response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/statements-1)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FinancialStatementDetailsResponse {
    /// List of financial statement details
    pub fs_details: Vec<FinancialStatementDetailItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for FinancialStatementDetailsResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for FinancialStatementDetailsResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.fs_details.extend(p.fs_details);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single financial statement detail item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FinancialStatementDetailItem {
    /// Disclosed Date (YYYY-MM-DD)
    #[serde(rename = "DisclosedDate")]
    pub disclosed_date: String,

    /// Disclosed Time (HH:MM:SS)
    #[serde(rename = "DisclosedTime")]
    pub disclosed_time: String,

    /// Issue Code (5-character)
    #[serde(rename = "LocalCode")]
    pub local_code: String,

    /// Disclosure Number
    #[serde(rename = "DisclosureNumber")]
    pub disclosure_number: String,

    /// Type of Document
    #[serde(rename = "TypeOfDocument")]
    pub type_of_document: TypeOfDocument,

    /// Financial Statement Entries
    #[serde(rename = "FinancialStatement")]
    pub financial_statement: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn test_deserialize_financial_statement_details_response() {
        let json_data = r#"
        {
            "fs_details": [
                {
                      "DisclosedDate": "2023-01-30",
                      "DisclosedTime": "12:00:00",
                      "LocalCode": "86970",
                      "DisclosureNumber": "20230127594871",
                      "TypeOfDocument": "3QFinancialStatements_Consolidated_IFRS",
                      "FinancialStatement": {
                            "Goodwill (IFRS)": "67374000000",
                            "Retained earnings (IFRS)": "263894000000",
                            "Operating profit (loss) (IFRS)": "51765000000.0",
                            "Previous fiscal year end date, DEI": "2022-03-31",
                            "Basic earnings (loss) per share (IFRS)": "66.76",
                            "Document type, DEI": "四半期第３号参考様式　[IFRS]（連結）",
                            "Current period end date, DEI": "2022-12-31",
                            "Revenue - 2 (IFRS)": "100987000000.0",
                            "Industry code when consolidated financial statements are prepared in accordance with industry specific regulations, DEI": "CTE",
                            "Profit (loss) attributable to owners of parent (IFRS)": "35175000000.0",
                            "Other current liabilities - CL (IFRS)": "8904000000",
                            "Share of profit (loss) of investments accounted for using equity method (IFRS)": "1042000000.0",
                            "Current liabilities (IFRS)": "78852363000000",
                            "Equity attributable to owners of parent (IFRS)": "311103000000",
                            "Whether consolidated financial statements are prepared, DEI": "true",
                            "Non-current liabilities (IFRS)": "33476000000",
                            "Other expenses (IFRS)": "58000000.0",
                            "Income taxes payable - CL (IFRS)": "5245000000",
                            "Filer name in English, DEI": "Japan Exchange Group, Inc.",
                            "Non-controlling interests (IFRS)": "8918000000",
                            "Capital surplus (IFRS)": "38844000000",
                            "Finance costs (IFRS)": "71000000.0",
                            "Other current assets - CA (IFRS)": "4217000000",
                            "Property, plant and equipment (IFRS)": "11277000000",
                            "Deferred tax liabilities (IFRS)": "419000000",
                            "Other components of equity (IFRS)": "422000000",
                            "Current fiscal year start date, DEI": "2022-04-01",
                            "Type of current period, DEI": "Q3",
                            "Cash and cash equivalents (IFRS)": "91135000000",
                            "Share capital (IFRS)": "11500000000",
                            "Retirement benefit asset - NCA (IFRS)": "9028000000",
                            "Number of submission, DEI": "1",
                            "Trade and other receivables - CA (IFRS)": "18837000000",
                            "Liabilities and equity (IFRS)": "79205861000000",
                            "EDINET code, DEI": "E03814",
                            "Equity (IFRS)": "320021000000",
                            "Security code, DEI": "86970",
                            "Other financial assets - CA (IFRS)": "112400000000",
                            "Other financial assets - NCA (IFRS)": "2898000000",
                            "Income taxes receivable - CA (IFRS)": "5529000000",
                            "Investments accounted for using equity method (IFRS)": "18362000000",
                            "Other non-current assets - NCA (IFRS)": "6240000000",
                            "Previous fiscal year start date, DEI": "2021-04-01",
                            "Filer name in Japanese, DEI": "株式会社日本取引所グループ",
                            "Deferred tax assets (IFRS)": "2862000000",
                            "Trade and other payables - CL (IFRS)": "5037000000",
                            "Bonds and borrowings - CL (IFRS)": "33000000000",
                            "Current fiscal year end date, DEI": "2023-03-31",
                            "XBRL amendment flag, DEI": "false",
                            "Non-current assets (IFRS)": "182317000000",
                            "Retirement benefit liability - NCL (IFRS)": "9214000000",
                            "Amendment flag, DEI": "false",
                            "Assets (IFRS)": "79205861000000",
                            "Income tax expense (IFRS)": "15841000000.0",
                            "Report amendment flag, DEI": "false",
                            "Profit (loss) (IFRS)": "35894000000.0",
                            "Operating expenses (IFRS)": "50206000000.0",
                            "Intangible assets (IFRS)": "36324000000",
                            "Profit (loss) before tax from continuing operations (IFRS)": "51736000000.0",
                            "Liabilities (IFRS)": "78885839000000",
                            "Accounting standards, DEI": "IFRS",
                            "Bonds and borrowings - NCL (IFRS)": "19972000000",
                            "Finance income (IFRS)": "43000000.0",
                            "Profit (loss) attributable to non-controlling interests (IFRS)": "719000000.0",
                            "Comparative period end date, DEI": "2021-12-31",
                            "Current assets (IFRS)": "79023543000000",
                            "Other non-current liabilities - NCL (IFRS)": "3870000000",
                            "Other income (IFRS)": "458000000.0",
                            "Treasury shares (IFRS)": "-3556000000"
                      }
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: FinancialStatementDetailsResponse = serde_json::from_str(json_data).unwrap();
        let financial_statement_map: HashMap<&str, &str> = hashmap! {
            "Goodwill (IFRS)" => "67374000000",
            "Retained earnings (IFRS)" => "263894000000",
            "Operating profit (loss) (IFRS)" => "51765000000.0",
            "Previous fiscal year end date, DEI" => "2022-03-31",
            "Basic earnings (loss) per share (IFRS)" => "66.76",
            "Document type, DEI" => "四半期第３号参考様式　[IFRS]（連結）",
            "Current period end date, DEI" => "2022-12-31",
            "Revenue - 2 (IFRS)" => "100987000000.0",
            "Industry code when consolidated financial statements are prepared in accordance with industry specific regulations, DEI" => "CTE",
            "Profit (loss) attributable to owners of parent (IFRS)" => "35175000000.0",
            "Other current liabilities - CL (IFRS)" => "8904000000",
            "Share of profit (loss) of investments accounted for using equity method (IFRS)" => "1042000000.0",
            "Current liabilities (IFRS)" => "78852363000000",
            "Equity attributable to owners of parent (IFRS)" => "311103000000",
            "Whether consolidated financial statements are prepared, DEI" => "true",
            "Non-current liabilities (IFRS)" => "33476000000",
            "Other expenses (IFRS)" => "58000000.0",
            "Income taxes payable - CL (IFRS)" => "5245000000",
            "Filer name in English, DEI" => "Japan Exchange Group, Inc.",
            "Non-controlling interests (IFRS)" => "8918000000",
            "Capital surplus (IFRS)" => "38844000000",
            "Finance costs (IFRS)" => "71000000.0",
            "Other current assets - CA (IFRS)" => "4217000000",
            "Property, plant and equipment (IFRS)" => "11277000000",
            "Deferred tax liabilities (IFRS)" => "419000000",
            "Other components of equity (IFRS)" => "422000000",
            "Current fiscal year start date, DEI" => "2022-04-01",
            "Type of current period, DEI" =>  "Q3",
            "Cash and cash equivalents (IFRS)" => "91135000000",
            "Share capital (IFRS)" => "11500000000",
            "Retirement benefit asset - NCA (IFRS)" => "9028000000",
            "Number of submission, DEI" =>  "1",
            "Trade and other receivables - CA (IFRS)" => "18837000000",
            "Liabilities and equity (IFRS)" => "79205861000000",
            "EDINET code, DEI" =>  "E03814",
            "Equity (IFRS)" =>  "320021000000",
            "Security code, DEI" =>  "86970",
            "Other financial assets - CA (IFRS)" => "112400000000",
            "Other financial assets - NCA (IFRS)" => "2898000000",
            "Income taxes receivable - CA (IFRS)" => "5529000000",
            "Investments accounted for using equity method (IFRS)" => "18362000000",
            "Other non-current assets - NCA (IFRS)" => "6240000000",
            "Previous fiscal year start date, DEI" => "2021-04-01",
            "Filer name in Japanese, DEI" => "株式会社日本取引所グループ",
            "Deferred tax assets (IFRS)" => "2862000000",
            "Trade and other payables - CL (IFRS)" => "5037000000",
            "Bonds and borrowings - CL (IFRS)" => "33000000000",
            "Current fiscal year end date, DEI" => "2023-03-31",
            "XBRL amendment flag, DEI" =>  "false",
            "Non-current assets (IFRS)" => "182317000000",
            "Retirement benefit liability - NCL (IFRS)" => "9214000000",
            "Amendment flag, DEI" =>  "false",
            "Assets (IFRS)" =>  "79205861000000",
            "Income tax expense (IFRS)" => "15841000000.0",
            "Report amendment flag, DEI" => "false",
            "Profit (loss) (IFRS)" => "35894000000.0",
            "Operating expenses (IFRS)" => "50206000000.0",
            "Intangible assets (IFRS)" => "36324000000",
            "Profit (loss) before tax from continuing operations (IFRS)" => "51736000000.0",
            "Liabilities (IFRS)" => "78885839000000",
            "Accounting standards, DEI" =>  "IFRS",
            "Bonds and borrowings - NCL (IFRS)" => "19972000000",
            "Finance income (IFRS)" => "43000000.0",
            "Profit (loss) attributable to non-controlling interests (IFRS)" => "719000000.0",
            "Comparative period end date, DEI" => "2021-12-31",
            "Current assets (IFRS)" => "79023543000000",
            "Other non-current liabilities - NCL (IFRS)" => "3870000000",
            "Other income (IFRS)" => "458000000.0",
            "Treasury shares (IFRS)" => "-3556000000",
        };

        let expected_response = FinancialStatementDetailsResponse {
            fs_details: vec![FinancialStatementDetailItem {
                disclosed_date: "2023-01-30".to_string(),
                disclosed_time: "12:00:00".to_string(),
                local_code: "86970".to_string(),
                disclosure_number: "20230127594871".to_string(),
                type_of_document: TypeOfDocument::Q3FinancialStatementsConsolidatedIFRS,
                financial_statement: financial_statement_map
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_financial_statement_details_response_no_pagination_key() {
        let json_data = r#"
        {
            "fs_details": [
                {
                    "DisclosedDate": "2023-01-30",
                    "DisclosedTime": "12:00:00",
                    "LocalCode": "86970",
                    "DisclosureNumber": "20230127594871",
                    "TypeOfDocument": "3QFinancialStatements_Consolidated_IFRS",
                    "FinancialStatement": {
                        "Goodwill (IFRS)": "67374000000",
                        "Retained earnings (IFRS)": "263894000000"
                    }
                }
            ]
        }
        "#;

        let response: FinancialStatementDetailsResponse = serde_json::from_str(json_data).unwrap();
        let financial_statement_map: HashMap<&str, &str> = hashmap! {
            "Goodwill (IFRS)" =>  "67374000000",
            "Retained earnings (IFRS)" => "263894000000"
        };

        let expected_response = FinancialStatementDetailsResponse {
            fs_details: vec![FinancialStatementDetailItem {
                disclosed_date: "2023-01-30".to_string(),
                disclosed_time: "12:00:00".to_string(),
                local_code: "86970".to_string(),
                disclosure_number: "20230127594871".to_string(),
                type_of_document: TypeOfDocument::Q3FinancialStatementsConsolidatedIFRS,
                financial_statement: financial_statement_map
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_financial_statement_details_response_multiple_items() {
        let json_data = r#"
        {
            "fs_details": [
                {
                    "DisclosedDate": "2023-01-30",
                    "DisclosedTime": "12:00:00",
                    "LocalCode": "86970",
                    "DisclosureNumber": "20230127594871",
                    "TypeOfDocument": "3QFinancialStatements_Consolidated_IFRS",
                    "FinancialStatement": {
                        "Goodwill (IFRS)": "67374000000",
                        "Retained earnings (IFRS)": "263894000000"
                    }
                },
                {
                    "DisclosedDate": "2023-02-15",
                    "DisclosedTime": "14:30:00",
                    "LocalCode": "86971",
                    "DisclosureNumber": "20230227594872",
                    "TypeOfDocument": "OtherPeriodFinancialStatements_Consolidated_IFRS",
                    "FinancialStatement": {
                        "Goodwill (IFRS)": "70000000000",
                        "Retained earnings (IFRS)": "280000000000"
                    }
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: FinancialStatementDetailsResponse = serde_json::from_str(json_data).unwrap();

        let fs_map1: HashMap<&str, &str> = hashmap! {
            "Goodwill (IFRS)" => "67374000000",
            "Retained earnings (IFRS)" => "263894000000",
            // ... other fields omitted for brevity
        };

        let fs_map2: HashMap<&str, &str> = hashmap! {
            "Goodwill (IFRS)" => "70000000000",
            "Retained earnings (IFRS)" => "280000000000"
            // ... other fields omitted for brevity
        };

        let expected_response = FinancialStatementDetailsResponse {
            fs_details: vec![
                FinancialStatementDetailItem {
                    disclosed_date: "2023-01-30".to_string(),
                    disclosed_time: "12:00:00".to_string(),
                    local_code: "86970".to_string(),
                    disclosure_number: "20230127594871".to_string(),
                    type_of_document: TypeOfDocument::Q3FinancialStatementsConsolidatedIFRS,
                    financial_statement: fs_map1
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                },
                FinancialStatementDetailItem {
                    disclosed_date: "2023-02-15".to_string(),
                    disclosed_time: "14:30:00".to_string(),
                    local_code: "86971".to_string(),
                    disclosure_number: "20230227594872".to_string(),
                    type_of_document:
                        TypeOfDocument::OtherPeriodFinancialStatementsConsolidatedIFRS,
                    financial_statement: fs_map2
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                },
            ],
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_financial_statement_details_response_no_data() {
        let json_data = r#"
        {
            "fs_details": []
        }
        "#;

        let response: FinancialStatementDetailsResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = FinancialStatementDetailsResponse {
            fs_details: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
