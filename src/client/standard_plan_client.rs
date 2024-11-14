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
