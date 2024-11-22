//! Central contract month flag.

use std::str::FromStr;

use serde::Deserialize;

/// Central contract month flag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "String")]
pub enum CentralContractMonthFlag {
    /// 0: Others
    Others,
    /// 1: Central contract month
    CentralContractMonth,
    /// Unknown value.
    Unknown(String),
}

impl FromStr for CentralContractMonthFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Others),
            "1" => Ok(Self::CentralContractMonth),
            _ => Ok(Self::Unknown(s.to_string())),
        }
    }
}
impl From<&str> for CentralContractMonthFlag {
    fn from(s: &str) -> Self {
        Self::from_str(s).expect("Failed to parse CentralContractMonthFlag")
    }
}
impl From<String> for CentralContractMonthFlag {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
