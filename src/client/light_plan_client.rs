//! Light plan client implementation for JQuants API.

use crate::api::{
    listed_info::{ListedInfoApi, ListedInfoFreePlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Light plan client for J-Quants API.
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

impl ListedInfoApi for JQuantsLightPlanClient {
    type Response = ListedInfoFreePlanResponse;
}
