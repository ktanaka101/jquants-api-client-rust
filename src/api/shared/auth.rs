//! This module contains the authentication API.

use reqwest::Client;

use crate::{
    api::build_url, ErrorResponse, IdTokenRequest, IdTokenResponse, JQuantsError,
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
    match response.status() {
        reqwest::StatusCode::OK => {
            let auth_response: RefreshTokenResponse = response.json().await?;
            Ok(auth_response.refresh_token)
        }
        reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::FORBIDDEN => {
            Err(JQuantsError::InvalidCredentials)
        }
        status => {
            let error_response: Result<ErrorResponse, _> = response.json().await;
            if let Ok(err) = error_response {
                Err(JQuantsError::ErrorResponse(Box::new(err)))
            } else {
                Err(JQuantsError::Unexpected(format!(
                    "Unexpected status code: {}",
                    status
                )))
            }
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

    match response.status() {
        reqwest::StatusCode::OK => {
            let id_response: IdTokenResponse = response.json().await?;
            Ok(id_response.id_token)
        }
        reqwest::StatusCode::BAD_REQUEST => Err(JQuantsError::InvalidRefreshToken),
        reqwest::StatusCode::FORBIDDEN => Err(JQuantsError::RefreshTokenExpired),
        status => {
            let error_response: Result<ErrorResponse, _> = response.json().await;
            if let Ok(err) = error_response {
                Err(JQuantsError::ErrorResponse(Box::new(err)))
            } else {
                Err(JQuantsError::Unexpected(format!(
                    "Unexpected status code: {}",
                    status
                )))
            }
        }
    }
}
