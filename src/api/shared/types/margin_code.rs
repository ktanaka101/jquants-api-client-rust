//! Margin codes.

use serde::{Deserialize, Serialize};

/// Margin codes.
///
/// [See Reference](https://jpx.gitbook.io/j-quants-en/api-reference/listed_info)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarginCode {
    /// 1: Margin issues
    #[serde(rename = "1")]
    MarginIssues,

    /// 2: Loan issues
    #[serde(rename = "2")]
    LoanIssues,

    /// 3: Other issues(non-loan, non-margin)
    #[serde(rename = "3")]
    OtherIssues,

    /// Handles unexpected or unknown margin codes.
    #[serde(untagged)]
    Unknown(String),
}
