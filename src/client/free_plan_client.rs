//! Free plan client implementation for JQuants API.

use crate::api::{
    listed_info::{ListedIssueInfoApi, ListedIssueInfoFreePlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Free plan client for J-Quants API.
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
