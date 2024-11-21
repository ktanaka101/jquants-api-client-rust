//! Accounting period.

use serde::{Deserialize, Serialize};

/// Accounting period.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountingPeriod {
    /// 1Q
    #[serde(rename = "1Q")]
    Q1,
    /// 2Q
    #[serde(rename = "2Q")]
    Q2,
    /// 3Q
    #[serde(rename = "3Q")]
    Q3,
    /// 4Q
    #[serde(rename = "4Q")]
    Q4,
    /// 5Q
    #[serde(rename = "5Q")]
    Q5,
    /// FY
    #[serde(rename = "FY")]
    FY,
}
