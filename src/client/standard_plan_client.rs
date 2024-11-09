//! Standard plan client implementation for JQuants API.

use crate::api::{
    listed_info::{ListedInfoApi, ListedInfoStandardPlanResponse},
    JQuantsApiClient, JQuantsPlanClient,
};

/// Standard plan client for J-Quants API.
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
    fn get_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }

    fn get_mut_client(&mut self) -> &mut JQuantsApiClient {
        &mut self.api_client
    }
}

impl ListedInfoApi for JQuantsStandardPlanClient {
    type Response = ListedInfoStandardPlanResponse;
}
