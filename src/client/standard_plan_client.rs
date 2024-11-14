//! Standard plan client implementation for JQuants API.

use crate::api::{
    listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoStandardPlanResponse},
    morning_session_stock_prices::{
        MorningSessionStockPricesApi, MorningSessionStockPricesStandardPlanResponse,
    },
    stock_prices::{StockPricesApi, StockPricesStandardPlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Standard plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{JQuantsBuilder, JQuantsStandardPlanClient, ListedIssueInfoApi, MorningSessionStockPricesApi, Paginatable, StockPricesApi};
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsStandardPlanClient::new_from_refresh_token("your_refresh_token".to_string());
///
///     // Get listed issue information.
///     let response = client.get_listed_issue_info().send().await.unwrap();
///
///     // Get stock prices.
///     let response = client.get_stock_prices().send().await.unwrap();
///
///     // Paginate stock prices.
///     let response = client.get_stock_prices().fetch_all().await.unwrap();
///     let response = client.get_stock_prices().fetch_all_and_merge().await.unwrap();
///     let builder = client.get_stock_prices();
///     let stream = builder.fetch_pages_stream();
///
///     // Get morning session stock prices.
///     let response = client.morning_session_stock_prices().send().await.unwrap();
///
///     // Paginate morning session stock prices.
///     let response = client.morning_session_stock_prices().fetch_all().await.unwrap();
///     let response = client.morning_session_stock_prices().fetch_all_and_merge().await.unwrap();
///     let builder = client.morning_session_stock_prices();
///     let stream = builder.fetch_pages_stream();
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

impl StockPricesApi for JQuantsStandardPlanClient {
    type Response = StockPricesStandardPlanResponse;
}

impl MorningSessionStockPricesApi for JQuantsStandardPlanClient {
    type Response = MorningSessionStockPricesStandardPlanResponse;
}
