//! Emergency margin trigger division.

use std::str::FromStr;

use serde::Deserialize;

/// Emergency margin trigger division.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(from = "String")]
pub enum EmergencyMarginTriggerDivision {
    /// 001: When emergency margin is triggered
    Triggered,
    /// 002: When settlement price is calculated
    Calculated,
    /// Unknown value.
    Unknown(String),
}

impl FromStr for EmergencyMarginTriggerDivision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "001" => Ok(Self::Triggered),
            "002" => Ok(Self::Calculated),
            _ => Ok(Self::Unknown(s.to_string())),
        }
    }
}
impl From<&str> for EmergencyMarginTriggerDivision {
    fn from(s: &str) -> Self {
        Self::from_str(s).expect("Failed to parse EmergencyMarginTriggerDivision")
    }
}
impl From<String> for EmergencyMarginTriggerDivision {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
