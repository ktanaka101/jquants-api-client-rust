//! Financial Statements Data API.

use serde::{Deserialize, Serialize};

use crate::{AccountingPeriod, TypeOfDocument};

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

/// Builder for Financial Statements Data API.
#[derive(Clone, Serialize)]
pub struct FinancialStatementsBuilder {
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

impl JQuantsBuilder<FinancialStatementsResponse> for FinancialStatementsBuilder {
    async fn send(self) -> Result<FinancialStatementsResponse, crate::JQuantsError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<FinancialStatementsResponse, crate::JQuantsError> {
        self.client.inner.get("fins/statements", self).await
    }
}

impl Paginatable<FinancialStatementsResponse> for FinancialStatementsBuilder {
    fn pagination_key(mut self, pagination_key: impl Into<String>) -> Self {
        self.pagination_key = Some(pagination_key.into());
        self
    }
}

impl FinancialStatementsBuilder {
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

/// Trait for Financial Statements Data API.
pub trait FinancialStatementsApi: JQuantsPlanClient {
    /// Get API builder for Financial Statements Data.
    ///
    /// Use [Financial Statements Data (/fins/statements) API](https://jpx.gitbook.io/j-quants-en/api-reference/statements)
    fn get_financial_statements(&self) -> FinancialStatementsBuilder {
        FinancialStatementsBuilder::new(self.get_api_client().clone())
    }
}

/// Financial Statements Data response.
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en/api-reference/statements)
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FinancialStatementsResponse {
    /// List of financial statements
    pub statements: Vec<FinancialStatementItem>,
    /// Pagination key for fetching next set of data
    pub pagination_key: Option<String>,
}

impl HasPaginationKey for FinancialStatementsResponse {
    fn get_pagination_key(&self) -> Option<&str> {
        self.pagination_key.as_deref()
    }
}

impl MergePage for FinancialStatementsResponse {
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError> {
        let mut page = page?;
        let mut merged = page.pop().unwrap();
        for p in page {
            merged.statements.extend(p.statements);
        }
        merged.pagination_key = None;

        Ok(merged)
    }
}



    /// Non-Consolidated Operating Profit at Fiscal Year End
    #[serde(rename = "NonConsolidatedOperatingProfit")]
    pub non_consolidated_operating_profit: String,

    /// Non-Consolidated Ordinary Profit at Fiscal Year End
    #[serde(rename = "NonConsolidatedOrdinaryProfit")]
    pub non_consolidated_ordinary_profit: String,
}

#[cfg(feature = "polars")]
impl FinancialStatementsResponse {
    /// Convert the response into a Polars DataFrame.
    pub fn into_polars(
        self,
    ) -> Result<polars::prelude::DataFrame, crate::polars_utils::IntoPolarsError> {
        use crate::polars_utils::build_categorical_column;
        use polars::prelude::*;

        let data = self.statements;

        let mut columns = Vec::new();

        // Define a macro to add columns
        macro_rules! add_column {
            ($name:expr, $field:expr, $type:expr) => {
                columns.push(Series::new(
                    $name,
                    data.iter().map(|item| &item.$field).collect::<Vec<_>>(),
                ).cast($type)?);
            };
        }

        // Add columns for each field
        add_column!("DisclosedDate", disclosed_date, &DataType::Date);
        add_column!("DisclosedTime", disclosed_time, &DataType::Utf8);
        add_column!("LocalCode", local_code, &DataType::Utf8);
        add_column!("DisclosureNumber", disclosure_number, &DataType::Utf8);
        add_column!("TypeOfDocument", type_of_document, &DataType::Categorical(None));
        add_column!("TypeOfCurrentPeriod", type_of_current_period, &DataType::Categorical(None));
        add_column!("CurrentPeriodStartDate", current_period_start_date, &DataType::Date);
        add_column!("CurrentPeriodEndDate", current_period_end_date, &DataType::Date);
        add_column!("CurrentFiscalYearStartDate", current_fiscal_year_start_date, &DataType::Date);
        add_column!("CurrentFiscalYearEndDate", current_fiscal_year_end_date, &DataType::Date);
        add_column!("NextFiscalYearStartDate", next_fiscal_year_start_date, &DataType::Date);
        add_column!("NextFiscalYearEndDate", next_fiscal_year_end_date, &DataType::Date);

        // Add numeric columns
        macro_rules! add_numeric_column {
            ($name:expr, $field:expr) => {
                columns.push(Series::new(
                    $name,
                    data.iter().map(|item| item.$field.parse::<f64>().ok()).collect::<Vec<_>>(),
                ));
            };
        }

        add_numeric_column!("NetSales", net_sales);
        add_numeric_column!("OperatingProfit", operating_profit);
        add_numeric_column!("OrdinaryProfit", ordinary_profit);
        add_numeric_column!("Profit", profit);
        add_numeric_column!("EarningsPerShare", earnings_per_share);
        add_numeric_column!("DilutedEarningsPerShare", diluted_earnings_per_share);
        add_numeric_column!("TotalAssets", total_assets);
        add_numeric_column!("Equity", equity);
        add_numeric_column!("EquityToAssetRatio", equity_to_asset_ratio);
        add_numeric_column!("BookValuePerShare", book_value_per_share);
        add_numeric_column!("CashFlowsFromOperatingActivities", cash_flows_from_operating_activities);
        add_numeric_column!("CashFlowsFromInvestingActivities", cash_flows_from_investing_activities);
        add_numeric_column!("CashFlowsFromFinancingActivities", cash_flows_from_financing_activities);
        add_numeric_column!("CashAndEquivalents", cash_and_equivalents);

        // Add remaining columns (you may need to adjust the types as needed)
        add_column!("MaterialChangesInSubsidiaries", material_changes_in_subsidiaries, &DataType::Utf8);
        add_column!("SignificantChangesInTheScopeOfConsolidation", significant_changes_in_the_scope_of_consolidation, &DataType::Utf8);
        add_column!("ChangesBasedOnRevisionsOfAccountingStandard", changes_based_on_revisions_of_accounting_standard, &DataType::Utf8);
        add_column!("ChangesOtherThanOnesBasedOnRevisionsOfAccountingStandard", changes_other_than_based_on_revisions_of_accounting_standard, &DataType::Utf8);
        add_column!("ChangesInAccountingEstimates", changes_in_accounting_estimates, &DataType::Utf8);
        add_column!("RetrospectiveRestatement", retrospective_restatement, &DataType::Utf8);

        let df = DataFrame::new(columns)?;
        Ok(df)
    }
}

    /// Non-Consolidated Profit at Fiscal Year End
    #[serde(rename = "NonConsolidatedProfit")]
    pub non_consolidated_profit: String,

    /// Non-Consolidated Earnings Per Share at Fiscal Year End
    #[serde(rename = "NonConsolidatedEarningsPerShare")]
    pub non_consolidated_earnings_per_share: String,

    /// Non-Consolidated Total Assets at Fiscal Year End
    #[serde(rename = "NonConsolidatedTotalAssets")]
    pub non_consolidated_total_assets: String,

    /// Non-Consolidated Equity at Fiscal Year End
    #[serde(rename = "NonConsolidatedEquity")]
    pub non_consolidated_equity: String,

    /// Non-Consolidated Equity to Asset Ratio at Fiscal Year End
    #[serde(rename = "NonConsolidatedEquityToAssetRatio")]
    pub non_consolidated_equity_to_asset_ratio: String,

    /// Non-Consolidated Book Value Per Share at Fiscal Year End
    #[serde(rename = "NonConsolidatedBookValuePerShare")]
    pub non_consolidated_book_value_per_share: String,

    /// Forecast Non-Consolidated Net Sales 2nd Quarter
    #[serde(rename = "ForecastNonConsolidatedNetSales2ndQuarter")]
    pub forecast_non_consolidated_net_sales_2nd_quarter: String,

    /// Forecast Non-Consolidated Operating Profit 2nd Quarter
    #[serde(rename = "ForecastNonConsolidatedOperatingProfit2ndQuarter")]
    pub forecast_non_consolidated_operating_profit_2nd_quarter: String,

    /// Forecast Non-Consolidated Ordinary Profit 2nd Quarter
    #[serde(rename = "ForecastNonConsolidatedOrdinaryProfit2ndQuarter")]
    pub forecast_non_consolidated_ordinary_profit_2nd_quarter: String,

    /// Forecast Non-Consolidated Profit 2nd Quarter
    #[serde(rename = "ForecastNonConsolidatedProfit2ndQuarter")]
    pub forecast_non_consolidated_profit_2nd_quarter: String,

    /// Forecast Non-Consolidated Earnings Per Share 2nd Quarter
    #[serde(rename = "ForecastNonConsolidatedEarningsPerShare2ndQuarter")]
    pub forecast_non_consolidated_earnings_per_share_2nd_quarter: String,

    /// Next Year Forecast Non-Consolidated Net Sales 2nd Quarter
    #[serde(rename = "NextYearForecastNonConsolidatedNetSales2ndQuarter")]
    pub next_year_forecast_non_consolidated_net_sales_2nd_quarter: String,

    /// Next Year Forecast Non-Consolidated Operating Profit 2nd Quarter
    #[serde(rename = "NextYearForecastNonConsolidatedOperatingProfit2ndQuarter")]
    pub next_year_forecast_non_consolidated_operating_profit_2nd_quarter: String,

    /// Next Year Forecast Non-Consolidated Ordinary Profit 2nd Quarter
    #[serde(rename = "NextYearForecastNonConsolidatedOrdinaryProfit2ndQuarter")]
    pub next_year_forecast_non_consolidated_ordinary_profit_2nd_quarter: String,

    /// Next Year Forecast Non-Consolidated Profit 2nd Quarter
    #[serde(rename = "NextYearForecastNonConsolidatedProfit2ndQuarter")]
    pub next_year_forecast_non_consolidated_profit_2nd_quarter: String,

    /// Next Year Forecast Non-Consolidated Earnings Per Share 2nd Quarter
    #[serde(rename = "NextYearForecastNonConsolidatedEarningsPerShare2ndQuarter")]
    pub next_year_forecast_non_consolidated_earnings_per_share_2nd_quarter: String,

    /// Forecast Non-Consolidated Net Sales at Fiscal Year End
    #[serde(rename = "ForecastNonConsolidatedNetSales")]
    pub forecast_non_consolidated_net_sales: String,

    /// Forecast Non-Consolidated Operating Profit at Fiscal Year End
    #[serde(rename = "ForecastNonConsolidatedOperatingProfit")]
    pub forecast_non_consolidated_operating_profit: String,

    /// Forecast Non-Consolidated Ordinary Profit at Fiscal Year End
    #[serde(rename = "ForecastNonConsolidatedOrdinaryProfit")]
    pub forecast_non_consolidated_ordinary_profit: String,

    /// Forecast Non-Consolidated Profit at Fiscal Year End
    #[serde(rename = "ForecastNonConsolidatedProfit")]
    pub forecast_non_consolidated_profit: String,

    /// Forecast Non-Consolidated Earnings Per Share at Fiscal Year End
    #[serde(rename = "ForecastNonConsolidatedEarningsPerShare")]
    pub forecast_non_consolidated_earnings_per_share: String,

    /// Next Year Forecast Non-Consolidated Net Sales at Fiscal Year End
    #[serde(rename = "NextYearForecastNonConsolidatedNetSales")]
    pub next_year_forecast_non_consolidated_net_sales: String,

    /// Next Year Forecast Non-Consolidated Operating Profit at Fiscal Year End
    #[serde(rename = "NextYearForecastNonConsolidatedOperatingProfit")]
    pub next_year_forecast_non_consolidated_operating_profit: String,

    /// Next Year Forecast Non-Consolidated Ordinary Profit at Fiscal Year End
    #[serde(rename = "NextYearForecastNonConsolidatedOrdinaryProfit")]
    pub next_year_forecast_non_consolidated_ordinary_profit: String,

    /// Next Year Forecast Non-Consolidated Profit at Fiscal Year End
    #[serde(rename = "NextYearForecastNonConsolidatedProfit")]
    pub next_year_forecast_non_consolidated_profit: String,

    /// Next Year Forecast Non-Consolidated Earnings Per Share at Fiscal Year End
    #[serde(rename = "NextYearForecastNonConsolidatedEarningsPerShare")]
    pub next_year_forecast_non_consolidated_earnings_per_share: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_financial_statements_response() {
        let json_data = r#"
        {
            "statements": [
                {
                    "DisclosedDate": "2023-01-30",
                    "DisclosedTime": "12:00:00",
                    "LocalCode": "86970",
                    "DisclosureNumber": "20230127594871",
                    "TypeOfDocument": "3QFinancialStatements_Consolidated_IFRS",
                    "TypeOfCurrentPeriod": "3Q",
                    "CurrentPeriodStartDate": "2022-04-01",
                    "CurrentPeriodEndDate": "2022-12-31",
                    "CurrentFiscalYearStartDate": "2022-04-01",
                    "CurrentFiscalYearEndDate": "2023-03-31",
                    "NextFiscalYearStartDate": "",
                    "NextFiscalYearEndDate": "",
                    "NetSales": "100529000000",
                    "OperatingProfit": "51765000000",
                    "OrdinaryProfit": "",
                    "Profit": "35175000000",
                    "EarningsPerShare": "66.76",
                    "DilutedEarningsPerShare": "",
                    "TotalAssets": "79205861000000",
                    "Equity": "320021000000",
                    "EquityToAssetRatio": "0.004",
                    "BookValuePerShare": "",
                    "CashFlowsFromOperatingActivities": "",
                    "CashFlowsFromInvestingActivities": "",
                    "CashFlowsFromFinancingActivities": "",
                    "CashAndEquivalents": "91135000000",
                    "ResultDividendPerShare1stQuarter": "",
                    "ResultDividendPerShare2ndQuarter": "26.0",
                    "ResultDividendPerShare3rdQuarter": "",
                    "ResultDividendPerShareFiscalYearEnd": "",
                    "ResultDividendPerShareAnnual": "",
                    "DistributionsPerUnit(REIT)": "",
                    "ResultTotalDividendPaidAnnual": "",
                    "ResultPayoutRatioAnnual": "",
                    "ForecastDividendPerShare1stQuarter": "",
                    "ForecastDividendPerShare2ndQuarter": "",
                    "ForecastDividendPerShare3rdQuarter": "",
                    "ForecastDividendPerShareFiscalYearEnd": "36.0",
                    "ForecastDividendPerShareAnnual": "62.0",
                    "ForecastDistributionsPerUnit(REIT)": "",
                    "ForecastTotalDividendPaidAnnual": "",
                    "ForecastPayoutRatioAnnual": "",
                    "NextYearForecastDividendPerShare1stQuarter": "",
                    "NextYearForecastDividendPerShare2ndQuarter": "",
                    "NextYearForecastDividendPerShare3rdQuarter": "",
                    "NextYearForecastDividendPerShareFiscalYearEnd": "",
                    "NextYearForecastDividendPerShareAnnual": "",
                    "NextYearForecastDistributionsPerUnit(REIT)": "",
                    "NextYearForecastPayoutRatioAnnual": "",
                    "ForecastNetSales2ndQuarter": "",
                    "ForecastOperatingProfit2ndQuarter": "",
                    "ForecastOrdinaryProfit2ndQuarter": "",
                    "ForecastProfit2ndQuarter": "",
                    "ForecastEarningsPerShare2ndQuarter": "",
                    "NextYearForecastNetSales2ndQuarter": "",
                    "NextYearForecastOperatingProfit2ndQuarter": "",
                    "NextYearForecastOrdinaryProfit2ndQuarter": "",
                    "NextYearForecastProfit2ndQuarter": "",
                    "NextYearForecastEarningsPerShare2ndQuarter": "",
                    "ForecastNetSales": "135000000000",
                    "ForecastOperatingProfit": "70000000000",
                    "ForecastOrdinaryProfit": "",
                    "ForecastProfit": "47000000000",
                    "ForecastEarningsPerShare": "89.22",
                    "NextYearForecastNetSales": "",
                    "NextYearForecastOperatingProfit": "",
                    "NextYearForecastOrdinaryProfit": "",
                    "NextYearForecastProfit": "",
                    "NextYearForecastEarningsPerShare": "",
                    "MaterialChangesInSubsidiaries": "None",
                    "ChangesBasedOnRevisionsOfAccountingStandard": "None",
                    "ChangesOtherThanOnesBasedOnRevisionsOfAccountingStandard": "None",
                    "ChangesInAccountingEstimates": "None",
                    "RetrospectiveRestatement": "None",
                    "NumberOfIssuedAndOutstandingSharesAtTheEndOfFiscalYearIncludingTreasuryStock": "527578000",
                    "NumberOfTreasuryStockAtTheEndOfFiscalYear": "1000",
                    "AverageNumberOfShares": "526800000",
                    "NonConsolidatedNetSales": "",
                    "NonConsolidatedOperatingProfit": "",
                    "NonConsolidatedOrdinaryProfit": "",
                    "NonConsolidatedProfit": "",
                    "NonConsolidatedEarningsPerShare": "",
                    "NonConsolidatedTotalAssets": "",
                    "NonConsolidatedEquity": "",
                    "NonConsolidatedEquityToAssetRatio": "",
                    "NonConsolidatedBookValuePerShare": "",
                    "ForecastNonConsolidatedNetSales2ndQuarter": "",
                    "ForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "ForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "ForecastNonConsolidatedProfit2ndQuarter": "",
                    "ForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "NextYearForecastNonConsolidatedNetSales2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "ForecastNonConsolidatedNetSales": "",
                    "ForecastNonConsolidatedOperatingProfit": "",
                    "ForecastNonConsolidatedOrdinaryProfit": "",
                    "ForecastNonConsolidatedProfit": "",
                    "ForecastNonConsolidatedEarningsPerShare": "",
                    "NextYearForecastNonConsolidatedNetSales": "",
                    "NextYearForecastNonConsolidatedOperatingProfit": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit": "",
                    "NextYearForecastNonConsolidatedProfit": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare": ""
                }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: FinancialStatementsResponse = serde_json::from_str(json_data).unwrap();

        assert_eq!(response.statements.len(), 1);
        assert_eq!(response.pagination_key, Some("value1.value2.".to_string()));

        let statement = &response.statements[0];
        assert_eq!(statement.disclosed_date, "2023-01-30");
        assert_eq!(statement.disclosed_time, "12:00:00");
        assert_eq!(statement.local_code, "86970");
        assert_eq!(statement.disclosure_number, "20230127594871");
        assert_eq!(statement.type_of_document, TypeOfDocument::_3QFinancialStatements_Consolidated_IFRS);
        assert_eq!(statement.type_of_current_period, AccountingPeriod::_3Q);
        assert_eq!(statement.current_period_start_date, "2022-04-01");
        assert_eq!(statement.current_period_end_date, "2022-12-31");
        assert_eq!(statement.current_fiscal_year_start_date, "2022-04-01");
        assert_eq!(statement.current_fiscal_year_end_date, "2023-03-31");
        assert_eq!(statement.next_fiscal_year_start_date, None);
        assert_eq!(statement.next_fiscal_year_end_date, None);
        assert_eq!(statement.net_sales, "100529000000");
        assert_eq!(statement.operating_profit, "51765000000");
        assert_eq!(statement.ordinary_profit, "");
        assert_eq!(statement.profit, "35175000000");
        assert_eq!(statement.earnings_per_share, "66.76");
        assert_eq!(statement.diluted_earnings_per_share, "");
        assert_eq!(statement.total_assets, "79205861000000");
        assert_eq!(statement.equity, "320021000000");
        assert_eq!(statement.equity_to_asset_ratio, "0.004");
        assert_eq!(statement.book_value_per_share, "");
        assert_eq!(statement.cash_flows_from_operating_activities, "");
        assert_eq!(statement.cash_flows_from_investing_activities, "");
        assert_eq!(statement.cash_flows_from_financing_activities, "");
        assert_eq!(statement.cash_and_equivalents, "91135000000");
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_into_polars() {
        use polars::prelude::*;

        let response = FinancialStatementsResponse {
            statements: vec![
                FinancialStatementItem {
                    disclosed_date: "2023-01-30".to_string(),
                    disclosed_time: "12:00:00".to_string(),
                    local_code: "86970".to_string(),
                    disclosure_number: "20230127594871".to_string(),
                    type_of_document: TypeOfDocument::_3QFinancialStatements_Consolidated_IFRS,
                    type_of_current_period: AccountingPeriod::_3Q,
                    current_period_start_date: "2022-04-01".to_string(),
                    current_period_end_date: "2022-12-31".to_string(),
                    current_fiscal_year_start_date: "2022-04-01".to_string(),
                    current_fiscal_year_end_date: "2023-03-31".to_string(),
                    next_fiscal_year_start_date: None,
                    next_fiscal_year_end_date: None,
                    net_sales: "100529000000".to_string(),
                    operating_profit: "51765000000".to_string(),
                    ordinary_profit: "".to_string(),
                    profit: "35175000000".to_string(),
                    earnings_per_share: "66.76".to_string(),
                    diluted_earnings_per_share: "".to_string(),
                    total_assets: "79205861000000".to_string(),
                    equity: "320021000000".to_string(),
                    equity_to_asset_ratio: "0.004".to_string(),
                    book_value_per_share: "".to_string(),
                    cash_flows_from_operating_activities: "".to_string(),
                    cash_flows_from_investing_activities: "".to_string(),
                    cash_flows_from_financing_activities: "".to_string(),
                    cash_and_equivalents: "91135000000".to_string(),
                    // Add other fields as needed
                    ..Default::default()
                }
            ],
            pagination_key: Some("value1.value2.".to_string()),
        };

        let df = response.into_polars().unwrap();

        assert_eq!(df.shape(), (1, 26)); // Adjust the number of columns as needed
        assert_eq!(df.column("DisclosedDate").unwrap().dtype(), &DataType::Date);
        assert_eq!(df.column("LocalCode").unwrap().dtype(), &DataType::Utf8);
        assert_eq!(df.column("NetSales").unwrap().dtype(), &DataType::Float64);
    }
}
                    "ChangesOtherThanOnesBasedOnRevisionsOfAccountingStandard": "false",
                    "ChangesInAccountingEstimates": "true",
                    "RetrospectiveRestatement": "",
                    "NumberOfIssuedAndOutstandingSharesAtTheEndOfFiscalYearIncludingTreasuryStock": "528578441",
                    "NumberOfTreasuryStockAtTheEndOfFiscalYear": "1861043",
                    "AverageNumberOfShares": "526874759",
                    "NonConsolidatedNetSales": "",
                    "NonConsolidatedOperatingProfit": "",
                    "NonConsolidatedOrdinaryProfit": "",
                    "NonConsolidatedProfit": "",
                    "NonConsolidatedEarningsPerShare": "",
                    "NonConsolidatedTotalAssets": "",
                    "NonConsolidatedEquity": "",
                    "NonConsolidatedEquityToAssetRatio": "",
                    "NonConsolidatedBookValuePerShare": "",
                    "ForecastNonConsolidatedNetSales2ndQuarter": "",
                    "ForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "ForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "ForecastNonConsolidatedProfit2ndQuarter": "",
                    "ForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "NextYearForecastNonConsolidatedNetSales2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "ForecastNonConsolidatedNetSales": "",
                    "ForecastNonConsolidatedOperatingProfit": "",
                    "ForecastNonConsolidatedOrdinaryProfit": "",
                    "ForecastNonConsolidatedProfit": "",
                    "ForecastNonConsolidatedEarningsPerShare": "",
                    "NextYearForecastNonConsolidatedNetSales": "",
                    "NextYearForecastNonConsolidatedOperatingProfit": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit": "",
                    "NextYearForecastNonConsolidatedProfit": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare": ""
              }
            ],
            "pagination_key": "value1.value2."
        }
        "#;

        let response: FinancialStatementsResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = FinancialStatementsResponse {
            statements: vec![FinancialStatementItem {
                disclosed_date: "2023-01-30".to_string(),
                disclosed_time: "12:00:00".to_string(),
                local_code: "86970".to_string(),
                disclosure_number: "20230127594871".to_string(),
                type_of_document: TypeOfDocument::Q3FinancialStatementsConsolidatedIFRS,
                type_of_current_period: AccountingPeriod::Q3,
                current_period_start_date: "2022-04-01".to_string(),
                current_period_end_date: "2022-12-31".to_string(),
                current_fiscal_year_start_date: "2022-04-01".to_string(),
                current_fiscal_year_end_date: "2023-03-31".to_string(),
                next_fiscal_year_start_date: None,
                next_fiscal_year_end_date: None,
                net_sales: "100529000000".to_string(),
                operating_profit: "51765000000".to_string(),
                ordinary_profit: "".to_string(),
                profit: "35175000000".to_string(),
                earnings_per_share: "66.76".to_string(),
                diluted_earnings_per_share: "".to_string(),
                total_assets: "79205861000000".to_string(),
                equity: "320021000000".to_string(),
                equity_to_asset_ratio: "0.004".to_string(),
                book_value_per_share: "".to_string(),
                cash_flows_from_operating_activities: "".to_string(),
                cash_flows_from_investing_activities: "".to_string(),
                cash_flows_from_financing_activities: "".to_string(),
                cash_and_equivalents: "91135000000".to_string(),
                result_dividend_per_share_1st_quarter: "".to_string(),
                result_dividend_per_share_2nd_quarter: "26.0".to_string(),
                result_dividend_per_share_3rd_quarter: "".to_string(),
                result_dividend_per_share_fiscal_year_end: "".to_string(),
                result_dividend_per_share_annual: "".to_string(),
                distributions_per_unit_reit: "".to_string(),
                result_total_dividend_paid_annual: "".to_string(),
                result_payout_ratio_annual: "".to_string(),
                forecast_dividend_per_share_1st_quarter: "".to_string(),
                forecast_dividend_per_share_2nd_quarter: "".to_string(),
                forecast_dividend_per_share_3rd_quarter: "".to_string(),
                forecast_dividend_per_share_fiscal_year_end: "36.0".to_string(),
                forecast_dividend_per_share_annual: "62.0".to_string(),
                forecast_distributions_per_unit_reit: "".to_string(),
                forecast_total_dividend_paid_annual: "".to_string(),
                forecast_payout_ratio_annual: "".to_string(),
                next_year_forecast_dividend_per_share_1st_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_3rd_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_fiscal_year_end: "".to_string(),
                next_year_forecast_dividend_per_share_annual: "".to_string(),
                next_year_forecast_distributions_per_unit_reit: "".to_string(),
                next_year_forecast_payout_ratio_annual: "".to_string(),
                forecast_net_sales_2nd_quarter: "".to_string(),
                forecast_operating_profit_2nd_quarter: "".to_string(),
                forecast_ordinary_profit_2nd_quarter: "".to_string(),
                forecast_profit_2nd_quarter: "".to_string(),
                forecast_earnings_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_net_sales_2nd_quarter: "".to_string(),
                next_year_forecast_operating_profit_2nd_quarter: "".to_string(),
                next_year_forecast_ordinary_profit_2nd_quarter: "".to_string(),
                next_year_forecast_profit_2nd_quarter: "".to_string(),
                next_year_forecast_earnings_per_share_2nd_quarter: "".to_string(),
                forecast_net_sales: "132500000000".to_string(),
                forecast_operating_profit: "65500000000".to_string(),
                forecast_ordinary_profit: "".to_string(),
                forecast_profit: "45000000000".to_string(),
                forecast_earnings_per_share: "85.42".to_string(),
                next_year_forecast_net_sales: "".to_string(),
                next_year_forecast_operating_profit: "".to_string(),
                next_year_forecast_ordinary_profit: "".to_string(),
                next_year_forecast_profit: "".to_string(),
                next_year_forecast_earnings_per_share: "".to_string(),
                material_changes_in_subsidiaries: "false".to_string(),
                significant_changes_in_the_scope_of_consolidation: None,
                changes_based_on_revisions_of_accounting_standard: "false".to_string(),
                changes_other_than_based_on_revisions_of_accounting_standard: "false".to_string(),
                changes_in_accounting_estimates: "true".to_string(),
                retrospective_restatement: "".to_string(),
                number_of_issued_and_outstanding_shares_at_the_end_of_fiscal_year_including_treasury_stock: "528578441".to_string(),
                number_of_treasury_stock_at_the_end_of_fiscal_year: "1861043".to_string(),
                average_number_of_shares: "526874759".to_string(),
                non_consolidated_net_sales: "".to_string(),
                non_consolidated_operating_profit: "".to_string(),
                non_consolidated_ordinary_profit: "".to_string(),
                non_consolidated_profit: "".to_string(),
                non_consolidated_earnings_per_share: "".to_string(),
                non_consolidated_total_assets: "".to_string(),
                non_consolidated_equity: "".to_string(),
                non_consolidated_equity_to_asset_ratio: "".to_string(),
                non_consolidated_book_value_per_share: "".to_string(),
                forecast_non_consolidated_net_sales_2nd_quarter: "".to_string(),
                forecast_non_consolidated_operating_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_ordinary_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_earnings_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_net_sales_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_operating_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_ordinary_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_earnings_per_share_2nd_quarter: "".to_string(),
                forecast_non_consolidated_net_sales: "".to_string(),
                forecast_non_consolidated_operating_profit: "".to_string(),
                forecast_non_consolidated_ordinary_profit: "".to_string(),
                forecast_non_consolidated_profit: "".to_string(),
                forecast_non_consolidated_earnings_per_share: "".to_string(),
                next_year_forecast_non_consolidated_net_sales: "".to_string(),
                next_year_forecast_non_consolidated_operating_profit: "".to_string(),
                next_year_forecast_non_consolidated_ordinary_profit: "".to_string(),
                next_year_forecast_non_consolidated_profit: "".to_string(),
                next_year_forecast_non_consolidated_earnings_per_share: "".to_string(),
            }],
            pagination_key: Some("value1.value2.".to_string()),
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_financial_statements_response_no_pagination_key() {
        let json_data = r#"
        {
            "statements": [
                {
                    "DisclosedDate": "2023-01-30",
                    "DisclosedTime": "12:00:00",
                    "LocalCode": "86970",
                    "DisclosureNumber": "20230127594871",
                    "TypeOfDocument": "3QFinancialStatements_Consolidated_IFRS",
                    "TypeOfCurrentPeriod": "3Q",
                    "CurrentPeriodStartDate": "2022-04-01",
                    "CurrentPeriodEndDate": "2022-12-31",
                    "CurrentFiscalYearStartDate": "2022-04-01",
                    "CurrentFiscalYearEndDate": "2023-03-31",
                    "NextFiscalYearStartDate": "2024-11-01",
                    "NextFiscalYearEndDate": "2024-11-01",
                    "NetSales": "100529000000",
                    "OperatingProfit": "51765000000",
                    "OrdinaryProfit": "",
                    "Profit": "35175000000",
                    "EarningsPerShare": "66.76",
                    "DilutedEarningsPerShare": "",
                    "TotalAssets": "79205861000000",
                    "Equity": "320021000000",
                    "EquityToAssetRatio": "0.004",
                    "BookValuePerShare": "",
                    "CashFlowsFromOperatingActivities": "",
                    "CashFlowsFromInvestingActivities": "",
                    "CashFlowsFromFinancingActivities": "",
                    "CashAndEquivalents": "91135000000",
                    "ResultDividendPerShare1stQuarter": "",
                    "ResultDividendPerShare2ndQuarter": "26.0",
                    "ResultDividendPerShare3rdQuarter": "",
                    "ResultDividendPerShareFiscalYearEnd": "",
                    "ResultDividendPerShareAnnual": "",
                    "DistributionsPerUnit(REIT)": "",
                    "ResultTotalDividendPaidAnnual": "",
                    "ResultPayoutRatioAnnual": "",
                    "ForecastDividendPerShare1stQuarter": "",
                    "ForecastDividendPerShare2ndQuarter": "",
                    "ForecastDividendPerShare3rdQuarter": "",
                    "ForecastDividendPerShareFiscalYearEnd": "36.0",
                    "ForecastDividendPerShareAnnual": "62.0",
                    "ForecastDistributionsPerUnit(REIT)": "",
                    "ForecastTotalDividendPaidAnnual": "",
                    "ForecastPayoutRatioAnnual": "",
                    "NextYearForecastDividendPerShare1stQuarter": "",
                    "NextYearForecastDividendPerShare2ndQuarter": "",
                    "NextYearForecastDividendPerShare3rdQuarter": "",
                    "NextYearForecastDividendPerShareFiscalYearEnd": "",
                    "NextYearForecastDividendPerShareAnnual": "",
                    "NextYearForecastDistributionsPerUnit(REIT)": "",
                    "NextYearForecastPayoutRatioAnnual": "",
                    "ForecastNetSales2ndQuarter": "",
                    "ForecastOperatingProfit2ndQuarter": "",
                    "ForecastOrdinaryProfit2ndQuarter": "",
                    "ForecastProfit2ndQuarter": "",
                    "ForecastEarningsPerShare2ndQuarter": "",
                    "NextYearForecastNetSales2ndQuarter": "",
                    "NextYearForecastOperatingProfit2ndQuarter": "",
                    "NextYearForecastOrdinaryProfit2ndQuarter": "",
                    "NextYearForecastProfit2ndQuarter": "",
                    "NextYearForecastEarningsPerShare2ndQuarter": "",
                    "ForecastNetSales": "132500000000",
                    "ForecastOperatingProfit": "65500000000",
                    "ForecastOrdinaryProfit": "",
                    "ForecastProfit": "45000000000",
                    "ForecastEarningsPerShare": "85.42",
                    "NextYearForecastNetSales": "",
                    "NextYearForecastOperatingProfit": "",
                    "NextYearForecastOrdinaryProfit": "",
                    "NextYearForecastProfit": "",
                    "NextYearForecastEarningsPerShare": "",
                    "MaterialChangesInSubsidiaries": "false",
                    "SignificantChangesInTheScopeOfConsolidation": "2024-11-01",
                    "ChangesBasedOnRevisionsOfAccountingStandard": "false",
                    "ChangesOtherThanOnesBasedOnRevisionsOfAccountingStandard": "false",
                    "ChangesInAccountingEstimates": "true",
                    "RetrospectiveRestatement": "",
                    "NumberOfIssuedAndOutstandingSharesAtTheEndOfFiscalYearIncludingTreasuryStock": "528578441",
                    "NumberOfTreasuryStockAtTheEndOfFiscalYear": "1861043",
                    "AverageNumberOfShares": "526874759",
                    "NonConsolidatedNetSales": "",
                    "NonConsolidatedOperatingProfit": "",
                    "NonConsolidatedOrdinaryProfit": "",
                    "NonConsolidatedProfit": "",
                    "NonConsolidatedEarningsPerShare": "",
                    "NonConsolidatedTotalAssets": "",
                    "NonConsolidatedEquity": "",
                    "NonConsolidatedEquityToAssetRatio": "",
                    "NonConsolidatedBookValuePerShare": "",
                    "ForecastNonConsolidatedNetSales2ndQuarter": "",
                    "ForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "ForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "ForecastNonConsolidatedProfit2ndQuarter": "",
                    "ForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "NextYearForecastNonConsolidatedNetSales2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOperatingProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedProfit2ndQuarter": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare2ndQuarter": "",
                    "ForecastNonConsolidatedNetSales": "",
                    "ForecastNonConsolidatedOperatingProfit": "",
                    "ForecastNonConsolidatedOrdinaryProfit": "",
                    "ForecastNonConsolidatedProfit": "",
                    "ForecastNonConsolidatedEarningsPerShare": "",
                    "NextYearForecastNonConsolidatedNetSales": "",
                    "NextYearForecastNonConsolidatedOperatingProfit": "",
                    "NextYearForecastNonConsolidatedOrdinaryProfit": "",
                    "NextYearForecastNonConsolidatedProfit": "",
                    "NextYearForecastNonConsolidatedEarningsPerShare": ""
                }
            ]
        }
        "#;

        let response: FinancialStatementsResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = FinancialStatementsResponse {
            statements: vec![FinancialStatementItem {
                disclosed_date: "2023-01-30".to_string(),
                disclosed_time: "12:00:00".to_string(),
                local_code: "86970".to_string(),
                disclosure_number: "20230127594871".to_string(),
                type_of_document: TypeOfDocument::Q3FinancialStatementsConsolidatedIFRS,
                type_of_current_period: AccountingPeriod::Q3,
                current_period_start_date: "2022-04-01".to_string(),
                current_period_end_date: "2022-12-31".to_string(),
                current_fiscal_year_start_date: "2022-04-01".to_string(),
                current_fiscal_year_end_date: "2023-03-31".to_string(),
                next_fiscal_year_start_date: Some("2024-11-01".to_string()),
                next_fiscal_year_end_date: Some("2024-11-01".to_string()),
                net_sales: "100529000000".to_string(),
                operating_profit: "51765000000".to_string(),
                ordinary_profit: "".to_string(),
                profit: "35175000000".to_string(),
                earnings_per_share: "66.76".to_string(),
                diluted_earnings_per_share: "".to_string(),
                total_assets: "79205861000000".to_string(),
                equity: "320021000000".to_string(),
                equity_to_asset_ratio: "0.004".to_string(),
                book_value_per_share: "".to_string(),
                cash_flows_from_operating_activities: "".to_string(),
                cash_flows_from_investing_activities: "".to_string(),
                cash_flows_from_financing_activities: "".to_string(),
                cash_and_equivalents: "91135000000".to_string(),
                result_dividend_per_share_1st_quarter: "".to_string(),
                result_dividend_per_share_2nd_quarter: "26.0".to_string(),
                result_dividend_per_share_3rd_quarter: "".to_string(),
                result_dividend_per_share_fiscal_year_end: "".to_string(),
                result_dividend_per_share_annual: "".to_string(),
                distributions_per_unit_reit: "".to_string(),
                result_total_dividend_paid_annual: "".to_string(),
                result_payout_ratio_annual: "".to_string(),
                forecast_dividend_per_share_1st_quarter: "".to_string(),
                forecast_dividend_per_share_2nd_quarter: "".to_string(),
                forecast_dividend_per_share_3rd_quarter: "".to_string(),
                forecast_dividend_per_share_fiscal_year_end: "36.0".to_string(),
                forecast_dividend_per_share_annual: "62.0".to_string(),
                forecast_distributions_per_unit_reit: "".to_string(),
                forecast_total_dividend_paid_annual: "".to_string(),
                forecast_payout_ratio_annual: "".to_string(),
                next_year_forecast_dividend_per_share_1st_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_3rd_quarter: "".to_string(),
                next_year_forecast_dividend_per_share_fiscal_year_end: "".to_string(),
                next_year_forecast_dividend_per_share_annual: "".to_string(),
                next_year_forecast_distributions_per_unit_reit: "".to_string(),
                next_year_forecast_payout_ratio_annual: "".to_string(),
                forecast_net_sales_2nd_quarter: "".to_string(),
                forecast_operating_profit_2nd_quarter: "".to_string(),
                forecast_ordinary_profit_2nd_quarter: "".to_string(),
                forecast_profit_2nd_quarter: "".to_string(),
                forecast_earnings_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_net_sales_2nd_quarter: "".to_string(),
                next_year_forecast_operating_profit_2nd_quarter: "".to_string(),
                next_year_forecast_ordinary_profit_2nd_quarter: "".to_string(),
                next_year_forecast_profit_2nd_quarter: "".to_string(),
                next_year_forecast_earnings_per_share_2nd_quarter: "".to_string(),
                forecast_net_sales: "132500000000".to_string(),
                forecast_operating_profit: "65500000000".to_string(),
                forecast_ordinary_profit: "".to_string(),
                forecast_profit: "45000000000".to_string(),
                forecast_earnings_per_share: "85.42".to_string(),
                next_year_forecast_net_sales: "".to_string(),
                next_year_forecast_operating_profit: "".to_string(),
                next_year_forecast_ordinary_profit: "".to_string(),
                next_year_forecast_profit: "".to_string(),
                next_year_forecast_earnings_per_share: "".to_string(),
                material_changes_in_subsidiaries: "false".to_string(),
                significant_changes_in_the_scope_of_consolidation: Some("2024-11-01".to_string()),
                changes_based_on_revisions_of_accounting_standard: "false".to_string(),
                changes_other_than_based_on_revisions_of_accounting_standard: "false".to_string(),
                changes_in_accounting_estimates: "true".to_string(),
                retrospective_restatement: "".to_string(),
                number_of_issued_and_outstanding_shares_at_the_end_of_fiscal_year_including_treasury_stock: "528578441".to_string(),
                number_of_treasury_stock_at_the_end_of_fiscal_year: "1861043".to_string(),
                average_number_of_shares: "526874759".to_string(),
                non_consolidated_net_sales: "".to_string(),
                non_consolidated_operating_profit: "".to_string(),
                non_consolidated_ordinary_profit: "".to_string(),
                non_consolidated_profit: "".to_string(),
                non_consolidated_earnings_per_share: "".to_string(),
                non_consolidated_total_assets: "".to_string(),
                non_consolidated_equity: "".to_string(),
                non_consolidated_equity_to_asset_ratio: "".to_string(),
                non_consolidated_book_value_per_share: "".to_string(),
                forecast_non_consolidated_net_sales_2nd_quarter: "".to_string(),
                forecast_non_consolidated_operating_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_ordinary_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_profit_2nd_quarter: "".to_string(),
                forecast_non_consolidated_earnings_per_share_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_net_sales_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_operating_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_ordinary_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_profit_2nd_quarter: "".to_string(),
                next_year_forecast_non_consolidated_earnings_per_share_2nd_quarter: "".to_string(),
                forecast_non_consolidated_net_sales: "".to_string(),
                forecast_non_consolidated_operating_profit: "".to_string(),
                forecast_non_consolidated_ordinary_profit: "".to_string(),
                forecast_non_consolidated_profit: "".to_string(),
                forecast_non_consolidated_earnings_per_share: "".to_string(),
                next_year_forecast_non_consolidated_net_sales: "".to_string(),
                next_year_forecast_non_consolidated_operating_profit: "".to_string(),
                next_year_forecast_non_consolidated_ordinary_profit: "".to_string(),
                next_year_forecast_non_consolidated_profit: "".to_string(),
                next_year_forecast_non_consolidated_earnings_per_share: "".to_string(),
            }],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }

    #[test]
    fn test_deserialize_financial_statements_response_no_data() {
        let json_data = r#"
        {
            "statements": []
        }
        "#;

        let response: FinancialStatementsResponse = serde_json::from_str(json_data).unwrap();
        let expected_response = FinancialStatementsResponse {
            statements: vec![],
            pagination_key: None,
        };

        pretty_assertions::assert_eq!(response, expected_response);
    }
}
