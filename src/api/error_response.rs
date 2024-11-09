//! The common error response definition for the API.

use std::{error::Error, fmt};

use serde::Deserialize;

/// The common error response definition for the API.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ErrorResponse {
    /// The error message.
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorResponse {}
