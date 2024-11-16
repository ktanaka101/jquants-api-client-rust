//! Free plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesFreePlanResponse},
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoFreePlanResponse},
        JQuantsApiClient, JQuantsPlanClient,
    },
    TradingCalendarApi,
};

/// Free plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     DailyStockPricesApi, JQuantsBuilder, JQuantsFreePlanClient, ListedIssueInfoApi,
///     TradingCalendarApi, Paginatable
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
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsFreePlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsFreePlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsFreePlanClient {
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
