//! Trading by Type of Investors API.

pub mod section_name;

use std::{fmt, marker::PhantomData};

use section_name::SectionName;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    builder::JQuantsBuilder,
    pagination::{HasPaginationKey, MergePage, Paginatable},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Builder for Trading by Type of Investors API.
#[derive(Clone, Serialize)]
pub struct TradingByInvestorTypeBuilder<R: DeserializeOwned + fmt::Debug + Clone> {
    #[serde(skip)]
    client: JQuantsApiClient,
    #[serde(skip)]
    phantom: PhantomData<R>,

    /// Section name (e.g. TSEPrime)
    #[serde(skip_serializing_if = "Option::is_none")]
    section: Option<SectionName>,
    /// Starting point of data period (e.g. 20210901 or 2021-09-01)
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// End point of data period (e.g. 20210907 or 2021-09-07)
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,

    /// Pagination key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
}

impl<R: DeserializeOwned + fmt::Debug + Clone> JQuantsBuilder<R>
    for TradingByInvestorTypeBuilder<R>
{
    async fn send(self) -> Result<R, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<R, crate::JQuantsError> {
        self.client.inner.get("markets/trades_spec", self).await
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone + HasPaginationKey + MergePage> Paginatable<R>
    for TradingByInvestorTypeBuilder<R>
{
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl<R: DeserializeOwned + fmt::Debug + Clone> TradingByInvestorTypeBuilder<R> {
    /// Create a new builder.
    pub(crate) fn new(client: JQuantsApiClient) -> Self {
        Self {
            client,
            phantom: PhantomData,
            section: None,
            from: None,
            to: None,
            pagination_key: None,
        }
    }

    /// Set section name (e.g. TSEPrime)
    pub fn section(mut self, section: impl Into<SectionName>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// Set starting point of data period (e.g. 20210901 or 2021-09-01)
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set end point of data period (e.g. 20210907 or 2021-09-07)
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }
}

/// Builder for Trading by Type of Investors API.
pub trait TradingByInvestorTypeApi: JQuantsPlanClient {
    /// Response type for Trading by Type of Investors API.
    type Response: DeserializeOwned + fmt::Debug + Clone;

    /// Get API builder for Trading by Type of Investors.
    ///
    /// Use [Trading by Type of Investors (/markets/trades_spec) API](https://jpx.gitbook.io/j-quants-en/api-reference/trades_spec)
    fn get_trading_by_investor_type(&self) -> TradingByInvestorTypeBuilder<Self::Response> {
        TradingByInvestorTypeBuilder::new(self.get_api_client().clone())
    }
}

/// Trading by Type of Investors response for free plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/trades_spec)
pub type TradingByInvestorTypeFreePlanResponse = TradingByInvestorTypePremiumPlanResponse;

/// Trading by Type of Investors response for light plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/trades_spec)
pub type TradingByInvestorTypeLightPlanResponse = TradingByInvestorTypePremiumPlanResponse;

/// Trading by Type of Investors response for standard plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/trades_spec)
pub type TradingByInvestorTypeStandardPlanResponse = TradingByInvestorTypePremiumPlanResponse;

/// Trading by Type of Investors response for premium plan.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/trades_spec)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradingByInvestorTypePremiumPlanResponse {
    /// List of trades specifications
    pub trades_spec: Vec<TradingByInvestorTypePremiumPlan>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for TradingByInvestorTypePremiumPlanResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for TradingByInvestorTypePremiumPlanResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.trades_spec.extend(p.trades_spec);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}

/// Trades Specification for premium plan.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradingByInvestorTypePremiumPlan {
    /// The common structure for trades specification
    #[serde(flatten)]
    pub common: TradingByInvestorTypeCommon,
    // Add any additional fields for premium plan here if applicable
}

/// Common structure for trades specification.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradingByInvestorTypeCommon {
    /// Published Date (YY-MM-DD)
    #[serde(rename = "PublishedDate")]
    pub published_date: String,

    /// Start Date (YY-MM-DD)
    #[serde(rename = "StartDate")]
    pub start_date: String,

    /// End Date (YY-MM-DD)
    #[serde(rename = "EndDate")]
    pub end_date: String,

    /// Section Name
    #[serde(rename = "Section")]
    pub section: String,

    /// Proprietary Sales Value
    #[serde(rename = "ProprietarySales")]
    pub proprietary_sales: f64,

    /// Proprietary Purchase Value
    #[serde(rename = "ProprietaryPurchases")]
    pub proprietary_purchases: f64,

    /// Proprietary Total Value
    #[serde(rename = "ProprietaryTotal")]
    pub proprietary_total: f64,

    /// Proprietary Balance Value
    #[serde(rename = "ProprietaryBalance")]
    pub proprietary_balance: f64,

    /// Brokerage Sales Value
    #[serde(rename = "BrokerageSales")]
    pub brokerage_sales: f64,

    /// Brokerage Purchase Value
    #[serde(rename = "BrokeragePurchases")]
    pub brokerage_purchases: f64,

    /// Brokerage Total Value
    #[serde(rename = "BrokerageTotal")]
    pub brokerage_total: f64,

    /// Brokerage Balance Value
    #[serde(rename = "BrokerageBalance")]
    pub brokerage_balance: f64,

    /// Total Sales Value
    #[serde(rename = "TotalSales")]
    pub total_sales: f64,

    /// Total Purchase Value
    #[serde(rename = "TotalPurchases")]
    pub total_purchases: f64,

    /// Total Value
    #[serde(rename = "TotalTotal")]
    pub total_total: f64,

    /// Total Balance Value
    #[serde(rename = "TotalBalance")]
    pub total_balance: f64,

    /// Individuals Sales Value
    #[serde(rename = "IndividualsSales")]
    pub individuals_sales: f64,

    /// Individuals Purchase Value
    #[serde(rename = "IndividualsPurchases")]
    pub individuals_purchases: f64,

    /// Individuals Total Value
    #[serde(rename = "IndividualsTotal")]
    pub individuals_total: f64,

    /// Individuals Balance Value
    #[serde(rename = "IndividualsBalance")]
    pub individuals_balance: f64,

    /// Foreigners Sales Value
    #[serde(rename = "ForeignersSales")]
    pub foreigners_sales: f64,

    /// Foreigners Purchase Value
    #[serde(rename = "ForeignersPurchases")]
    pub foreigners_purchases: f64,

    /// Foreigners Total Value
    #[serde(rename = "ForeignersTotal")]
    pub foreigners_total: f64,

    /// Foreigners Balance Value
    #[serde(rename = "ForeignersBalance")]
    pub foreigners_balance: f64,

    /// Securities Companies Sales Value
    #[serde(rename = "SecuritiesCosSales")]
    pub securities_cos_sales: f64,

    /// Securities Companies Purchase Value
    #[serde(rename = "SecuritiesCosPurchases")]
    pub securities_cos_purchases: f64,

    /// Securities Companies Total
    #[serde(rename = "SecuritiesCosTotal")]
    pub securities_cos_total: f64,

    /// Securities Companies Balance Value
    #[serde(rename = "SecuritiesCosBalance")]
    pub securities_cos_balance: f64,

    /// Investment Trusts Sales Value
    #[serde(rename = "InvestmentTrustsSales")]
    pub investment_trusts_sales: f64,

    /// Investment Trusts Purchase Value
    #[serde(rename = "InvestmentTrustsPurchases")]
    pub investment_trusts_purchases: f64,

    /// Investment Trusts Total Value
    #[serde(rename = "InvestmentTrustsTotal")]
    pub investment_trusts_total: f64,

    /// Investment Trusts Balance Value
    #[serde(rename = "InvestmentTrustsBalance")]
    pub investment_trusts_balance: f64,

    /// Business Companies Sales Value
    #[serde(rename = "BusinessCosSales")]
    pub business_cos_sales: f64,

    /// Business Companies Purchase Value
    #[serde(rename = "BusinessCosPurchases")]
    pub business_cos_purchases: f64,

    /// Business Companies Total Value
    #[serde(rename = "BusinessCosTotal")]
    pub business_cos_total: f64,

    /// Business Companies Balance Value
    #[serde(rename = "BusinessCosBalance")]
    pub business_cos_balance: f64,

    /// Other Companies Sales Value
    #[serde(rename = "OtherCosSales")]
    pub other_cos_sales: f64,

    /// Other Companies Purchase Value
    #[serde(rename = "OtherCosPurchases")]
    pub other_cos_purchases: f64,

    /// Other Companies Total Value
    #[serde(rename = "OtherCosTotal")]
    pub other_cos_total: f64,

    /// Other Companies Balance Value
    #[serde(rename = "OtherCosBalance")]
    pub other_cos_balance: f64,

    /// Insurance Companies Sales Value
    #[serde(rename = "InsuranceCosSales")]
    pub insurance_cos_sales: f64,

    /// Insurance Companies Purchase Value
    #[serde(rename = "InsuranceCosPurchases")]
    pub insurance_cos_purchases: f64,

    /// Insurance Companies Total Value
    #[serde(rename = "InsuranceCosTotal")]
    pub insurance_cos_total: f64,

    /// Insurance Companies Balance Value
    #[serde(rename = "InsuranceCosBalance")]
    pub insurance_cos_balance: f64,

    /// City Banks Regional Banks Etc Sales Value
    #[serde(rename = "CityBKsRegionalBKsEtcSales")]
    pub city_bks_regional_bks_etc_sales: f64,

    /// City Banks Regional Banks Etc Purchase Value
    #[serde(rename = "CityBKsRegionalBKsEtcPurchases")]
    pub city_bks_regional_bks_etc_purchases: f64,

    /// City Banks Regional Banks Etc Total Value
    #[serde(rename = "CityBKsRegionalBKsEtcTotal")]
    pub city_bks_regional_bks_etc_total: f64,

    /// City Banks Regional Banks Etc Balance Value
    #[serde(rename = "CityBKsRegionalBKsEtcBalance")]
    pub city_bks_regional_bks_etc_balance: f64,

    /// Trust Banks Sales Value
    #[serde(rename = "TrustBanksSales")]
    pub trust_banks_sales: f64,

    /// Trust Banks Purchase Value
    #[serde(rename = "TrustBanksPurchases")]
    pub trust_banks_purchases: f64,

    /// Trust Banks Total Value
    #[serde(rename = "TrustBanksTotal")]
    pub trust_banks_total: f64,

    /// Trust Banks Balance Value
    #[serde(rename = "TrustBanksBalance")]
    pub trust_banks_balance: f64,

    /// Other Financial Institutions Sales Value
    #[serde(rename = "OtherFinancialInstitutionsSales")]
    pub other_financial_institutions_sales: f64,

    /// Other Financial Institutions Purchase Value
    #[serde(rename = "OtherFinancialInstitutionsPurchases")]
    pub other_financial_institutions_purchases: f64,

    /// Other Financial Institutions Total Value
    #[serde(rename = "OtherFinancialInstitutionsTotal")]
    pub other_financial_institutions_total: f64,

    /// Other Financial Institutions Balance Value
    #[serde(rename = "OtherFinancialInstitutionsBalance")]
    pub other_financial_institutions_balance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_trades_spec_premium_plan_response() {
        let json = r#"
            {
                "trades_spec": [
                    {
                        "PublishedDate": "2017-01-13",
                        "StartDate": "2017-01-04",
                        "EndDate": "2017-01-06",
                        "Section": "TSE1st",
                        "ProprietarySales": 1311271004,
                        "ProprietaryPurchases": 1453326508,
                        "ProprietaryTotal": 2764597512,
                        "ProprietaryBalance": 142055504,
                        "BrokerageSales": 7165529005,
                        "BrokeragePurchases": 7030019854,
                        "BrokerageTotal": 14195548859,
                        "BrokerageBalance": -135509151,
                        "TotalSales": 8476800009,
                        "TotalPurchases": 8483346362,
                        "TotalTotal": 16960146371,
                        "TotalBalance": 6546353,
                        "IndividualsSales": 1401711615,
                        "IndividualsPurchases": 1161801155,
                        "IndividualsTotal": 2563512770,
                        "IndividualsBalance": -239910460,
                        "ForeignersSales": 5094891735,
                        "ForeignersPurchases": 5317151774,
                        "ForeignersTotal": 10412043509,
                        "ForeignersBalance": 222260039,
                        "SecuritiesCosSales": 76381455,
                        "SecuritiesCosPurchases": 61700100,
                        "SecuritiesCosTotal": 138081555,
                        "SecuritiesCosBalance": -14681355,
                        "InvestmentTrustsSales": 168705109,
                        "InvestmentTrustsPurchases": 124389642,
                        "InvestmentTrustsTotal": 293094751,
                        "InvestmentTrustsBalance": -44315467,
                        "BusinessCosSales": 71217959,
                        "BusinessCosPurchases": 63526641,
                        "BusinessCosTotal": 134744600,
                        "BusinessCosBalance": -7691318,
                        "OtherCosSales": 10745152,
                        "OtherCosPurchases": 15687836,
                        "OtherCosTotal": 26432988,
                        "OtherCosBalance": 4942684,
                        "InsuranceCosSales": 15926202,
                        "InsuranceCosPurchases": 9831555,
                        "InsuranceCosTotal": 25757757,
                        "InsuranceCosBalance": -6094647,
                        "CityBKsRegionalBKsEtcSales": 10606789,
                        "CityBKsRegionalBKsEtcPurchases": 8843871,
                        "CityBKsRegionalBKsEtcTotal": 19450660,
                        "CityBKsRegionalBKsEtcBalance": -1762918,
                        "TrustBanksSales": 292932297,
                        "TrustBanksPurchases": 245322795,
                        "TrustBanksTotal": 538255092,
                        "TrustBanksBalance": -47609502,
                        "OtherFinancialInstitutionsSales": 22410692,
                        "OtherFinancialInstitutionsPurchases": 21764485,
                        "OtherFinancialInstitutionsTotal": 44175177,
                        "OtherFinancialInstitutionsBalance": -646207
                    }
                ],
                "pagination_key": "value1.value2."
            }
        "#;

        let response: TradingByInvestorTypePremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = TradingByInvestorTypePremiumPlanResponse {
            trades_spec: vec![TradingByInvestorTypePremiumPlan {
                common: TradingByInvestorTypeCommon {
                    published_date: "2017-01-13".to_string(),
                    start_date: "2017-01-04".to_string(),
                    end_date: "2017-01-06".to_string(),
                    section: "TSE1st".to_string(),
                    proprietary_sales: 1311271004.0,
                    proprietary_purchases: 1453326508.0,
                    proprietary_total: 2764597512.0,
                    proprietary_balance: 142055504.0,
                    brokerage_sales: 7165529005.0,
                    brokerage_purchases: 7030019854.0,
                    brokerage_total: 14195548859.0,
                    brokerage_balance: -135509151.0,
                    total_sales: 8476800009.0,
                    total_purchases: 8483346362.0,
                    total_total: 16960146371.0,
                    total_balance: 6546353.0,
                    individuals_sales: 1401711615.0,
                    individuals_purchases: 1161801155.0,
                    individuals_total: 2563512770.0,
                    individuals_balance: -239910460.0,
                    foreigners_sales: 5094891735.0,
                    foreigners_purchases: 5317151774.0,
                    foreigners_total: 10412043509.0,
                    foreigners_balance: 222260039.0,
                    securities_cos_sales: 76381455.0,
                    securities_cos_purchases: 61700100.0,
                    securities_cos_total: 138081555.0,
                    securities_cos_balance: -14681355.0,
                    investment_trusts_sales: 168705109.0,
                    investment_trusts_purchases: 124389642.0,
                    investment_trusts_total: 293094751.0,
                    investment_trusts_balance: -44315467.0,
                    business_cos_sales: 71217959.0,
                    business_cos_purchases: 63526641.0,
                    business_cos_total: 134744600.0,
                    business_cos_balance: -7691318.0,
                    other_cos_sales: 10745152.0,
                    other_cos_purchases: 15687836.0,
                    other_cos_total: 26432988.0,
                    other_cos_balance: 4942684.0,
                    insurance_cos_sales: 15926202.0,
                    insurance_cos_purchases: 9831555.0,
                    insurance_cos_total: 25757757.0,
                    insurance_cos_balance: -6094647.0,
                    city_bks_regional_bks_etc_sales: 10606789.0,
                    city_bks_regional_bks_etc_purchases: 8843871.0,
                    city_bks_regional_bks_etc_total: 19450660.0,
                    city_bks_regional_bks_etc_balance: -1762918.0,
                    trust_banks_sales: 292932297.0,
                    trust_banks_purchases: 245322795.0,
                    trust_banks_total: 538255092.0,
                    trust_banks_balance: -47609502.0,
                    other_financial_institutions_sales: 22410692.0,
                    other_financial_institutions_purchases: 21764485.0,
                    other_financial_institutions_total: 44175177.0,
                    other_financial_institutions_balance: -646207.0,
                },
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_trades_spec_premium_plan_response_no_pagination_key() {
        let json = r#"
            {
                "trades_spec": [
                    {
                        "PublishedDate": "2017-01-13",
                        "StartDate": "2017-01-04",
                        "EndDate": "2017-01-06",
                        "Section": "TSE1st",
                        "ProprietarySales": 1311271004,
                        "ProprietaryPurchases": 1453326508,
                        "ProprietaryTotal": 2764597512,
                        "ProprietaryBalance": 142055504,
                        "BrokerageSales": 7165529005,
                        "BrokeragePurchases": 7030019854,
                        "BrokerageTotal": 14195548859,
                        "BrokerageBalance": -135509151,
                        "TotalSales": 8476800009,
                        "TotalPurchases": 8483346362,
                        "TotalTotal": 16960146371,
                        "TotalBalance": 6546353,
                        "IndividualsSales": 1401711615,
                        "IndividualsPurchases": 1161801155,
                        "IndividualsTotal": 2563512770,
                        "IndividualsBalance": -239910460,
                        "ForeignersSales": 5094891735,
                        "ForeignersPurchases": 5317151774,
                        "ForeignersTotal": 10412043509,
                        "ForeignersBalance": 222260039,
                        "SecuritiesCosSales": 76381455,
                        "SecuritiesCosPurchases": 61700100,
                        "SecuritiesCosTotal": 138081555,
                        "SecuritiesCosBalance": -14681355,
                        "InvestmentTrustsSales": 168705109,
                        "InvestmentTrustsPurchases": 124389642,
                        "InvestmentTrustsTotal": 293094751,
                        "InvestmentTrustsBalance": -44315467,
                        "BusinessCosSales": 71217959,
                        "BusinessCosPurchases": 63526641,
                        "BusinessCosTotal": 134744600,
                        "BusinessCosBalance": -7691318,
                        "OtherCosSales": 10745152,
                        "OtherCosPurchases": 15687836,
                        "OtherCosTotal": 26432988,
                        "OtherCosBalance": 4942684,
                        "InsuranceCosSales": 15926202,
                        "InsuranceCosPurchases": 9831555,
                        "InsuranceCosTotal": 25757757,
                        "InsuranceCosBalance": -6094647,
                        "CityBKsRegionalBKsEtcSales": 10606789,
                        "CityBKsRegionalBKsEtcPurchases": 8843871,
                        "CityBKsRegionalBKsEtcTotal": 19450660,
                        "CityBKsRegionalBKsEtcBalance": -1762918,
                        "TrustBanksSales": 292932297,
                        "TrustBanksPurchases": 245322795,
                        "TrustBanksTotal": 538255092,
                        "TrustBanksBalance": -47609502,
                        "OtherFinancialInstitutionsSales": 22410692,
                        "OtherFinancialInstitutionsPurchases": 21764485,
                        "OtherFinancialInstitutionsTotal": 44175177,
                        "OtherFinancialInstitutionsBalance": -646207
                    }
                ]
            }
        "#;

        let response: TradingByInvestorTypePremiumPlanResponse =
            serde_json::from_str(json).unwrap();
        let expected_response = TradingByInvestorTypePremiumPlanResponse {
            trades_spec: vec![TradingByInvestorTypePremiumPlan {
                common: TradingByInvestorTypeCommon {
                    published_date: "2017-01-13".to_string(),
                    start_date: "2017-01-04".to_string(),
                    end_date: "2017-01-06".to_string(),
                    section: "TSE1st".to_string(),
                    proprietary_sales: 1311271004.0,
                    proprietary_purchases: 1453326508.0,
                    proprietary_total: 2764597512.0,
                    proprietary_balance: 142055504.0,
                    brokerage_sales: 7165529005.0,
                    brokerage_purchases: 7030019854.0,
                    brokerage_total: 14195548859.0,
                    brokerage_balance: -135509151.0,
                    total_sales: 8476800009.0,
                    total_purchases: 8483346362.0,
                    total_total: 16960146371.0,
                    total_balance: 6546353.0,
                    individuals_sales: 1401711615.0,
                    individuals_purchases: 1161801155.0,
                    individuals_total: 2563512770.0,
                    individuals_balance: -239910460.0,
                    foreigners_sales: 5094891735.0,
                    foreigners_purchases: 5317151774.0,
                    foreigners_total: 10412043509.0,
                    foreigners_balance: 222260039.0,
                    securities_cos_sales: 76381455.0,
                    securities_cos_purchases: 61700100.0,
                    securities_cos_total: 138081555.0,
                    securities_cos_balance: -14681355.0,
                    investment_trusts_sales: 168705109.0,
                    investment_trusts_purchases: 124389642.0,
                    investment_trusts_total: 293094751.0,
                    investment_trusts_balance: -44315467.0,
                    business_cos_sales: 71217959.0,
                    business_cos_purchases: 63526641.0,
                    business_cos_total: 134744600.0,
                    business_cos_balance: -7691318.0,
                    other_cos_sales: 10745152.0,
                    other_cos_purchases: 15687836.0,
                    other_cos_total: 26432988.0,
                    other_cos_balance: 4942684.0,
                    insurance_cos_sales: 15926202.0,
                    insurance_cos_purchases: 9831555.0,
                    insurance_cos_total: 25757757.0,
                    insurance_cos_balance: -6094647.0,
                    city_bks_regional_bks_etc_sales: 10606789.0,
                    city_bks_regional_bks_etc_purchases: 8843871.0,
                    city_bks_regional_bks_etc_total: 19450660.0,
                    city_bks_regional_bks_etc_balance: -1762918.0,
                    trust_banks_sales: 292932297.0,
                    trust_banks_purchases: 245322795.0,
                    trust_banks_total: 538255092.0,
                    trust_banks_balance: -47609502.0,
                    other_financial_institutions_sales: 22410692.0,
                    other_financial_institutions_purchases: 21764485.0,
                    other_financial_institutions_total: 44175177.0,
                    other_financial_institutions_balance: -646207.0,
                },
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
