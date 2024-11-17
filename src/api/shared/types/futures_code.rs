//! Futures codes.

use serde::{Deserialize, Serialize};

/// Futures codes.
///
/// [See Reference](Add_your_reference_URL_here)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuturesCode {
    /// TOPIXF: TOPIX Futures (Large)
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "TOPIXF")]
    TOPIXF,

    /// TOPIXMF: TOPIX Futures (Mini)
    ///
    /// Data Period: Since June 16, 2008
    #[serde(rename = "TOPIXMF")]
    TOPIXMF,

    /// MOTF: TSE Mothers Index Futures
    ///
    /// Data Period: Since July 19, 2016
    #[serde(rename = "MOTF")]
    MOTF,

    /// NKVIF: Nikkei 225 VI Futures
    ///
    /// Data Period: Since February 27, 2012
    #[serde(rename = "NKVIF")]
    NKVIF,

    /// NKYDF: Nikkei 225 Dividend Index Futures
    ///
    /// Data Period: Since July 26, 2010
    #[serde(rename = "NKYDF")]
    NKYDF,

    /// NK225F: Nikkei 225 Futures (Large)
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "NK225F")]
    NK225F,

    /// NK225MF: Nikkei 225 Futures (Mini)
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "NK225MF")]
    NK225MF,

    /// JN400F: JPX-Nikkei Index 400 Futures
    ///
    /// Data Period: Since November 25, 2014
    #[serde(rename = "JN400F")]
    JN400F,

    /// REITF: TSE REIT Index Futures
    ///
    /// Data Period: Since June 16, 2008
    #[serde(rename = "REITF")]
    REITF,

    /// DJIAF: DJIA Futures
    ///
    /// Data Period: Since May 28, 2012
    #[serde(rename = "DJIAF")]
    DJIAF,

    /// JGBLF: 10-year JGB Futures (Large)
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "JGBLF")]
    JGBLF,

    /// NK225MCF: Nikkei 225 Futures (micro)
    ///
    /// Data Period: Since May 29, 2023
    #[serde(rename = "NK225MCF")]
    NK225MCF,

    /// TOA3MF: 3-Month TONA Futures
    ///
    /// Data Period: Since May 29, 2023
    #[serde(rename = "TOA3MF")]
    TOA3MF,

    /// Handles unexpected or unknown futures codes.
    #[serde(untagged)]
    Unknown(String),
}
