//! Custom error type module.

use thiserror::Error;

use crate::api::error_response::ErrorResponse;

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
}
