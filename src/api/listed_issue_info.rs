//! Listed info API endpoints.

use std::{fmt, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::MarginCode;

use super::{
    shared::{
        traits::builder::JQuantsBuilder,
        types::{
            market_code::MarketCode, sector17_code::Sector17Code, sector33_code::Sector33Code,
        },
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Listed Issue Info API.
#[derive(Clone, Serialize)]
pub struct ListedIssueInfoApiBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    ///
    /// If a 4-character issue code is specified,
    /// only the data of common stock will be obtained for the issue on which both common and preferred stocks are listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// Date of application of information
    /// (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}
impl<R: DeserializeOwned + fmt::Debug + Clone> ListedIssueInfoApiBuilder<R> {
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            date: None,
        }
    }

    /// Set issue code. (e.g. 27800 or 2780)
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set date. (e.g. 27800 or 2780)
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R> for ListedIssueInfoApiBuilder<R> {
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("listed/info", self).await
    }
}

/// Listed issue info API endpoints.
pub trait ListedIssueInfoApi: JQuantsPlanClient {
    /// Response type for listed info API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get api builder for listed issue information.
    ///
    /// Use [Listed Issue Information (/listed/info) API](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
    fn get_listed_issue_info(&self) -> ListedIssueInfoApiBuilder<Self::Response> {
        ListedIssueInfoApiBuilder::new(self.get_api_client().clone())
    }
}

/// Listed issue info response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
pub type ListedIssueInfoFreePlanResponse = ListedIssueInfoLightPlanResponse;

/// Listed issue info response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ListedIssueInfoLightPlanResponse {
    /// The listed info for light plan.
    pub info: Vec<IssueInfoLightPlanItem>,
}

/// Listed issue info response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
pub type ListedIssueInfoStandardPlanResponse = ListedIssueInfoPremiumPlanResponse;

/// Listed issue info response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListedIssueInfoPremiumPlanResponse {
    /// The listed info for premium plan.
    pub info: Vec<IssueInfoPremiumPlanItem>,
}

/// Issue info for free plan.
pub type IssueInfoFreePlanItem = IssueInfoLightPlanItem;

/// Issue info for light plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueInfoLightPlanItem {
    /// The common structure for issue info.
    #[serde(flatten)]
    pub common: IssueInfoCommonItem,
}

/// Issue info for standard plan.
pub type IssueInfoStandardPlanItem = IssueInfoPremiumPlanItem;

/// Issue info for standard plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueInfoPremiumPlanItem {
    /// The common structure for issue info.
    #[serde(flatten)]
    pub common: IssueInfoCommonItem,

    /// Flags of margin and loan issues.
    #[serde(rename = "MarginCode")]
    pub margin_code: MarginCode,

    /// Name of flags of margin and loan issues.
    #[serde(rename = "MarginCodeName")]
    pub margin_code_name: String,
}

/// Common structure for issue info.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueInfoCommonItem {
    /// Date of application of information (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// Issue code.
    #[serde(rename = "Code")]
    pub code: String,

    /// Company Name (Japanese)
    #[serde(rename = "CompanyName")]
    pub company_name: String,

    /// Company Name (English).
    #[serde(rename = "CompanyNameEnglish")]
    pub company_name_english: String,

    /// 17-Sector code.
    #[serde(rename = "Sector17Code")]
    pub sector17_code: Sector17Code,

    /// 17-Sector code name (Japanese).
    #[serde(rename = "Sector17CodeName")]
    pub sector17_code_name: String,

    /// 33-Sector code.
    #[serde(rename = "Sector33Code")]
    pub sector33_code: Sector33Code,

    /// 33-Sector code name (Japanese).
    #[serde(rename = "Sector33CodeName")]
    pub sector33_code_name: String,

    /// TOPIX Scale category.
    #[serde(rename = "ScaleCategory")]
    pub scale_category: String,

    /// Market segment code.
    #[serde(rename = "MarketCode")]
    pub market_code: MarketCode,

    /// Market segment code name (Japanese).
    #[serde(rename = "MarketCodeName")]
    pub market_code_name: String,
}

#[cfg(feature = "polars")]
use polars::prelude::*;

#[cfg(feature = "polars")]
fn build_common_columns(
    data: Vec<IssueInfoCommonItem>,
) -> Result<Vec<Column>, crate::polars_utils::IntoPolarsError> {
    use crate::polars_utils::build_categorical_column;

    let mut dates = Vec::with_capacity(data.len());
    let mut codes = Vec::with_capacity(data.len());
    let mut company_names = Vec::with_capacity(data.len());
    let mut company_names_english = Vec::with_capacity(data.len());
    let mut sector17_codes = Vec::with_capacity(data.len());
    let mut sector17_code_names = Vec::with_capacity(data.len());
    let mut sector33_codes = Vec::with_capacity(data.len());
    let mut sector33_code_names = Vec::with_capacity(data.len());
    let mut scale_categories = Vec::with_capacity(data.len());
    let mut market_codes = Vec::with_capacity(data.len());
    let mut market_code_names = Vec::with_capacity(data.len());

    for common_item in data {
        let IssueInfoCommonItem {
            date,
            code,
            company_name,
            company_name_english,
            sector17_code,
            sector17_code_name,
            sector33_code,
            sector33_code_name,
            scale_category,
            market_code,
            market_code_name,
        } = common_item;

        dates.push(date);
        codes.push(code);
        company_names.push(company_name);
        company_names_english.push(company_name_english);
        sector17_codes.push(sector17_code);
        sector17_code_names.push(sector17_code_name);
        sector33_codes.push(sector33_code);
        sector33_code_names.push(sector33_code_name);
        scale_categories.push(scale_category);
        market_codes.push(market_code);
        market_code_names.push(market_code_name);
    }

    let columns = vec![
        Column::new("Date".into(), dates).cast(&DataType::Date)?,
        Column::new("Code".into(), codes),
        Column::new("CompanyName".into(), company_names),
        Column::new("CompanyNameEnglish".into(), company_names_english),
        build_categorical_column("Sector17Code", sector17_codes)?,
        build_categorical_column("Sector17CodeName", sector17_code_names)?,
        build_categorical_column("Sector33Code", sector33_codes)?,
        build_categorical_column("Sector33CodeName", sector33_code_names)?,
        build_categorical_column("ScaleCategory", scale_categories)?,
        build_categorical_column("MarketCode", market_codes)?,
        build_categorical_column("MarketCodeName", market_code_names)?,
    ];

    Ok(columns)
}

#[cfg(feature = "polars")]
impl ListedIssueInfoLightPlanResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        let data = self.info;
        let columns = build_common_columns(data.into_iter().map(|d| d.common).collect::<Vec<_>>())?;
        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
}

#[cfg(feature = "polars")]
impl ListedIssueInfoPremiumPlanResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_categorical_column;

        let data = self.info;

        let mut common = Vec::with_capacity(data.len());
        let mut margin_codes = Vec::with_capacity(data.len());
        let mut margin_code_names = Vec::with_capacity(data.len());

        for item in data {
            common.push(item.common);
            margin_codes.push(item.margin_code);
            margin_code_names.push(item.margin_code_name);
        }

        let mut columns = build_common_columns(common)?;
        columns.push(build_categorical_column("MarginCode", margin_codes)?);
        columns.push(build_categorical_column(
            "MarginCodeName",
            margin_code_names,
        )?);

        let df = polars::frame::DataFrame::new(columns)?;

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_listed_issue_info_light_plan_response() {
        let json = r#"
            {
                "info": [
                    {
                        "Date": "2022-11-11",
                        "Code": "86970",
                        "CompanyName": "日本取引所グループ",
                        "CompanyNameEnglish": "Japan Exchange Group,Inc.",
                        "Sector17Code": "16",
                        "Sector17CodeName": "金融（除く銀行）",
                        "Sector33Code": "7200",
                        "Sector33CodeName": "その他金融業",
                        "ScaleCategory": "TOPIX Large70",
                        "MarketCode": "0111",
                        "MarketCodeName": "プライム"
                    }
                ]
            }
        "#;

        let response: ListedIssueInfoLightPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response: ListedIssueInfoLightPlanResponse =
            ListedIssueInfoLightPlanResponse {
                info: vec![IssueInfoLightPlanItem {
                    common: IssueInfoCommonItem {
                        date: "2022-11-11".to_string(),
                        code: "86970".to_string(),
                        company_name: "日本取引所グループ".to_string(),
                        company_name_english: "Japan Exchange Group,Inc.".to_string(),
                        sector17_code: Sector17Code::FinancialsExBanks,
                        sector17_code_name: "金融（除く銀行）".to_string(),
                        sector33_code: Sector33Code::OtherFinancingBusiness,
                        sector33_code_name: "その他金融業".to_string(),
                        scale_category: "TOPIX Large70".to_string(),
                        market_code: MarketCode::Prime,
                        market_code_name: "プライム".to_string(),
                    },
                }],
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_listed_issue_info_premium_plan_response() {
        let json = r#"
            {
                "info": [
                    {
                        "Date": "2022-11-11",
                        "Code": "86970",
                        "CompanyName": "日本取引所グループ",
                        "CompanyNameEnglish": "Japan Exchange Group,Inc.",
                        "Sector17Code": "16",
                        "Sector17CodeName": "金融（除く銀行）",
                        "Sector33Code": "7200",
                        "Sector33CodeName": "その他金融業",
                        "ScaleCategory": "TOPIX Large70",
                        "MarketCode": "0111",
                        "MarketCodeName": "プライム",
                        "MarginCode": "1",
                        "MarginCodeName": "信用"
                    }
                ]
            }
        "#;

        let response: ListedIssueInfoPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response: ListedIssueInfoPremiumPlanResponse =
            ListedIssueInfoPremiumPlanResponse {
                info: vec![IssueInfoPremiumPlanItem {
                    common: IssueInfoCommonItem {
                        date: "2022-11-11".to_string(),
                        code: "86970".to_string(),
                        company_name: "日本取引所グループ".to_string(),
                        company_name_english: "Japan Exchange Group,Inc.".to_string(),
                        sector17_code: Sector17Code::FinancialsExBanks,
                        sector17_code_name: "金融（除く銀行）".to_string(),
                        sector33_code: Sector33Code::OtherFinancingBusiness,
                        sector33_code_name: "その他金融業".to_string(),
                        scale_category: "TOPIX Large70".to_string(),
                        market_code: MarketCode::Prime,
                        market_code_name: "プライム".to_string(),
                    },
                    margin_code: MarginCode::MarginIssues,
                    margin_code_name: "信用".to_string(),
                }],
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_light_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = ListedIssueInfoLightPlanResponse {
            info: vec![
                IssueInfoLightPlanItem {
                    common: IssueInfoCommonItem {
                        date: "2022-11-11".to_string(),
                        code: "86970".to_string(),
                        company_name: "Group".to_string(),
                        company_name_english: "JEG".to_string(),
                        sector17_code: Sector17Code::FinancialsExBanks,
                        sector17_code_name: "Bank".to_string(),
                        sector33_code: Sector33Code::OtherFinancingBusiness,
                        sector33_code_name: "Bank".to_string(),
                        scale_category: "TOPIX Large70".to_string(),
                        market_code: MarketCode::Prime,
                        market_code_name: "Prime".to_string(),
                    },
                },
                IssueInfoLightPlanItem {
                    common: IssueInfoCommonItem {
                        date: "2022-11-12".to_string(),
                        code: "86971".to_string(),
                        company_name: "Group".to_string(),
                        company_name_english: "JEG2".to_string(),
                        sector17_code: Sector17Code::Foods,
                        sector17_code_name: "Bank-A".to_string(),
                        sector33_code: Sector33Code::FisheryAgricultureForestry,
                        sector33_code_name: "Bank-B".to_string(),
                        scale_category: "TOPIX Large70-A".to_string(),
                        market_code: MarketCode::TSEFirstSection,
                        market_code_name: "Prime-B".to_string(),
                    },
                },
            ],
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (2, 11)
            ┌────────────┬───────┬─────────────┬────────────────────┬──────────────┬──────────────────┬──────────────┬──────────────────┬─────────────────┬────────────┬────────────────┐
            │ Date       ┆ Code  ┆ CompanyName ┆ CompanyNameEnglish ┆ Sector17Code ┆ Sector17CodeName ┆ Sector33Code ┆ Sector33CodeName ┆ ScaleCategory   ┆ MarketCode ┆ MarketCodeName │
            │ ---        ┆ ---   ┆ ---         ┆ ---                ┆ ---          ┆ ---              ┆ ---          ┆ ---              ┆ ---             ┆ ---        ┆ ---            │
            │ date       ┆ str   ┆ str         ┆ str                ┆ cat          ┆ cat              ┆ cat          ┆ cat              ┆ cat             ┆ cat        ┆ cat            │
            ╞════════════╪═══════╪═════════════╪════════════════════╪══════════════╪══════════════════╪══════════════╪══════════════════╪═════════════════╪════════════╪════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ Group       ┆ JEG                ┆ 16           ┆ Bank             ┆ 7200         ┆ Bank             ┆ TOPIX Large70   ┆ 0111       ┆ Prime          │
            │ 2022-11-12 ┆ 86971 ┆ Group       ┆ JEG2               ┆ 1            ┆ Bank-A           ┆ 0050         ┆ Bank-B           ┆ TOPIX Large70-A ┆ 0101       ┆ Prime-B        │
            └────────────┴───────┴─────────────┴────────────────────┴──────────────┴──────────────────┴──────────────┴──────────────────┴─────────────────┴────────────┴────────────────┘"#]]
        .assert_eq(&df.to_string());
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_premium_into_polars() {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

        let response = ListedIssueInfoPremiumPlanResponse {
            info: vec![IssueInfoPremiumPlanItem {
                common: IssueInfoCommonItem {
                    date: "2022-11-11".to_string(),
                    code: "86970".to_string(),
                    company_name: "Group".to_string(),
                    company_name_english: "JEG".to_string(),
                    sector17_code: Sector17Code::FinancialsExBanks,
                    sector17_code_name: "Bank".to_string(),
                    sector33_code: Sector33Code::OtherFinancingBusiness,
                    sector33_code_name: "Bank".to_string(),
                    scale_category: "TOPIX Large70".to_string(),
                    market_code: MarketCode::Prime,
                    market_code_name: "Prime".to_string(),
                },
                margin_code: MarginCode::MarginIssues,
                margin_code_name: "MarginTrading".to_string(),
            }],
        };

        let df = response.into_polars().unwrap();

        expect_test::expect![[r#"
            shape: (1, 13)
            ┌────────────┬───────┬─────────────┬────────────────────┬──────────────┬──────────────────┬──────────────┬──────────────────┬───────────────┬────────────┬────────────────┬────────────┬────────────────┐
            │ Date       ┆ Code  ┆ CompanyName ┆ CompanyNameEnglish ┆ Sector17Code ┆ Sector17CodeName ┆ Sector33Code ┆ Sector33CodeName ┆ ScaleCategory ┆ MarketCode ┆ MarketCodeName ┆ MarginCode ┆ MarginCodeName │
            │ ---        ┆ ---   ┆ ---         ┆ ---                ┆ ---          ┆ ---              ┆ ---          ┆ ---              ┆ ---           ┆ ---        ┆ ---            ┆ ---        ┆ ---            │
            │ date       ┆ str   ┆ str         ┆ str                ┆ cat          ┆ cat              ┆ cat          ┆ cat              ┆ cat           ┆ cat        ┆ cat            ┆ cat        ┆ cat            │
            ╞════════════╪═══════╪═════════════╪════════════════════╪══════════════╪══════════════════╪══════════════╪══════════════════╪═══════════════╪════════════╪════════════════╪════════════╪════════════════╡
            │ 2022-11-11 ┆ 86970 ┆ Group       ┆ JEG                ┆ 16           ┆ Bank             ┆ 7200         ┆ Bank             ┆ TOPIX Large70 ┆ 0111       ┆ Prime          ┆ 1          ┆ MarginTrading  │
            └────────────┴───────┴─────────────┴────────────────────┴──────────────┴──────────────────┴──────────────┴──────────────────┴───────────────┴────────────┴────────────────┴────────────┴────────────────┘"#]]
        .assert_eq(&df.to_string());
    }
}
