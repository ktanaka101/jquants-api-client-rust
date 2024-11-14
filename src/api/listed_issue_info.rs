//! Listed info API endpoints.

use std::{fmt, marker::PhantomData};

use market_code::MarketCode;
use sector17_code::Sector17Code;
use sector33_code::Sector33Code;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{JQuantsApiClient, JQuantsPlanClient};

mod market_code;
mod sector17_code;
mod sector33_code;

/// Builder for Listed Issue Info API.
#[derive(Serialize)]
pub struct ListedIssueInfoApiBuilder<R: DeserializeOwned + fmt::Debug> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Date (e.g. 27800 or 2780)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}
impl<R: DeserializeOwned + fmt::Debug> ListedIssueInfoApiBuilder<R> {
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            date: None,
        }
    }

    /// Set issue code. (e.g. 27800 or 2780)
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        self.code = Some(code.into());
        self
    }

    /// Set date. (e.g. 27800 or 2780)
    pub fn date(&mut self, date: impl Into<String>) -> &mut Self {
        self.date = Some(date.into());
        self
    }

    /// Get listed information.
    pub async fn send(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get::<R>("listed/info", self).await
    }
}

/// Listed issue info API endpoints.
pub trait ListedIssueInfoApi: JQuantsPlanClient {
    /// Response type for listed info API.
    type Response: DeserializeOwned + fmt::Debug;

    /// Get listed issue information.
    ///
    /// Use [Listed Issue Information (/listed/info) API](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
    ///
    /// # Parameters
    ///
    /// [API Param specification](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info#parameter-and-response)
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
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedIssueInfoLightPlanResponse {
    /// The listed info for light plan.
    pub info: Vec<IssueInfoLightPlan>,
}

/// Listed issue info response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
pub type ListedIssueInfoStandardPlanResponse = ListedIssueInfoPremiumPlanResponse;

/// Listed issue info response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedIssueInfoPremiumPlanResponse {
    /// The listed info for premium plan.
    pub info: Vec<IssueInfoPremiumPlan>,
}

/// Issue info for free plan.
pub type IssueInfoFreePlan = IssueInfoLightPlan;

/// Issue info for light plan.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct IssueInfoLightPlan {
    /// The common structure for issue info.
    #[serde(flatten)]
    pub common: IssueInfoCommon,
}

/// Issue info for standard plan.
pub type IssueInfoStandardPlan = IssueInfoPremiumPlan;

/// Issue info for standard plan.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct IssueInfoPremiumPlan {
    /// The common structure for issue info.
    #[serde(flatten)]
    pub common: IssueInfoCommon,

    /// The margin code.
    #[serde(rename = "MarginCode")]
    pub margin_code: String,

    /// The margin code name.
    #[serde(rename = "MarginCodeName")]
    pub margin_code_name: String,
}

/// Common structure for issue info.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct IssueInfoCommon {
    /// The date.
    #[serde(rename = "Date")]
    pub date: String,

    /// The code.
    #[serde(rename = "Code")]
    pub code: String,

    /// The company name.
    #[serde(rename = "CompanyName")]
    pub company_name: String,

    /// The company name in English.
    #[serde(rename = "CompanyNameEnglish")]
    pub company_name_english: String,

    /// The sector 17 code.
    #[serde(rename = "Sector17Code")]
    pub sector17_code: Sector17Code,

    /// The sector 17 code name.
    #[serde(rename = "Sector17CodeName")]
    pub sector17_code_name: String,

    /// The sector 33 code.
    #[serde(rename = "Sector33Code")]
    pub sector33_code: Sector33Code,

    /// The sector 33 code name.
    #[serde(rename = "Sector33CodeName")]
    pub sector33_code_name: String,

    /// The scale category.
    #[serde(rename = "ScaleCategory")]
    pub scale_category: String,

    /// The market code.
    #[serde(rename = "MarketCode")]
    pub market_code: MarketCode,

    /// The market code name.
    #[serde(rename = "MarketCodeName")]
    pub market_code_name: String,
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
                info: vec![IssueInfoLightPlan {
                    common: IssueInfoCommon {
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
                info: vec![IssueInfoPremiumPlan {
                    common: IssueInfoCommon {
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
                    margin_code: "1".to_string(),
                    margin_code_name: "信用".to_string(),
                }],
            };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
