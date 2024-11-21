//! Section name

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Section name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SectionName {
    /// TSE1st
    /// * en: First Section
    /// * ja: 市場一部
    #[serde(rename = "TSE1st")]
    FirstSection,

    /// TSE2nd
    /// * en: Second Section
    /// * ja: 市場二部
    #[serde(rename = "TSE2nd")]
    SecondSection,

    /// TSEMothers
    /// * en: Mothers
    /// * ja: マザーズ
    #[serde(rename = "TSEMothers")]
    Mothers,

    /// TSEJASDAQ
    /// * en: JASDAQ
    /// * ja: JASDAQ
    #[serde(rename = "TSEJASDAQ")]
    JASDAQ,

    /// TSEPrime
    /// * en: Prime
    /// * ja: プライム
    #[serde(rename = "TSEPrime")]
    Prime,

    /// TSEStandard
    /// * en: Standard
    /// * ja: スタンダード
    #[serde(rename = "TSEStandard")]
    Standard,

    /// TSEGrowth
    /// * en: Growth
    /// * ja: グロース
    #[serde(rename = "TSEGrowth")]
    Growth,

    /// TokyoNagoya
    /// * en: Tokyo and Nagoya Stock Exchange
    /// * ja: 東証および名証
    #[serde(rename = "TokyoNagoya")]
    TokyoNagoyaStockExchange,

    /// Unknown section name.
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for SectionName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SectionName::from(s))
    }
}
impl From<&str> for SectionName {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("Failed to deserialize SectionName")
    }
}
impl From<String> for SectionName {
    fn from(s: String) -> Self {
        SectionName::from(s.as_str())
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
            "sections": [
                { "Name": "TSE1st" },
                { "Name": "TSEMothers" },
                { "Name": "AAA" },
            ]
        });

        #[derive(Debug, Deserialize)]
        struct Section {
            #[serde(rename = "Name")]
            name: SectionName,
        }

        #[derive(Debug, Deserialize)]
        struct Root {
            sections: Vec<Section>,
        }

        let root: Root = serde_json::from_value(json_data).unwrap();

        assert_eq!(root.sections.len(), 3);

        assert_eq!(root.sections[0].name, SectionName::FirstSection);
        assert_eq!(root.sections[1].name, SectionName::Mothers);
        assert_eq!(
            root.sections[2].name,
            SectionName::Unknown("AAA".to_string())
        ); // Unknown code mapped to Unknown
    }
}
