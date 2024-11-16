//! Issue Type Module for Margin Trading Outstandings API.

use serde::{Deserialize, Serialize};

/// Represents the classification of an issue.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum IssueType {
    /// 1: Margin issues
    #[serde(rename = "1")]
    Margin,
    /// 2: Loan issues
    #[serde(rename = "2")]
    Loan,
    /// 3: Other issues (non-loan, non-margin)
    #[serde(rename = "3")]
    Other,

    /// Unknown issue type
    #[serde(untagged)]
    Unknown(String),
}
