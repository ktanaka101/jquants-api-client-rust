//! The common error response definition for the JQuants API.

use std::{error::Error, fmt};

use serde::Deserialize;

/// The common error response definition for the JQuants API.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct JQuantsErrorResponse {
    /// The error message.
    pub message: String,
}

impl fmt::Display for JQuantsErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for JQuantsErrorResponse {}
