//! This module contains all the API models.
//! The models are used to serialize and deserialize the data that is sent to and from the API.

pub mod error_response;
pub mod listed_info;
pub mod token_auth_refresh;
pub mod token_auth_user;

use std::fmt;

use crate::api::error_response::ErrorResponse;
use crate::api::token_auth_refresh::TokenAuthRefreshResponse;
use crate::error::JQuantsError;
use chrono::{DateTime, Local};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://api.jquants.com/v1";
/// Concatenate the base URL and the path.
///
/// `path` does not need to include a leading `/`.
///
/// # Example
///
/// ```rust
/// let path = "token/auth_refresh";
/// let url = build_url(path);
/// assert_eq!(url, "https://api.jquants.com/v1/token/auth_refresh");
/// ```
fn build_url(path: &str) -> String {
    format!("{}/{}", BASE_URL, path)
}

/// J-Quants API client
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en)
pub struct JQuantsApiClient {
    /// HTTP client
    client: Client,
    /// Refresh token
    /// Use this token to refresh the ID token.
    refresh_token: String,
    /// ID token
    id_token: Option<IdTokenWrapper>,
}

impl JQuantsApiClient {
    /// Create a new client from a refresh token.
    pub(crate) fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            client: Client::new(),
            refresh_token,
            id_token: None,
        }
    }

    /// Refresh the token.
    ///
    /// Use [ID Token (/token/auth_refresh) API](https://jpx.gitbook.io/j-quants-en/api-reference/idtoken)
    pub(crate) async fn refresh_token(&mut self) -> Result<(), JQuantsError> {
        let url = build_url("token/auth_refresh");
        let request = self
            .client
            .post(&url)
            .query(&[("refreshtoken", &self.refresh_token)]);

        let now = Local::now();
        let response = self
            .common_send::<TokenAuthRefreshResponse>(request)
            .await?;
        self.id_token = Some(IdTokenWrapper {
            id_token: response.id_token,
            updated_at: now,
        });

        Ok(())
    }

    /// Send a GET request to the API.
    pub(crate) async fn get<T: DeserializeOwned + fmt::Debug>(
        &mut self,
        path: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<T, JQuantsError> {
        let url = format!("{BASE_URL}/{}", path);
        let request = self.client.get(&url).query(&params);

        self.common_send_with_auth::<T>(request).await
    }

    /// Sends a common request with authentication.
    /// Reuses the ID token if it is valid.
    /// If the ID token is missing or invalid, it retrieves a new ID token.
    async fn common_send_with_auth<T: DeserializeOwned + fmt::Debug>(
        &mut self,
        request: RequestBuilder,
    ) -> Result<T, JQuantsError> {
        match self.id_token {
            Some(ref id_token) => {
                if id_token.is_valid() {
                    tracing::info!("Using cached ID token");
                } else {
                    tracing::info!("ID token expired. Refreshing...");
                    self.refresh_token().await?;
                }
            }
            _ => {
                tracing::info!("ID token is None. Refreshing...");
                self.refresh_token().await?;
            }
        }

        self.common_send(request.header(
            "Authorization",
            &format!(
                "Bearer {}",
                self.id_token.as_ref().expect("BUG: token is None").id_token
            ),
        ))
        .await
    }

    /// Send a request and parse the response.
    async fn common_send<T: DeserializeOwned + fmt::Debug>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, JQuantsError> {
        let response = request.send().await?;
        match response.status().as_u16() {
            200 => match response.json::<T>().await {
                Ok(json_data) => {
                    tracing::info!("Response: {json_data:?}");
                    Ok(json_data)
                }
                Err(e) => {
                    tracing::error!("Can't parse response to json: {:?}", e);
                    Err(JQuantsError::ReqwestError(e))
                }
            },
            _ => match response.json::<ErrorResponse>().await {
                Ok(error_response) => {
                    tracing::info!("Response error: {error_response:?}");
                    Err(JQuantsError::ErrorResponse(error_response.into()))
                }
                Err(e) => {
                    tracing::info!("Unknown response error: {e:?}");
                    Err(JQuantsError::ReqwestError(e))
                }
            },
        }
    }
}

/// ID Token wrapper
///
/// The ID token is valid for 24 hours.
pub(crate) struct IdTokenWrapper {
    /// ID Token
    id_token: String,
    /// ID Token updated at
    updated_at: DateTime<Local>,
}
impl IdTokenWrapper {
    /// Check if the ID token is valid.
    /// The ID token is valid for 24 hours.
    ///
    /// [Docs](https://jpx.gitbook.io/j-quants-en/api-reference/idtoken#attention)
    pub fn is_valid(&self) -> bool {
        let now = Local::now();
        let duration = now.signed_duration_since(self.updated_at);
        duration.num_hours() < 24
    }
}

/// Mask the ID token for security reasons.
/// If you want to display the ID token, do so at your own risk.
impl fmt::Debug for IdTokenWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.id_token.len();
        let masking_id_token = "*".repeat(len);

        f.debug_struct("IdTokenWrapper")
            .field("id_token", &masking_id_token)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}
