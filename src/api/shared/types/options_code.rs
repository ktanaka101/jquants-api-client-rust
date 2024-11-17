//! Options codes.

use serde::{Deserialize, Serialize};

/// Options codes.
///
/// [See Reference](Add_your_reference_URL_here)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionsCode {
    /// TOPIXE: TOPIX Options
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "TOPIXE")]
    TOPIXE,

    /// NK225E: Nikkei 225 Options
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "NK225E")]
    NK225E,

    /// JGBLFE: Options on 10-year JGB Futures
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "JGBLFE")]
    JGBLFE,

    /// EQOP: Securities Options
    ///
    /// Data Period: Since November 17, 2014
    #[serde(rename = "EQOP")]
    EQOP,

    /// NK225MWE: Nikkei 225 mini Options
    ///
    /// Data Period: Since May 29, 2023
    #[serde(rename = "NK225MWE")]
    NK225MWE,

    /// Handles unexpected or unknown options codes.
    #[serde(untagged)]
    Unknown(String),
}
