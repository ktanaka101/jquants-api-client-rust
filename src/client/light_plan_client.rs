//! Light plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesLightPlanResponse},
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoLightPlanResponse},
        JQuantsApiClient, JQuantsPlanClient,
    },
    TradingByInvestorTypeApi, TradingCalendarApi,
};

/// Light plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     DailyStockPricesApi, JQuantsBuilder, JQuantsLightPlanClient, ListedIssueInfoApi,
///     TradingByInvestorTypeApi, TradingCalendarApi, Paginatable
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
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsLightPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsLightPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsLightPlanClient {
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
