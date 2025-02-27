//! Light plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesLightPlanResponse},
        financial_statements::FinancialStatementsApi,
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoLightPlanResponse},
        JQuantsApiClient, JQuantsPlanClient,
    },
    EarningsCalendarApi, TopixPricesApi, TradingByInvestorTypeApi, TradingCalendarApi,
};

/// Light plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     DailyStockPricesApi, EarningsCalendarApi, FinancialStatementsApi, JQuantsBuilder, JQuantsLightPlanClient,
///     JQuantsPlanClient, ListedIssueInfoApi, TradingByInvestorTypeApi, TopixPricesApi, TradingCalendarApi,
///     Paginatable
/// };
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsLightPlanClient::new_from_refresh_token("your_refresh_token".to_string());
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
///     // Get trading calendar.
///     let response = client.get_trading_calendar().send().await.unwrap();
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
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsLightPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsPlanClient for JQuantsLightPlanClient {
    fn new(api_client: JQuantsApiClient) -> Self {
        Self { api_client }
    }

    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsLightPlanClient {
    type Response = ListedIssueInfoLightPlanResponse;
}

impl DailyStockPricesApi for JQuantsLightPlanClient {
    type Response = DailyStockPricesLightPlanResponse;
}

impl TradingByInvestorTypeApi for JQuantsLightPlanClient {}

impl TradingCalendarApi for JQuantsLightPlanClient {}

impl TopixPricesApi for JQuantsLightPlanClient {}

impl FinancialStatementsApi for JQuantsLightPlanClient {}

impl EarningsCalendarApi for JQuantsLightPlanClient {}
