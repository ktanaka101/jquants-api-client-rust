//! This module contains the authentication API.

use reqwest::Client;

use crate::{
    api::build_url, IdTokenRequest, IdTokenResponse, JQuantsError, JQuantsErrorResponse,
    RefreshTokenRequest, RefreshTokenResponse,
};

pub mod id_token;
pub mod refresh_token;

/// Get a refresh token from the Refresh Token (/token/auth_user) API.
pub(crate) async fn get_refresh_token_from_api(
    client: &Client,
    mail_address: &str,
    password: &str,
) -> Result<String, JQuantsError> {
    let url = build_url("token/auth_user");
    let request_body = RefreshTokenRequest {
        mail_address: mail_address.to_string(),
        password: password.to_string(),
    };

    let response = client.post(&url).json(&request_body).send().await?;
    let status = response.status();
    let status_code = status.as_u16();
    let text = response.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::OK {
        match serde_json::from_str::<RefreshTokenResponse>(&text) {
            Ok(data) => Ok(data.refresh_token),
            Err(_) => Err(JQuantsError::InvalidResponseFormat {
                status_code,
                body: text,
            }),
        }
    } else {
        match serde_json::from_str::<JQuantsErrorResponse>(&text) {
            Ok(error_response) => match status {
                reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::FORBIDDEN => {
                    Err(JQuantsError::InvalidCredentials {
                        body: error_response,
                        status_code,
                    })
                }
                _ => Err(JQuantsError::ApiError {
                    body: error_response,
                    status_code,
                }),
            },
            Err(_) => Err(JQuantsError::InvalidResponseFormat {
                status_code,
                body: text,
            }),
        }
    }
}

/// リフレッシュトークンを使用してAPI経由でIDトークンを取得
pub(crate) async fn get_id_token_from_api(
    client: &Client,
    refresh_token: &str,
) -> Result<String, JQuantsError> {
    let url = build_url("token/auth_refresh");
    let request_body = IdTokenRequest {
        refresh_token: refresh_token.to_string(),
    };
    let response = client.post(&url).query(&request_body).send().await?;
    let status = response.status();
    let status_code = status.as_u16();
    let text = response.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::OK {
        match serde_json::from_str::<IdTokenResponse>(&text) {
            Ok(data) => Ok(data.id_token),
            Err(_) => Err(JQuantsError::InvalidResponseFormat {
                status_code,
                body: text,
            }),
        }
    } else {
        match serde_json::from_str::<JQuantsErrorResponse>(&text) {
            Ok(error_response) => match status {
                reqwest::StatusCode::FORBIDDEN => Err(JQuantsError::IdTokenInvalidOrExpired {
                    body: error_response,
                    status_code,
                }),
                _ => Err(JQuantsError::ApiError {
                    body: error_response,
                    status_code,
                }),
            },
            Err(_) => Err(JQuantsError::InvalidResponseFormat {
                status_code,
                body: text,
            }),
        }
    }
}
