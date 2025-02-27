//! Earnings Calendar (/fins/announcement) API

use serde::{Deserialize, Serialize};

use super::{
    shared::{
        deserialize_utils::empty_string_or_null_as_none,
        traits::{
            builder::JQuantsBuilder,
            pagination::{HasPaginationKey, MergePage, Paginatable},
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Earnings Calendar Data API.
#[derive(Clone, Serialize)]
pub struct EarningsCalendarBuilder {
    #[serde(skip)]
    client: JQuantsApiClient,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl JQuantsBuilder<EarningsCalendarResponse> for EarningsCalendarBuilder {
    async fn send(self) -> Result<EarningsCalendarResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<EarningsCalendarResponse, crate::JQuantsError> {
        self.client.inner.get("fins/announcement", self).await
    }
}

impl Paginatable<EarningsCalendarResponse> for EarningsCalendarBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl EarningsCalendarBuilder {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            pagination_key: None,
        }
    }

    /// Set pagination key for fetching the next set of data.
    pub fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

/// Trait for Earnings Calendar Data API.
pub trait EarningsCalendarApi: JQuantsPlanClient {
    /// Get API builder for Earnings Calendar Data.
    ///
    /// Use [Earnings Calendar Data (/fins/announcement) API](https://jpx.gitbook.io/j-quants-en/api-reference/announcement)
    fn get_earnings_calendar(&self) -> EarningsCalendarBuilder {
        EarningsCalendarBuilder::new(self.get_api_client().clone())
    }
}

/// Earnings Calendar Data API response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/announcement)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EarningsCalendarResponse {
    /// List of earnings announcements
    pub announcement: Vec<EarningsAnnouncementItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for EarningsCalendarResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for EarningsCalendarResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.announcement.extend(p.announcement);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single earnings announcement item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EarningsAnnouncementItem {
    /// Announcement Date (YYYY-MM-DD)
    ///
    /// If the earnings announcement date is undecided, the data will be an empty string ("").
    #[serde(rename = "Date", deserialize_with = "empty_string_or_null_as_none")]
    pub date: Option<String>,

    /// Issue Code (e.g., "43760")
    #[serde(rename = "Code")]
    pub code: String,

    /// Company Name (Japanese)
    #[serde(rename = "CompanyName")]
    pub company_name: String,

    /// End of Fiscal Year (Japanese, e.g., "9月30日")
    #[serde(rename = "FiscalYear")]
    pub fiscal_year: String,

    /// Sector Name (Japanese)
    #[serde(rename = "SectorName")]
    pub sector_name: String,

    /// Fiscal Quarter (Japanese, e.g., "第１四半期")
    #[serde(rename = "FiscalQuarter")]
    pub fiscal_quarter: String,

    /// Market Segment Name (Japanese, e.g., "マザーズ")
    #[serde(rename = "Section")]
    pub section: String,
}

#[cfg(feature = "polars")]
impl EarningsCalendarResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_categorical_column;
        use polars::prelude::*;

        let data = self.announcement;

        let mut dates = Vec::with_capacity(data.len());
        let mut codes = Vec::with_capacity(data.len());
        let mut company_names = Vec::with_capacity(data.len());
        let mut fiscal_years = Vec::with_capacity(data.len());
        let mut sector_names = Vec::with_capacity(data.len());
        let mut fiscal_quarters = Vec::with_capacity(data.len());
        let mut sections = Vec::with_capacity(data.len());

        for item in data {
            let EarningsAnnouncementItem {
                date,
                code,
                company_name,
                fiscal_year,
                sector_name,
                fiscal_quarter,
                section,
            } = item;

            dates.push(date);
            codes.push(code);
            company_names.push(company_name);
            fiscal_years.push(fiscal_year);
            sector_names.push(sector_name);
            fiscal_quarters.push(fiscal_quarter);
            sections.push(section);
        }

        let columns = vec![
            Column::new("Date".into(), dates).cast(&DataType::Date)?,
            Column::new("Code".into(), codes),
            Column::new("CompanyName".into(), company_names),
            build_categorical_column("FiscalYear", fiscal_years)?,
            build_categorical_column("SectorName", sector_names)?,
            build_categorical_column("FiscalQuarter", fiscal_quarters)?,
            build_categorical_column("Section", sections)?,
        ];

        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_earnings_calendar_response() {
        let json_data = r#"
        {
            "announcement": [
                {
                    "Date": "2022-02-14",
                    "Code": "43760",
                    "CompanyName": "くふうカンパニー",
                    "FiscalYear": "9月30日",
                    "SectorName": "情報・通信業",
                    "FiscalQuarter": "第１四半期",
                    "Section": "マザーズ"
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: EarningsCalendarResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![EarningsAnnouncementItem {
            date: Some("2022-02-14".to_string()),
            code: "43760".to_string(),
            company_name: "くふうカンパニー".to_string(),
            fiscal_year: "9月30日".to_string(),
            sector_name: "情報・通信業".to_string(),
            fiscal_quarter: "第１四半期".to_string(),
            section: "マザーズ".to_string(),
        }];

        let expected_response = EarningsCalendarResponse {
            announcement: expected_announcement,
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_earnings_calendar_response_no_pagination_key() {
        let json_data = r#"
        {
            "announcement": [
                {
                    "Date": "2022-02-14",
                    "Code": "43760",
                    "CompanyName": "くふうカンパニー",
                    "FiscalYear": "9月30日",
                    "SectorName": "情報・通信業",
                    "FiscalQuarter": "第１四半期",
                    "Section": "マザーズ"
                }
            ]
        }
        "#;

        let response: EarningsCalendarResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![EarningsAnnouncementItem {
            date: Some("2022-02-14".to_string()),
            code: "43760".to_string(),
            company_name: "くふうカンパニー".to_string(),
            fiscal_year: "9月30日".to_string(),
            sector_name: "情報・通信業".to_string(),
            fiscal_quarter: "第１四半期".to_string(),
            section: "マザーズ".to_string(),
        }];

        let expected_response = EarningsCalendarResponse {
            announcement: expected_announcement,
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_earnings_calendar_response_multiple_items() {
        let json_data = r#"
        {
            "announcement": [
                {
                    "Date": "2023-03-06",
                    "Code": "86970",
                    "CompanyName": "株式会社XYZ",
                    "FiscalYear": "3月31日",
                    "SectorName": "製造業",
                    "FiscalQuarter": "第4四半期",
                    "Section": "東証プライム"
                },
                {
                    "Date": "2023-03-07",
                    "Code": "86971",
                    "CompanyName": "株式会社ABC",
                    "FiscalYear": "9月30日",
                    "SectorName": "金融業",
                    "FiscalQuarter": "第1四半期",
                    "Section": "東証マザーズ"
                }
            ],
            "pagination_key": "value3.value4."
        }
        "#;

        let response: EarningsCalendarResponse = serde_json::from_str(json_data).unwrap();

        let expected_announcement = vec![
            EarningsAnnouncementItem {
                date: Some("2023-03-06".to_string()),
                code: "86970".to_string(),
                company_name: "株式会社XYZ".to_string(),
                fiscal_year: "3月31日".to_string(),
                sector_name: "製造業".to_string(),
                fiscal_quarter: "第4四半期".to_string(),
                section: "東証プライム".to_string(),
            },
            EarningsAnnouncementItem {
                date: Some("2023-03-07".to_string()),
                code: "86971".to_string(),
                company_name: "株式会社ABC".to_string(),
                fiscal_year: "9月30日".to_string(),
                sector_name: "金融業".to_string(),
                fiscal_quarter: "第1四半期".to_string(),
                section: "東証マザーズ".to_string(),
            },
        ];

        let expected_response = EarningsCalendarResponse {
            announcement: expected_announcement,
            pagination_key: Some("value3.value4.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_earnings_calendar_response_no_data() {
        let json_data = r#"
        {
            "announcement": []
        }
        "#;

        let response: EarningsCalendarResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = EarningsCalendarResponse {
            announcement: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = EarningsCalendarResponse {
            announcement: vec![
                EarningsAnnouncementItem {
                    date: Some("2022-02-14".to_string()),
                    code: "43760".to_string(),
                    company_name: "ABC".to_string(),
                    fiscal_year: "9/30".to_string(),
                    sector_name: "IT".to_string(),
                    fiscal_quarter: "1Q".to_string(),
                    section: "Mothers".to_string(),
                },
                EarningsAnnouncementItem {
                    date: None,
                    code: "86970".to_string(),
                    company_name: "XYZ".to_string(),
                    fiscal_year: "3/31".to_string(),
                    sector_name: "Prod".to_string(),
                    fiscal_quarter: "4Q".to_string(),
                    section: "Prime".to_string(),
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 7)
            ┌────────────┬───────┬─────────────┬────────────┬────────────┬───────────────┬─────────┐
            │ Date       ┆ Code  ┆ CompanyName ┆ FiscalYear ┆ SectorName ┆ FiscalQuarter ┆ Section │
            │ ---        ┆ ---   ┆ ---         ┆ ---        ┆ ---        ┆ ---           ┆ ---     │
            │ date       ┆ str   ┆ str         ┆ cat        ┆ cat        ┆ cat           ┆ cat     │
            ╞════════════╪═══════╪═════════════╪════════════╪════════════╪═══════════════╪═════════╡
            │ 2022-02-14 ┆ 43760 ┆ ABC         ┆ 9/30       ┆ IT         ┆ 1Q            ┆ Mothers │
            │ null       ┆ 86970 ┆ XYZ         ┆ 3/31       ┆ Prod       ┆ 4Q            ┆ Prime   │
            └────────────┴───────┴─────────────┴────────────┴────────────┴───────────────┴─────────┘"#]].assert_eq(&df.to_string());
    }
}
