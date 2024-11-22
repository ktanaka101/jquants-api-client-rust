//! Put/Call division.

use serde::{Deserialize, Serialize};

/// Put/Call division.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PutCallDivision {
    /// 1: Put
    #[serde(rename = "1")]
    Put,
    /// 2: Call
    #[serde(rename = "2")]
    Call,
    /// Unknown value.
    #[serde(untagged)]
    Unknown(String),
}
