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

/// Parameters for listed info API.
#[derive(Serialize)]
pub struct ListedInfoApiBuilder<'a, R: DeserializeOwned + fmt::Debug> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Issue code (e.g. 27800 or 2780)
    pub code: Option<&'a str>,
    /// Date (e.g. 20210907 or 2021-09-07)
    pub date: Option<&'a str>,
}
impl<'a, R: DeserializeOwned + fmt::Debug> ListedInfoApiBuilder<'a, R> {
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            code: None,
            date: None,
        }
    }

    /// All listed issues as of the day when API is executed.
    pub fn all_stocks_today(self) -> Self {
        ListedInfoApiBuilder {
            code: None,
            date: None,
            ..self
        }
    }

    /// Specified listed issues as of the day when API is executed.
    pub fn stock_today(self, code: &'a str) -> Self {
        ListedInfoApiBuilder {
            code: Some(code),
            date: None,
            ..self
        }
    }

    /// All listed issues as of the specified day.
    pub fn all_stocks_on_date(self, date: &'a str) -> Self {
        ListedInfoApiBuilder {
            code: None,
            date: Some(date),
            ..self
        }
    }

    /// Specified listed issues as of the specified day.
    pub fn stock_on_date(self, code: &'a str, date: &'a str) -> Self {
        ListedInfoApiBuilder {
            code: Some(code),
            date: Some(date),
            ..self
        }
    }

    /// Set code.
    pub fn set_code(mut self, code: &'a str) -> Self {
        self.code = Some(code);
        self
    }

    /// Set date.
    pub fn set_date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get::<R>("listed/info", self).await
    }
}

/// Listed info API endpoints.
pub trait ListedInfoApi: JQuantsPlanClient {
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
    fn get_listed_info(&self) -> ListedInfoApiBuilder<Self::Response> {
        ListedInfoApiBuilder::new(self.get_api_client().clone())
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
