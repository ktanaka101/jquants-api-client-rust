//! Underlying SSO type.

use serde::{Deserialize, Serialize};

/// Underlying SSO type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnderlyingSSO {
    /// "-": Other.
    #[serde(rename = "-")]
    Other,
    /// Issue codes of security options.
    #[serde(untagged)]
    IssueCode,
}
