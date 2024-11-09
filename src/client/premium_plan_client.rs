//! Premium plan client implementation for JQuants API.

use crate::{
    api::{listed_info::ListedInfoStandardPlanResponse, JQuantsApiClient},
    JQuantsError,
};

/// Premium plan client for J-Quants API.
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

    /// Get the listed information.
    pub async fn get_listed_info(
        &mut self,
        code: &str,
        date: &str,
    ) -> Result<ListedInfoStandardPlanResponse, JQuantsError> {
        self.api_client.get_listed_info(code, date).await
    }
}
