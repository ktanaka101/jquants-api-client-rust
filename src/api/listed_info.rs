//! Listed info API endpoints.

use std::fmt;

use market_code::MarketCode;
use sector17_code::Sector17Code;
use sector33_code::Sector33Code;
use serde::{de::DeserializeOwned, Deserialize};

use crate::JQuantsError;

use super::JQuantsPlanClient;

mod market_code;
mod sector17_code;
mod sector33_code;

/// Listed info API endpoints.
pub trait ListedInfoApi: JQuantsPlanClient + Send {
    /// Response type for listed info API.
    type Response: DeserializeOwned + fmt::Debug;

    /// Get listed information
    ///
    /// Use [Listed Information (/listed/info) API](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
    ///
    /// # Parameters
    ///
    /// [API Param specification](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info#parameter-and-response)
    /// * `code` - Issue code (e.g. 27800 or 2780)
    /// * `date` - Date (e.g. 20210907 or 2021-09-07)
    fn get_listed_info(
        &mut self,
        code: &str,
        date: &str,
        pagination_key: &str,
    ) -> impl std::future::Future<Output = Result<Self::Response, JQuantsError>> + Send {
        async move {
            let params = [
                ("code", code),
                ("date", date),
                ("pagination_key", pagination_key),
            ];
            self.get_mut_client()
                .get::<Self::Response>("listed/info", Some(&params[..]))
                .await
        }
    }
}

/// Listed info response for free plan
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedInfoFreePlanResponse {
    /// The listed info for free plan.
    pub info: Vec<ListedInfoFreePlan>,
}

/// Listed info response for standard plan
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedInfoStandardPlanResponse {
    /// The listed info for standard plan.
    pub info: Vec<ListedInfoStandardPlan>,
}

/// Listed info for free plan
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedInfoFreePlan {
    /// The common structure for listed info.
    #[serde(flatten)]
    pub base: ListedInfoBase,
}

/// Listed info for standard plan
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedInfoStandardPlan {
    /// The common structure for listed info.
    #[serde(flatten)]
    pub base: ListedInfoBase,

    /// The margin code.
    #[serde(rename = "MarginCode")]
    pub margin_code: String,

    /// The margin code name.
    #[serde(rename = "MarginCodeName")]
    pub margin_code_name: String,
}

/// Common structure for listed info
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ListedInfoBase {
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
    fn test_deserialize_listed_info_free_plan_response() {
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

        let response: ListedInfoFreePlanResponse = serde_json::from_str(json).unwrap();
        let expected_response: ListedInfoFreePlanResponse = ListedInfoFreePlanResponse {
            info: vec![ListedInfoFreePlan {
                base: ListedInfoBase {
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

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_listed_info_standard_plan_response() {
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

        let response: ListedInfoStandardPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response: ListedInfoStandardPlanResponse = ListedInfoStandardPlanResponse {
            info: vec![ListedInfoStandardPlan {
                base: ListedInfoBase {
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

        assert_eq!(response, expected_response);
    }
}
