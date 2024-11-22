//! Accounting period.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Accounting period.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum AccountingPeriod {
    /// 1Q
    Q1,
    /// 2Q
    Q2,
    /// 3Q
    Q3,
    /// 4Q
    Q4,
    /// 5Q
    Q5,
    /// FY
    FY,
    /// Unknown accounting period.
    Unknown(String),
}

impl FromStr for AccountingPeriod {
    type Err = String; // You can define a more specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1Q" => Ok(AccountingPeriod::Q1),
            "2Q" => Ok(AccountingPeriod::Q2),
            "3Q" => Ok(AccountingPeriod::Q3),
            "4Q" => Ok(AccountingPeriod::Q4),
            "5Q" => Ok(AccountingPeriod::Q5),
            "FY" => Ok(AccountingPeriod::FY),
            unknown => Ok(AccountingPeriod::Unknown(unknown.to_string())),
        }
    }
}
impl From<&str> for AccountingPeriod {
    fn from(s: &str) -> Self {
        Self::from_str(s).expect("Failed to parse AccountingPeriod")
    }
}
impl From<String> for AccountingPeriod {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
impl From<AccountingPeriod> for String {
    fn from(ap: AccountingPeriod) -> String {
        match ap {
            AccountingPeriod::Q1 => "1Q".to_string(),
            AccountingPeriod::Q2 => "2Q".to_string(),
            AccountingPeriod::Q3 => "3Q".to_string(),
            AccountingPeriod::Q4 => "4Q".to_string(),
            AccountingPeriod::Q5 => "5Q".to_string(),
            AccountingPeriod::FY => "FY".to_string(),
            AccountingPeriod::Unknown(unknown) => unknown,
        }
    }
}
