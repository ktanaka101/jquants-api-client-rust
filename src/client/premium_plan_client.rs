//! Premium plan client implementation for JQuants API.

use crate::api::{
    listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoPremiumPlanResponse},
    morning_session_stock_prices::{
        MorningSessionStockPricesApi, MorningSessionStockPricesPremiumPlanResponse,
    },
    stock_prices::{StockPricesApi, StockPricesPremiumPlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Premium plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{JQuantsBuilder, JQuantsPremiumPlanClient, ListedIssueInfoApi, MorningSessionStockPricesApi, Paginatable, StockPricesApi};
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsPremiumPlanClient::new_from_refresh_token("your_refresh_token".to_string());
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
///     let stream = client.get_stock_prices().fetch_pages_stream();
///
///     // Get morning session stock prices.
///     let response = client.morning_session_stock_prices().send().await.unwrap();
///
///     // Paginate morning session stock prices.
///     let response = client.morning_session_stock_prices().fetch_all().await.unwrap();
///     let response = client.morning_session_stock_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.morning_session_stock_prices().fetch_pages_stream();
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsPremiumPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsPremiumPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsPremiumPlanClient {
    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsPremiumPlanClient {
    type Response = ListedIssueInfoPremiumPlanResponse;
}

impl StockPricesApi for JQuantsPremiumPlanClient {
    type Response = StockPricesPremiumPlanResponse;
}

impl MorningSessionStockPricesApi for JQuantsPremiumPlanClient {
    type Response = MorningSessionStockPricesPremiumPlanResponse;
}
