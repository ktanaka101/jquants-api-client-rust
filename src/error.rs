//! Custom error type module.

use thiserror::Error;

use crate::api::shared::responses::error_response::ErrorResponse;

/// Custom error type for JQuants API client.
/// This is a simple enum that wraps the reqwest::Error and ErrorResponse types.
#[derive(Error, Debug)]
pub enum JQuantsError {
    /// Error when making an HTTP request
    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error when parsing an HTTP response
    #[error("HTTP response error: {0}")]
    ErrorResponse(#[from] Box<ErrorResponse>),

    /// Invalid credentials provided
    #[error("Invalid credentials provided.")]
    InvalidCredentials,

    /// Invalid refresh token
    #[error("Invalid refresh token.")]
    InvalidRefreshToken,

    /// Refresh token has expired and re-authentication is required
    #[error("Refresh token has expired and re-authentication is required.")]
    RefreshTokenExpired,

    /// Bug error. This should never happen.
    #[error("BUG: {0}. Please report this issue.")]
    BugError(String),

    /// Unexpected error
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
