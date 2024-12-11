//! Dividend related types

use serde::{Deserialize, Serialize};

/// Code stands for dividend status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DevidendStatucCode {
    /// 1: new
    #[serde(rename = "1")]
    New,
    /// 2: revised
    #[serde(rename = "2")]
    Revised,
    /// 3: delete
    #[serde(rename = "3")]
    Delete,
    /// Unknown reference status code
    #[serde(untagged)]
    Unknown(String),
}

/// Code stands for interim/final dividend
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DividendInterimFinalCode {
    /// 1: interim
    #[serde(rename = "1")]
    Interim,
    /// 2: final
    #[serde(rename = "2")]
    Final,
    /// Unknown dividend interim/final code
    #[serde(untagged)]
    Unknown(String),
}

/// Code stands for determined/forecast
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DividendForecastResultCode {
    /// 1: result
    #[serde(rename = "1")]
    Determined,
    /// 2: forecast
    #[serde(rename = "2")]
    Forecast,
    /// Unknown dividend forecast result code
    #[serde(untagged)]
    Unknown(String),
}

/// Code stands for commemorative/special dividend
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DividendCommemorativeSpecialCode {
    /// 0: Normal
    #[serde(rename = "0")]
    Normal,
    /// 1: Commemorative
    #[serde(rename = "1")]
    Commemorative,
    /// 2: Special
    #[serde(rename = "2")]
    Special,
    /// 3: Both
    #[serde(rename = "3")]
    Both,
    /// Unknown dividend commemorative/special code
    #[serde(untagged)]
    Unknown(String),
}
