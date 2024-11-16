//! Market code

use serde::{Deserialize, Serialize};

/// Represents the market codes.
///
/// See: https://jpx.gitbook.io/j-quants-en/api-reference/listed_info/marketcode
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum MarketCode {
    /// 0101
    /// * en: 1st Section
    /// * ja: 東証一部
    #[serde(rename = "0101")]
    TSEFirstSection,

    /// 0102
    /// * en: 2nd Section
    /// * ja: 東証二部
    #[serde(rename = "0102")]
    TSESecondSection,

    /// 0104
    /// * en: Mothers
    /// * ja: マザーズ
    #[serde(rename = "0104")]
    Mothers,

    /// 0105
    /// * en: TOKYO PRO MARKET
    /// * ja: 東証 PRO Market
    #[serde(rename = "0105")]
    TokyoProMarket,

    /// 0106
    /// * en: JASDAQ Standard
    /// * ja: JASDAQ スタンダード
    #[serde(rename = "0106")]
    JasdaqStandard,

    /// 0107
    /// * en: JASDAQ Growth
    /// * ja: JASDAQ グロース
    #[serde(rename = "0107")]
    JasdaqGrowth,

    /// 0109
    /// * en: Other
    /// * ja: その他
    #[serde(rename = "0109")]
    Others,

    /// 0111
    /// * en: Prime
    /// * ja: プライム
    #[serde(rename = "0111")]
    Prime,

    /// 0112
    /// en: Standard
    /// ja: スタンダード
    #[serde(rename = "0112")]
    Standard,

    /// 0113
    /// * en: Growth
    /// * ja: グロース
    #[serde(rename = "0113")]
    Growth,

    /// Unknown market segment code.
    #[serde(untagged)]
    Unknown(String),
}

impl MarketCode {
    /// Returns the English name corresponding to the market segment code.
    ///
    /// The document's English expressions are hardcoded.
    /// See: https://jpx.gitbook.io/j-quants-en/api-reference/listed_info/marketcode
    pub fn en_name(&self) -> &'static str {
        match self {
            MarketCode::TSEFirstSection => "1st Section",
            MarketCode::TSESecondSection => "2nd Section",
            MarketCode::Mothers => "Mothers",
            MarketCode::TokyoProMarket => "TOKYO PRO MARKET",
            MarketCode::JasdaqStandard => "JASDAQ Standard",
            MarketCode::JasdaqGrowth => "JASDAQ Growth",
            MarketCode::Others => "Others",
            MarketCode::Prime => "Prime",
            MarketCode::Standard => "Standard",
            MarketCode::Growth => "Growth",
            MarketCode::Unknown(_) => "Unknown",
        }
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sector_code_deserialize() {
        let json_data = json!({
            "sectors": [
                { "Code": "0101" },
                { "Code": "0102" },
                { "Code": "0113" },
                { "Code": "0114" }
            ]
        });

        #[derive(Debug, Deserialize)]
        struct SectorInfo {
            #[serde(rename = "Code")]
            code: MarketCode,
        }

        #[derive(Debug, Deserialize)]
        struct Root {
            sectors: Vec<SectorInfo>,
        }

        let root: Root = serde_json::from_value(json_data).unwrap();

        assert_eq!(root.sectors.len(), 4);

        assert_eq!(root.sectors[0].code, MarketCode::TSEFirstSection);
        assert_eq!(root.sectors[1].code, MarketCode::TSESecondSection);
        assert_eq!(root.sectors[2].code, MarketCode::Growth);
        assert_eq!(
            root.sectors[3].code,
            MarketCode::Unknown("0114".to_string())
        ); // Unknown code mapped to Unknown
    }
}
