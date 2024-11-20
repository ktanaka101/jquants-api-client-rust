//! Custom error type module.

use serde::Deserialize;
use thiserror::Error;

use crate::api::shared::responses::error_response::JQuantsErrorResponse;

/// Common Error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Error message
    pub error_message: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status code: {}, Error message: {}",
            self.status_code, self.error_message
        )
    }
}

impl std::error::Error for ErrorResponse {}

/// Custom error type for JQuants API client.
/// This is a simple enum that wraps the reqwest::Error and ErrorResponse types.
#[derive(Error, Debug)]
pub enum JQuantsError {
    /// Invalid credentials provided.
    #[error("Invalid credentials provided. Status code: {status_code}, Body: {body}")]
    InvalidCredentials {
        /// HTTP status code
        status_code: u16,

        /// The error response
        body: JQuantsErrorResponse,
    },

    /// Id token is invalid or expired.
    #[error("ID token is invalid or expired. Status code: {status_code}, Message: {body}")]
    IdTokenInvalidOrExpired {
        /// HTTP status code
        status_code: u16,

        /// The error response
        body: JQuantsErrorResponse,
    },

    /// Status code is 400 ~ 599. Response format is JQuants error response.
    #[error("API error occurred. Status code: {status_code}, Message: {body}")]
    ApiError {
        /// HTTP status code
        status_code: u16,

        /// The error response. This is a JQuants error response format.
        body: JQuantsErrorResponse,
    },

    /// Response format is not JQuants error response.
    #[error("Invalid response format. Status code: {status_code}, Response body: {body}")]
    InvalidResponseFormat {
        /// HTTP status code
        status_code: u16,

        /// Response body
        body: String,
    },

    /// HTTP request error
    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Bug error. This should never happen.
    #[error("BUG: {0}. Please report this issue.")]
    BugError(String),
}
