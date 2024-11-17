//! Standard plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesStandardPlanResponse},
        financial_statements::FinancialStatementsApi,
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoStandardPlanResponse},
        short_sale_by_sector::ShortSaleBySectorApi,
        weekly_margin_trading_outstandings::WeeklyMarginTradingOutstandingsApi,
        JQuantsApiClient, JQuantsPlanClient,
    },
    EarningsCalendarApi, IndexOptionPricesApi, IndicesApi, TopixPricesApi,
    TradingByInvestorTypeApi, TradingCalendarApi,
};

/// Standard plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     DailyStockPricesApi, EarningsCalendarApi, FinancialStatementsApi, IndexOptionPricesApi, IndicesApi,
///     JQuantsBuilder, JQuantsStandardPlanClient, ListedIssueInfoApi, ShortSaleBySectorApi, TopixPricesApi,
///     TradingByInvestorTypeApi, TradingCalendarApi, Paginatable, WeeklyMarginTradingOutstandingsApi
/// };
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsStandardPlanClient::new_from_refresh_token("your_refresh_token".to_string());
///
///     // Get listed issue information.
///     let response = client.get_listed_issue_info().send().await.unwrap();
///
///     // Get stock prices.
///     let response = client.get_daily_stock_prices().send().await.unwrap();
///
///     // Paginate stock prices.
///     let response = client.get_daily_stock_prices().fetch_all().await.unwrap();
///     let response = client.get_daily_stock_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_daily_stock_prices().fetch_pages_stream();
///
///     // Get trading by investor type.
///     let response = client.get_trading_by_investor_type().send().await.unwrap();
///
///     // Paginate trading by investor type.
///     let response = client.get_trading_by_investor_type().fetch_all().await.unwrap();
///     let response = client.get_trading_by_investor_type().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_trading_by_investor_type().fetch_pages_stream();
///
///     // Get margin trading outstandings.
///     let response = client.get_weekly_margin_trading_outstandings().send().await.unwrap();
///
///     // Paginate margin trading outstandings.
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all().await.unwrap();
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_weekly_margin_trading_outstandings().fetch_pages_stream();
///
///     // Get short sale value and ratio by sector.
///     let response = client.get_weekly_margin_trading_outstandings().send().await.unwrap();
///
///     // Paginate short sale value and ratio by sector.
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all().await.unwrap();
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_weekly_margin_trading_outstandings().fetch_pages_stream();
///
///     // Get trading calendar.
///     let response = client.get_trading_calendar().send().await.unwrap();
///
///     // Get indices.
///     let response = client.get_indices().send().await.unwrap();
///
///     // Paginate  indices.
///     let response = client.get_indices().fetch_all().await.unwrap();
///     let response = client.get_indices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_indices().fetch_pages_stream();
///
///     // Get TOPIX prices.
///     let response = client.get_topix_prices().send().await.unwrap();
///
///     // Paginate TOPIX prices.
///     let response = client.get_topix_prices().fetch_all().await.unwrap();
///     let response = client.get_topix_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_topix_prices().fetch_pages_stream();
///
///     // Get financial statements.
///     let response = client.get_financial_statements().send().await.unwrap();
///
///     // Paginate stock prices.
///     let response = client.get_financial_statements().fetch_all().await.unwrap();
///     let response = client.get_financial_statements().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_financial_statements().fetch_pages_stream();
///
///     // Get earnings calendar.
///     let response = client.get_earnings_calendar().send().await.unwrap();
///
///     // Paginate earnings calendar.
///     let response = client.get_earnings_calendar().fetch_all().await.unwrap();
///     let response = client.get_earnings_calendar().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_earnings_calendar().fetch_pages_stream();
///
///     // Get idnex option prices.
///     let response = client.get_index_option_prices("2024-08-01").send().await.unwrap();
///
///     // Paginate idnex option prices.
///     let response = client.get_index_option_prices("2024-08-01").fetch_all().await.unwrap();
///     let response = client.get_index_option_prices("2024-08-01").fetch_all_and_merge().await.unwrap();
///     let stream = client.get_index_option_prices("2024-08-01").fetch_pages_stream();
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsStandardPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsStandardPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsStandardPlanClient {
    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsStandardPlanClient {
    type Response = ListedIssueInfoStandardPlanResponse;
}

impl DailyStockPricesApi for JQuantsStandardPlanClient {
    type Response = DailyStockPricesStandardPlanResponse;
}

impl TradingByInvestorTypeApi for JQuantsStandardPlanClient {}

impl WeeklyMarginTradingOutstandingsApi for JQuantsStandardPlanClient {}

impl ShortSaleBySectorApi for JQuantsStandardPlanClient {}

impl TradingCalendarApi for JQuantsStandardPlanClient {}

impl IndicesApi for JQuantsStandardPlanClient {}

impl TopixPricesApi for JQuantsStandardPlanClient {}

impl FinancialStatementsApi for JQuantsStandardPlanClient {}

impl EarningsCalendarApi for JQuantsStandardPlanClient {}

impl IndexOptionPricesApi for JQuantsStandardPlanClient {}
