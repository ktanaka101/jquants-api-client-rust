//! Short Sale Value and Ratio by Sector API.

use std::{fmt, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Sector33Code;

use super::{
    shared::traits::{
        builder::JQuantsBuilder,
        pagination::{HasPaginationKey, MergePage, Paginatable},
    },
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Short Sale Value and Ratio by Sector API.
#[derive(Clone, Serialize)]
pub struct ShortSaleBySectorBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// 33-sector code (e.g. "0050" or "50")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector33code: Option<Sector33Code>,
    /// Starting point of data period (e.g. "20210901" or "2021-09-01")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End point of data period (e.g. "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Date of data (e.g. "20210907" or "2021-09-07")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R> for ShortSaleBySectorBuilder<R> {
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("markets/short_selling", self).await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for ShortSaleBySectorBuilder<R>
{
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> ShortSaleBySectorBuilder<R> {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            sector33code: None,
            from: None,
            to: None,
            date: None,
            pagination_key: None,
        }
    }

    /// Set 33-sector code (e.g. "0050" or "50")
    pub fn sector33code(mut self, sector33code: Sector33Code) -> Self {
        self.sector33code = Some(sector33code);
        self
    }

    /// Set starting point of data period (e.g. "20210901" or "2021-09-01")
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set end point of data period (e.g. "20210907" or "2021-09-07")
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set date of data (e.g. "20210907" or "2021-09-07")
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Builder for Short Sale Value and Ratio by Sector API.
pub trait ShortSaleBySectorApi: JQuantsPlanClient {
    /// Response type for Short Sale Value and Ratio by Sector API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get API builder for Short Sale Value and Ratio by Sector.
    ///
    /// Use [Short Sale Value and Ratio by Sector (/markets/short_selling) API](https://jpx.gitbook.io/j-quants-en/api-reference/short_selling)
    fn get_short_sale_by_sector(&self) -> ShortSaleBySectorBuilder<Self::Response> {
        ShortSaleBySectorBuilder::new(self.get_api_client().clone())
    }
}

/// Short Sale Value and Ratio by Sector response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/short_selling)
pub type ShortSaleBySectorFreePlanResponse = ShortSaleBySectorPremiumPlanResponse;

/// Short Sale Value and Ratio by Sector response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/short_selling)
pub type ShortSaleBySectorLightPlanResponse = ShortSaleBySectorPremiumPlanResponse;

/// Short Sale Value and Ratio by Sector response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/short_selling)
pub type ShortSaleBySectorStandardPlanResponse = ShortSaleBySectorPremiumPlanResponse;

/// Short Sale Value and Ratio by Sector response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/short_selling)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShortSaleBySectorPremiumPlanResponse {
    /// List of short selling data
    pub short_selling: Vec<ShortSaleBySectorItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for ShortSaleBySectorPremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for ShortSaleBySectorPremiumPlanResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.short_selling.extend(p.short_selling);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Represents a single short selling data item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShortSaleBySectorItem {
    /// Date (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,

    /// 33-sector code
    #[serde(rename = "Sector33Code")]
    pub sector33code: Sector33Code,

    /// Trading value of long selling
    #[serde(rename = "SellingExcludingShortSellingTurnoverValue")]
    pub selling_excluding_short_selling_turnover_value: Option<f64>,

    /// Value of short sales with price restrictions
    #[serde(rename = "ShortSellingWithRestrictionsTurnoverValue")]
    pub short_selling_with_restrictions_turnover_value: Option<f64>,

    /// Value of short sales without price restrictions
    #[serde(rename = "ShortSellingWithoutRestrictionsTurnoverValue")]
    pub short_selling_without_restrictions_turnover_value: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_short_sale_by_sector_premium_plan_response() {
        let json = r#"
            {
                "short_selling": [
                    {
                        "Date": "2022-10-25",
                        "Sector33Code": "0050",
                        "SellingExcludingShortSellingTurnoverValue": 1333126400.0,
                        "ShortSellingWithRestrictionsTurnoverValue": 787355200.0,
                        "ShortSellingWithoutRestrictionsTurnoverValue": 149084300.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: ShortSaleBySectorPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = ShortSaleBySectorPremiumPlanResponse {
            short_selling: vec![ShortSaleBySectorItem {
                date: "2022-10-25".to_string(),
                sector33code: Sector33Code::FisheryAgricultureForestry,
                selling_excluding_short_selling_turnover_value: Some(1333126400.0),
                short_selling_with_restrictions_turnover_value: Some(787355200.0),
                short_selling_without_restrictions_turnover_value: Some(149084300.0),
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_short_sale_by_sector_premium_plan_response_no_pagination_key() {
        let json = r#"
            {
                "short_selling": [
                    {
                        "Date": "2022-10-25",
                        "Sector33Code": "0050",
                        "SellingExcludingShortSellingTurnoverValue": 1333126400.0,
                        "ShortSellingWithRestrictionsTurnoverValue": 787355200.0,
                        "ShortSellingWithoutRestrictionsTurnoverValue": 149084300.0
                    }
                ]
            }
        "#;

        let response: ShortSaleBySectorPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = ShortSaleBySectorPremiumPlanResponse {
            short_selling: vec![ShortSaleBySectorItem {
                date: "2022-10-25".to_string(),
                sector33code: Sector33Code::FisheryAgricultureForestry,
                selling_excluding_short_selling_turnover_value: Some(1333126400.0),
                short_selling_with_restrictions_turnover_value: Some(787355200.0),
                short_selling_without_restrictions_turnover_value: Some(149084300.0),
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_short_sale_by_sector_premium_plan_response_multiple_items() {
        let json = r#"
            {
                "short_selling": [
                    {
                        "Date": "2022-10-18",
                        "Sector33Code": "0050",
                        "SellingExcludingShortSellingTurnoverValue": 1300000000.0,
                        "ShortSellingWithRestrictionsTurnoverValue": 780000000.0,
                        "ShortSellingWithoutRestrictionsTurnoverValue": 150000000.0
                    },
                    {
                        "Date": "2022-10-25",
                        "Sector33Code": "0050",
                        "SellingExcludingShortSellingTurnoverValue": 1333126400.0,
                        "ShortSellingWithRestrictionsTurnoverValue": 787355200.0,
                        "ShortSellingWithoutRestrictionsTurnoverValue": 149084300.0
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: ShortSaleBySectorPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = ShortSaleBySectorPremiumPlanResponse {
            short_selling: vec![
                ShortSaleBySectorItem {
                    date: "2022-10-18".to_string(),
                    sector33code: Sector33Code::FisheryAgricultureForestry,
                    selling_excluding_short_selling_turnover_value: Some(1300000000.0),
                    short_selling_with_restrictions_turnover_value: Some(780000000.0),
                    short_selling_without_restrictions_turnover_value: Some(150000000.0),
                },
                ShortSaleBySectorItem {
                    date: "2022-10-25".to_string(),
                    sector33code: Sector33Code::FisheryAgricultureForestry,
                    selling_excluding_short_selling_turnover_value: Some(1333126400.0),
                    short_selling_with_restrictions_turnover_value: Some(787355200.0),
                    short_selling_without_restrictions_turnover_value: Some(149084300.0),
                },
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_short_sale_by_sector_premium_plan_response_no_data() {
        let json = r#"
            {
                "short_selling": []
            }
        "#;

        let response: ShortSaleBySectorPremiumPlanResponse = serde_json::from_str(json).unwrap();
        let expected_response = ShortSaleBySectorPremiumPlanResponse {
            short_selling: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
