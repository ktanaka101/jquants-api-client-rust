//! This module contains all the API models.
//! The models are used to serialize and deserialize the data that is sent to and from the API.

pub mod breakdown_trading_data;
pub mod cash_dividend_data;
pub mod daily_stock_prices;
pub mod earnings_calendar;
pub mod financial_statement_details;
pub mod financial_statements;
pub mod futures_prices;
pub mod index_option_prices;
pub mod indicies;
pub mod listed_issue_info;
pub mod morning_session_stock_prices;
pub mod options_prices;
pub mod shared;
pub mod short_sale_by_sector;
pub mod topic_prices;
pub mod trading_by_type_of_investors;
pub mod trading_calendar;
pub mod weekly_margin_trading_outstandings;

use shared::{
    auth::{get_id_token_from_api, get_refresh_token_from_api},
    responses::error_response::JQuantsErrorResponse,
};
use std::{fmt, sync::Arc};
use tokio::sync::RwLock;

use crate::error::JQuantsError;
use chrono::{DateTime, Local};
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};

const BASE_URL: &str = "https://api.jquants.com/v1";
/// Concatenate the base URL and the path.
///
/// `path` does not need to include a leading `/`.
///
/// # Example
///
/// ```ignore
/// let path = "token/auth_refresh";
/// let url = build_url(path);
/// assert_eq!(url, "https://api.jquants.com/v1/token/auth_refresh");
/// ```
fn build_url(path: &str) -> String {
    format!("{}/{}", BASE_URL, path)
}

/// J-Quants API client trait
pub trait JQuantsPlanClient: Clone {
    /// Create a new client from an API client.
    fn new(api_client: JQuantsApiClient) -> Self;

    /// Create a new client from a refresh token.
    fn new_from_refresh_token(refresh_token: String) -> Self {
        let api_client = JQuantsApiClient::new_from_refresh_token(refresh_token);
        Self::new(api_client)
    }

    /// Create a new client from an account.
    fn new_from_account(
        mailaddress: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<Self, JQuantsError>> + Send {
        async {
            let api_client = JQuantsApiClient::new_from_account(mailaddress, password).await?;
            Ok(Self::new(api_client))
        }
    }

    /// Get the API client.
    fn get_api_client(&self) -> &JQuantsApiClient;

    /// Get a current refresh token.
    fn get_current_refresh_token(&self) -> impl std::future::Future<Output = String> + Send {
        let api_client = self.get_api_client().clone();
        async move {
            api_client
                .inner
                .token_set
                .read()
                .await
                .refresh_token
                .clone()
        }
    }

    /// Get a new refresh token from an account.
    /// But don't update the ID token in the client.
    ///
    /// Use `refresh_refresh_token` if you want to update the refresh token in the client.
    fn get_refresh_token_from_api(
        &self,
        mail_address: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<String, JQuantsError>> + Send {
        let api_client = self.get_api_client().clone();
        async move { get_refresh_token_from_api(&api_client.inner.client, mail_address, password).await }
    }

    /// Get a new ID token from a refresh token.
    /// But don't update the ID token in the client.
    ///
    /// Use `refresh_id_token` if you want to update the ID token in the client.
    fn get_id_token_from_api(
        &self,
        refresh_token: &str,
    ) -> impl std::future::Future<Output = Result<String, JQuantsError>> + Send {
        let api_client = self.get_api_client().clone();
        async move { get_id_token_from_api(&api_client.inner.client, refresh_token).await }
    }

    /// Renew the refresh token in the client.
    fn reset_refresh_token(
        &self,
        mail_address: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<(), JQuantsError>> + Send {
        let api_client = self.get_api_client().clone();
        async move {
            api_client
                .inner
                .reset_refresh_token(mail_address, password)
                .await
        }
    }

    /// Renew the ID token in the client.
    fn reset_id_token(&self) -> impl std::future::Future<Output = Result<(), JQuantsError>> + Send {
        let api_client = self.get_api_client().clone();
        async move { api_client.inner.reset_id_token().await }
    }

    /// Reauthenticate with a new refresh token and a new id token.
    fn reauthenticate(
        &self,
        mail_address: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<(), JQuantsError>> + Send {
        let api_client = self.get_api_client().clone();
        async move { api_client.inner.reset_tokens(mail_address, password).await }
    }
}

/// J-Quants API client
#[derive(Clone)]
pub struct JQuantsApiClient {
    inner: Arc<JQuantsApiClientRef>,
}
impl JQuantsApiClient {
    /// Create a new client from a refresh token.
    fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            inner: Arc::new(JQuantsApiClientRef::new_from_refresh_token(refresh_token)),
        }
    }

    /// Create a new client from an account.
    async fn new_from_account(mailaddress: &str, password: &str) -> Result<Self, JQuantsError> {
        let client_ref = JQuantsApiClientRef::new_from_account(mailaddress, password).await?;
        Ok(Self {
            inner: Arc::new(client_ref),
        })
    }
}

/// J-Quants API client
///
/// See: [API Reference](https://jpx.gitbook.io/j-quants-en)
pub(crate) struct JQuantsApiClientRef {
    /// HTTP client
    client: Client,
    /// Refresh token and ID token
    token_set: Arc<RwLock<TokenSet>>,
}

impl JQuantsApiClientRef {
    /// Create a new client from a refresh token.
    fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            client: Client::new(),
            token_set: Arc::new(RwLock::new(TokenSet {
                refresh_token,
                id_token: None,
            })),
        }
    }

    /// Create a new client from an account.
    async fn new_from_account(mailaddress: &str, password: &str) -> Result<Self, JQuantsError> {
        let client = Client::new();
        let refresh_token = get_refresh_token_from_api(&client, mailaddress, password).await?;
        let id_token = get_id_token_from_api(&client, &refresh_token).await?;

        let id_token_wrapper = IdTokenWrapper {
            id_token,
            updated_at: Local::now(),
        };

        Ok(Self {
            client,
            token_set: Arc::new(RwLock::new(TokenSet {
                refresh_token,
                id_token: Some(id_token_wrapper),
            })),
        })
    }

    /// Get a new refresh token from an account.
    async fn reset_refresh_token(
        &self,
        mail_address: &str,
        password: &str,
    ) -> Result<(), JQuantsError> {
        match get_refresh_token_from_api(&self.client, mail_address, password).await {
            Ok(new_refresh_token) => {
                let mut token_set_write = self.token_set.write().await;
                token_set_write.refresh_token = new_refresh_token;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Get a new ID token from a refresh token.
    async fn reset_id_token(&self) -> Result<(), JQuantsError> {
        let refresh_token = { self.token_set.read().await.refresh_token.clone() };
        match get_id_token_from_api(&self.client, &refresh_token).await {
            Ok(new_id_token) => {
                let mut token_set_write = self.token_set.write().await;
                token_set_write.id_token = Some(IdTokenWrapper {
                    id_token: new_id_token,
                    updated_at: Local::now(),
                });
                tracing::info!("ID token refreshed successfully.");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to refresh ID token: {:?}", e);
                Err(e)
            }
        }
    }

    /// Reset the refresh token if needed.
    async fn reset_id_token_if_needed(&self) -> Result<(), JQuantsError> {
        let needs_refresh = {
            let token_set = self.token_set.read().await;
            match &token_set.id_token {
                Some(token) => !token.is_valid(),
                None => true,
            }
        };

        if needs_refresh {
            tracing::info!("ID token is invalid or expired. Attempting to refresh.");
            match self.reset_id_token().await {
                Ok(_) => {
                    tracing::info!("Successfully refreshed ID token.");
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("Failed to refresh ID token: {:?}", e);
                    Err(e)
                }
            }
        } else {
            tracing::debug!("ID token is still valid.");
            Ok(())
        }
    }

    /// Reauthenticate with a new refresh token and a new id token.
    async fn reset_tokens(&self, mail_address: &str, password: &str) -> Result<(), JQuantsError> {
        // 再認証して新しいrefresh_tokenとid_tokenを取得
        let new_refresh_token =
            get_refresh_token_from_api(&self.client, mail_address, password).await?;
        let new_id_token = get_id_token_from_api(&self.client, &new_refresh_token).await?;

        {
            let mut token_set_write = self.token_set.write().await;
            token_set_write.refresh_token = new_refresh_token;
            token_set_write.id_token = Some(IdTokenWrapper {
                id_token: new_id_token,
                updated_at: Local::now(),
            });
        }

        Ok(())
    }

    /// Send a GET request to the API.
    /// The request is authenticated with the ID token.
    /// If the ID token is expired, it will be refreshed.
    /// If the refresh token is expired, it will return an error.
    async fn get<T: DeserializeOwned + fmt::Debug>(
        &self,
        path: &str,
        params: impl Serialize,
    ) -> Result<T, JQuantsError> {
        let url = format!("{BASE_URL}/{}", path);
        let request = self.client.get(&url).query(&params);

        self.common_send_and_refresh_token_if_needed::<T>(request)
            .await
    }

    /// Sends a common request and authentication if needed.
    async fn common_send_and_refresh_token_if_needed<T: DeserializeOwned + fmt::Debug>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, JQuantsError> {
        self.reset_id_token_if_needed().await?;

        let id_token = {
            self.token_set
                .read()
                .await
                .id_token
                .as_ref()
                .ok_or_else(|| {
                    tracing::error!("ID token not found.");
                    JQuantsError::BugError("ID token not found.".to_string())
                })?
                .id_token
                .clone()
        };
        self.common_send(request.header("Authorization", &format!("Bearer {id_token}")))
            .await
    }

    /// Send a request and parse the response.
    async fn common_send<T: DeserializeOwned + fmt::Debug>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, JQuantsError> {
        tracing::debug!("Sending API request: {:?}", request);
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        tracing::debug!("Received response with status: {}", status);

        if status.is_success() {
            match serde_json::from_str::<T>(&text) {
                Ok(data) => {
                    tracing::debug!("Successfully parsed response.");
                    Ok(data)
                }
                Err(_) => {
                    tracing::error!("Failed to parse response: {}", text);
                    Err(JQuantsError::InvalidResponseFormat {
                        status_code: status.as_u16(),
                        body: text,
                    })
                }
            }
        } else {
            match serde_json::from_str::<JQuantsErrorResponse>(&text) {
                Ok(error_response) => match status {
                    reqwest::StatusCode::UNAUTHORIZED => {
                        tracing::warn!("Received UNAUTHORIZED error: {:?}", error_response);
                        Err(JQuantsError::IdTokenInvalidOrExpired {
                            body: error_response,
                            status_code: status.as_u16(),
                        })
                    }
                    _ => {
                        tracing::error!(
                            "API error occurred: Status code {}, Body: {}",
                            status.as_u16(),
                            text
                        );
                        Err(JQuantsError::ApiError {
                            body: error_response,
                            status_code: status.as_u16(),
                        })
                    }
                },
                Err(_) => {
                    tracing::error!(
                        "Invalid response format: Status code {}, Body: {}",
                        status.as_u16(),
                        text
                    );
                    Err(JQuantsError::InvalidResponseFormat {
                        status_code: status.as_u16(),
                        body: text,
                    })
                }
            }
        }
    }
}

/// Token set
///
/// The refresh token is valid for one week and the ID token is valid for 24 hours.
pub(crate) struct TokenSet {
    /// Refresh token
    /// Use this token to refresh the ID token.
    refresh_token: String,
    /// ID token
    id_token: Option<IdTokenWrapper>,
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
    fn is_valid(&self) -> bool {
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
