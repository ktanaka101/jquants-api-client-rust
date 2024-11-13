//! Premium plan client implementation for JQuants API.

use crate::api::{
    listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoStandardPlanResponse},
    stock_prices::{StockPricesApi, StockPricesPremiumPlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Premium plan client for J-Quants API.
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
    type Response = ListedIssueInfoStandardPlanResponse;
}

impl StockPricesApi for JQuantsPremiumPlanClient {
    type Response = StockPricesPremiumPlanResponse;
}
