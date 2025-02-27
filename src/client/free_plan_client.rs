//! Free plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesFreePlanResponse},
        financial_statements::FinancialStatementsApi,
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoFreePlanResponse},
        JQuantsApiClient, JQuantsPlanClient,
    },
    EarningsCalendarApi, TradingCalendarApi,
};

/// Free plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     DailyStockPricesApi, EarningsCalendarApi, FinancialStatementsApi, JQuantsBuilder, JQuantsFreePlanClient,
///     JQuantsPlanClient, ListedIssueInfoApi, TradingCalendarApi, Paginatable
/// };
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsFreePlanClient::new_from_refresh_token("your_refresh_token".to_string());
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
///     // Get trading calendar.
///     let response = client.get_trading_calendar().send().await.unwrap();
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
pub struct JQuantsFreePlanClient {
    pub(crate) api_client: JQuantsApiClient,
}

impl JQuantsPlanClient for JQuantsFreePlanClient {
    fn new(api_client: JQuantsApiClient) -> Self {
        Self { api_client }
    }

    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsFreePlanClient {
    type Response = ListedIssueInfoFreePlanResponse;
}

impl DailyStockPricesApi for JQuantsFreePlanClient {
    type Response = DailyStockPricesFreePlanResponse;
}

impl TradingCalendarApi for JQuantsFreePlanClient {}

impl FinancialStatementsApi for JQuantsFreePlanClient {}

impl EarningsCalendarApi for JQuantsFreePlanClient {}
