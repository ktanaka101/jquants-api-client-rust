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
    ///
    /// Ascending order by disclosure number.
    #[serde(rename = "DisclosureNumber")]
    pub disclosure_number: String,

    /// Type of Document
    #[serde(rename = "TypeOfDocument")]
    pub type_of_document: TypeOfDocument,

    /// Financial Statement Entries
    ///
    /// Redundant labels (English) associated with XBRL tags and their values
    #[serde(rename = "FinancialStatement")]
    pub financial_statement: HashMap<String, String>,
}

#[cfg(feature = "polars")]
impl FinancialStatementDetailsResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::{build_categorical_column, hashmap_list_to_columns};
        use polars::prelude::*;

        let data = self.fs_details;

        let mut disclosed_dates = Vec::with_capacity(data.len());
        let mut disclosed_times = Vec::with_capacity(data.len());
        let mut local_codes = Vec::with_capacity(data.len());
        let mut disclosure_numbers = Vec::with_capacity(data.len());
        let mut type_of_documents = Vec::with_capacity(data.len());
        let mut financial_statements = Vec::with_capacity(data.len());

        for item in data {
            let FinancialStatementDetailItem {
                disclosed_date,
                disclosed_time,
                local_code,
                disclosure_number,
                type_of_document,
                financial_statement,
            } = item;

            disclosed_dates.push(disclosed_date);
            disclosed_times.push(disclosed_time);
            local_codes.push(local_code);
            disclosure_numbers.push(disclosure_number);
            type_of_documents.push(type_of_document);
            financial_statements.push(financial_statement);
        }

        let extra_columns = hashmap_list_to_columns(financial_statements);
        let mut columns = vec![
            Column::new("DisclosedDate".into(), disclosed_dates).cast(&DataType::Date)?,
            Column::new("DisclosedTime".into(), disclosed_times).cast(&DataType::Time)?,
            build_categorical_column("LocalCode", local_codes)?,
            build_categorical_column("DisclosureNumber", disclosure_numbers)?,
            build_categorical_column("TypeOfDocument", type_of_documents)?,
        ];
        columns.extend(extra_columns);

        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
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

    #[cfg(feature = "polars")]
    #[test]
    fn test_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

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

        let response = FinancialStatementDetailsResponse {
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

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (1, 74)
            ┌─────────────┬─────────────┬───────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────┬─────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬────────┬─────────────┬─────────────┬─────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────┬─────────┬────────────┬────────────┐
            │ DisclosedDa ┆ DisclosedTi ┆ LocalCode ┆ DisclosureN ┆ TypeOfDocum ┆ Accounting  ┆ Amendment   ┆ Assets      ┆ Basic       ┆ Bonds and   ┆ Bonds and   ┆ Capital     ┆ Cash and    ┆ Comparative ┆ Current     ┆ Current     ┆ Current     ┆ Current     ┆ Current     ┆ Deferred    ┆ Deferred    ┆ Document    ┆ EDINET      ┆ Equity      ┆ Equity attr ┆ Filer name  ┆ Filer name  ┆ Finance ┆ Finance ┆ Goodwill    ┆ Income tax  ┆ Income      ┆ Income      ┆ Industry    ┆ Intangible  ┆ Investments ┆ Liabilities ┆ Liabilities ┆ Non-control ┆ Non-current ┆ Non-current ┆ Number of   ┆ Operating   ┆ Operating   ┆ Other       ┆ Other       ┆ Other       ┆ Other       ┆ Other       ┆ Other       ┆ Other  ┆ Other       ┆ Other       ┆ Previous    ┆ Previous   ┆ Profit     ┆ Profit     ┆ Profit     ┆ Profit     ┆ Property,  ┆ Report     ┆ Retained   ┆ Retirement ┆ Retirement ┆ Revenue -  ┆ Security   ┆ Share      ┆ Share of   ┆ Trade and  ┆ Trade and  ┆ Treasury   ┆ Type of ┆ Whether    ┆ XBRL       │
            │ te          ┆ me          ┆ ---       ┆ umber       ┆ ent         ┆ standards,  ┆ flag, DEI   ┆ (IFRS)      ┆ earnings    ┆ borrowings  ┆ borrowings  ┆ surplus     ┆ cash        ┆ period end  ┆ assets      ┆ fiscal year ┆ fiscal year ┆ liabilities ┆ period end  ┆ tax assets  ┆ tax         ┆ type, DEI   ┆ code, DEI   ┆ (IFRS)      ┆ ibutable to ┆ in English, ┆ in          ┆ costs   ┆ income  ┆ (IFRS)      ┆ expense     ┆ taxes       ┆ taxes       ┆ code when   ┆ assets      ┆ accounted   ┆ (IFRS)      ┆ and equity  ┆ ling        ┆ assets      ┆ liabilities ┆ submission, ┆ expenses    ┆ profit      ┆ components  ┆ current     ┆ current     ┆ expenses    ┆ financial   ┆ financial   ┆ income ┆ non-current ┆ non-current ┆ fiscal year ┆ fiscal     ┆ (loss)     ┆ (loss) att ┆ (loss) att ┆ (loss)     ┆ plant and  ┆ amendment  ┆ earnings   ┆ benefit    ┆ benefit    ┆ 2 (IFRS)   ┆ code, DEI  ┆ capital    ┆ profit     ┆ other      ┆ other rece ┆ shares     ┆ current ┆ consolidat ┆ amendment  │
            │ ---         ┆ ---         ┆ cat       ┆ ---         ┆ ---         ┆ DEI         ┆ ---         ┆ ---         ┆ (loss) per  ┆ - CL (IFR…  ┆ - NCL (IF…  ┆ (IFRS)      ┆ equivalents ┆ date, D…    ┆ (IFRS)      ┆ end date, … ┆ start date… ┆ (IFRS)      ┆ date, DEI   ┆ (IFRS)      ┆ liabilities ┆ ---         ┆ ---         ┆ ---         ┆ owners …    ┆ DEI         ┆ Japanese,   ┆ (IFRS)  ┆ (IFRS)  ┆ ---         ┆ (IFRS)      ┆ payable -   ┆ receivable  ┆ consolidate ┆ (IFRS)      ┆ for usin…   ┆ ---         ┆ (IFRS)      ┆ interests   ┆ (IFRS)      ┆ (IFRS)      ┆ DEI         ┆ (IFRS)      ┆ (loss)      ┆ of equity   ┆ assets - CA ┆ liabilities ┆ (IFRS)      ┆ assets - CA ┆ assets -    ┆ (IFRS) ┆ assets -    ┆ liabilities ┆ end date,…  ┆ year start ┆ (IFRS)     ┆ ributable  ┆ ributable  ┆ before tax ┆ equipment  ┆ flag, DEI  ┆ (IFRS)     ┆ asset -    ┆ liability  ┆ ---        ┆ ---        ┆ (IFRS)     ┆ (loss) of  ┆ payables - ┆ ivables -  ┆ (IFRS)     ┆ period, ┆ ed         ┆ flag, DEI  │
            │ date        ┆ time        ┆           ┆ cat         ┆ cat         ┆ ---         ┆ bool        ┆ f64         ┆ shar…       ┆ ---         ┆ ---         ┆ ---         ┆ (IFR…       ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ (IFRS…      ┆ str         ┆ str         ┆ f64         ┆ ---         ┆ ---         ┆ DEI         ┆ ---     ┆ ---     ┆ f64         ┆ ---         ┆ CL (IFR…    ┆ - CA (…     ┆ …           ┆ ---         ┆ ---         ┆ f64         ┆ ---         ┆ (IFR…       ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ (IFRS)      ┆ (IF…        ┆ (IFR…       ┆ - CL…       ┆ ---         ┆ (I…         ┆ NCA (…      ┆ ---    ┆ NCA…        ┆ …           ┆ ---         ┆ dat…       ┆ ---        ┆ to …       ┆ to …       ┆ from …     ┆ …          ┆ ---        ┆ ---        ┆ NCA…       ┆ -…         ┆ f64        ┆ f64        ┆ ---        ┆ inve…      ┆ CL …       ┆ …          ┆ ---        ┆ DEI     ┆ financial… ┆ ---        │
            │             ┆             ┆           ┆             ┆             ┆ str         ┆             ┆             ┆ ---         ┆ f64         ┆ f64         ┆ f64         ┆ ---         ┆ str         ┆ f64         ┆ str         ┆ str         ┆ f64         ┆ str         ┆ f64         ┆ ---         ┆             ┆             ┆             ┆ f64         ┆ str         ┆ ---         ┆ f64     ┆ f64     ┆             ┆ f64         ┆ ---         ┆ ---         ┆ ---         ┆ f64         ┆ f64         ┆             ┆ f64         ┆ ---         ┆ f64         ┆ f64         ┆ f64         ┆ f64         ┆ ---         ┆ ---         ┆ ---         ┆ ---         ┆ f64         ┆ ---         ┆ ---         ┆ f64    ┆ ---         ┆ ---         ┆ str         ┆ ---        ┆ f64        ┆ ---        ┆ ---        ┆ ---        ┆ ---        ┆ bool       ┆ f64        ┆ ---        ┆ ---        ┆            ┆            ┆ f64        ┆ ---        ┆ ---        ┆ ---        ┆ f64        ┆ ---     ┆ ---        ┆ bool       │
            │             ┆             ┆           ┆             ┆             ┆             ┆             ┆             ┆ f64         ┆             ┆             ┆             ┆ f64         ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆ f64         ┆             ┆             ┆             ┆             ┆             ┆ str         ┆         ┆         ┆             ┆             ┆ f64         ┆ f64         ┆ str         ┆             ┆             ┆             ┆             ┆ f64         ┆             ┆             ┆             ┆             ┆ f64         ┆ f64         ┆ f64         ┆ f64         ┆             ┆ f64         ┆ f64         ┆        ┆ f64         ┆ f64         ┆             ┆ str        ┆            ┆ f64        ┆ f64        ┆ f64        ┆ f64        ┆            ┆            ┆ f64        ┆ f64        ┆            ┆            ┆            ┆ f64        ┆ f64        ┆ f64        ┆            ┆ str     ┆ bool       ┆            │
            ╞═════════════╪═════════════╪═══════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════╪═════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪═════════════╪════════╪═════════════╪═════════════╪═════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪════════════╪═════════╪════════════╪════════════╡
            │ 2023-01-30  ┆ null        ┆ 86970     ┆ 20230127594 ┆ 3QFinancial ┆ IFRS        ┆ false       ┆ 7.9206e13   ┆ 66.76       ┆ 3.3000e10   ┆ 1.9972e10   ┆ 3.8844e10   ┆ 9.1135e10   ┆ 2021-12-31  ┆ 7.9024e13   ┆ 2023-03-31  ┆ 2022-04-01  ┆ 7.8852e13   ┆ 2022-12-31  ┆ 2.8620e9    ┆ 4.19e8      ┆ 四半期第３  ┆ E03814      ┆ 3.2002e11   ┆ 3.1110e11   ┆ Japan       ┆ 株式会社日  ┆ 7.1e7   ┆ 4.3e7   ┆ 6.7374e10   ┆ 1.5841e10   ┆ 5.2450e9    ┆ 5.5290e9    ┆ CTE         ┆ 3.6324e10   ┆ 1.8362e10   ┆ 7.8886e13   ┆ 7.9206e13   ┆ 8.9180e9    ┆ 1.8232e11   ┆ 3.3476e10   ┆ 1.0         ┆ 5.0206e10   ┆ 5.1765e10   ┆ 4.22e8      ┆ 4.2170e9    ┆ 8.9040e9    ┆ 5.8e7       ┆ 1.1240e11   ┆ 2.8980e9    ┆ 4.58e8 ┆ 6.2400e9    ┆ 3.8700e9    ┆ 2022-03-31  ┆ 2021-04-01 ┆ 3.5894e10  ┆ 7.19e8     ┆ 3.5175e10  ┆ 5.1736e10  ┆ 1.1277e10  ┆ false      ┆ 2.6389e11  ┆ 9.0280e9   ┆ 9.2140e9   ┆ 1.0099e11  ┆ 86970.0    ┆ 1.1500e10  ┆ 1.0420e9   ┆ 5.0370e9   ┆ 1.8837e10  ┆ -3.5560e9  ┆ Q3      ┆ true       ┆ false      │
            │             ┆             ┆           ┆ 871         ┆ Statements_ ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆ 号参考様式  ┆             ┆             ┆             ┆ Exchange    ┆ 本取引所グ  ┆         ┆         ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆        ┆             ┆             ┆             ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆         ┆            ┆            │
            │             ┆             ┆           ┆             ┆ Consolid…   ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆ 　[IFRS]（  ┆             ┆             ┆             ┆ Group, Inc. ┆ ループ      ┆         ┆         ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆        ┆             ┆             ┆             ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆         ┆            ┆            │
            │             ┆             ┆           ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆ 連結）      ┆             ┆             ┆             ┆             ┆             ┆         ┆         ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆             ┆        ┆             ┆             ┆             ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆            ┆         ┆            ┆            │
            └─────────────┴─────────────┴───────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────┴─────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴────────┴─────────────┴─────────────┴─────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴────────────┴─────────┴────────────┴────────────┘"#]]
        .assert_eq(&df.to_string());
    }
}
