//! Price limits.

use serde::{Deserialize, Serialize};

/// Upper or Lower price limit.
///
/// [See Reference](https://jpx.gitbook.io/j-quants-en/api-reference/daily_quotes)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriceLimit {
    /// 0: Other than hitting the price limit
    #[serde(rename = "0")]
    NotHit,

    /// 1: Hitting the price limit
    #[serde(rename = "1")]
    Hit,

    /// Unknown price limit.
    #[serde(untagged)]
    Unknown(String),
}
