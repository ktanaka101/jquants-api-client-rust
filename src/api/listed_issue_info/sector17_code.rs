use serde::Deserialize;

/// Represents the 17 sector codes
///
/// See: https://jpx.gitbook.io/j-quants-ja/api-reference/listed_info/sector17code
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum Sector17Code {
    /// code: 1
    /// * en: FOODS
    /// * ja: 食品
    #[serde(rename = "1")]
    Foods,

    /// code: 2
    /// * en: ENERGY RESOURCES
    /// * ja: エネルギー資源
    #[serde(rename = "2")]
    EnergyResources,

    /// code: 3
    /// * en: CONSTRUCTION & MATERIALS
    /// * ja: 建設・資材
    #[serde(rename = "3")]
    ConstructionMaterials,

    /// code: 4
    /// * en: RAW MATERIALS & CHEMICALS
    /// * ja: 素材・化学
    #[serde(rename = "4")]
    RawMaterialsChemicals,

    /// code: 5
    /// * en: PHARMACEUTICAL
    /// * ja: 医薬品
    #[serde(rename = "5")]
    Pharmaceutical,

    /// code: 6
    /// * en: AUTOMOBILES & TRANSPORTATION EQUIPMENT
    /// * ja: 自動車・輸送機
    #[serde(rename = "6")]
    AutomobilesTransportationEquipment,

    /// code: 7
    /// * en: STEEL & NONFERROUS METALS
    /// * ja: 鉄鋼・非鉄
    #[serde(rename = "7")]
    SteelNonferrousMetals,

    /// code: 8
    /// * en: MACHINERY
    /// * ja: 機械
    #[serde(rename = "8")]
    Machinery,

    /// code: 9
    /// * en: ELECTRIC APPLIANCES & PRECISION INSTRUMENTS
    /// * ja: 電機・精密
    #[serde(rename = "9")]
    ElectricAppliancesPrecisionInstruments,

    /// code: 10
    /// * en: IT & SERVICES, OTHERS
    /// * ja: 情報通信・サービスその他
    #[serde(rename = "10")]
    ITServicesOthers,

    /// code: 11
    /// * en: ELECTRIC POWER & GAS
    /// * ja: 電気・ガス
    #[serde(rename = "11")]
    ElectricPowerGas,

    /// code: 12
    /// * en: TRANSPORTATION & LOGISTICS
    /// * ja:運輸・物流
    #[serde(rename = "12")]
    TransportationLogistics,

    /// code: 13
    /// * en: COMMERCIAL & WHOLESALE TRADE
    /// * ja: 商社・卸売
    #[serde(rename = "13")]
    CommercialWholesaleTrade,

    /// code: 14
    /// * en: RETAIL TRADE
    /// * ja: 小売
    #[serde(rename = "14")]
    RetailTrade,

    /// code: 15
    /// * en: BANKS
    /// * ja: 銀行
    #[serde(rename = "15")]
    Banks,

    /// code: 16
    /// * en: FINANCIALS (EX BANKS)
    /// * ja: 金融（除く銀行）
    #[serde(rename = "16")]
    FinancialsExBanks,

    /// code: 17
    /// * en: REAL ESTATE
    /// * ja: 不動産
    #[serde(rename = "17")]
    RealEstate,

    /// code: 99
    /// * en: OTHER
    /// * ja: その他
    #[serde(rename = "99")]
    Other,

    /// Unknown code
    /// Takes this value if new code is added to the API.
    /// Please use this value until enum variants are added via library updates.
    #[serde(untagged)]
    Unknown(String),
}

impl Sector17Code {
    /// Returns english name of the sector.
    ///
    /// The document's English expressions are hardcoded.
    /// See: https://jpx.gitbook.io/j-quants-en/api-reference/listed_info/sector17code
    pub fn en_name(&self) -> &'static str {
        match self {
            Sector17Code::Foods => "FOODS",
            Sector17Code::EnergyResources => "ENERGY RESOURCES",
            Sector17Code::ConstructionMaterials => "CONSTRUCTION & MATERIALS",
            Sector17Code::RawMaterialsChemicals => "RAW MATERIALS & CHEMICALS",
            Sector17Code::Pharmaceutical => "PHARMACEUTICAL",
            Sector17Code::AutomobilesTransportationEquipment => {
                "AUTOMOBILES & TRANSPORTATION EQUIPMENT"
            }
            Sector17Code::SteelNonferrousMetals => "STEEL & NONFERROUS METALS",
            Sector17Code::Machinery => "MACHINERY",
            Sector17Code::ElectricAppliancesPrecisionInstruments => {
                "ELECTRIC APPLIANCES & PRECISION INSTRUMENTS"
            }
            Sector17Code::ITServicesOthers => "IT & SERVICES, OTHERS",
            Sector17Code::ElectricPowerGas => "ELECTRIC POWER & GAS",
            Sector17Code::TransportationLogistics => "TRANSPORTATION & LOGISTICS",
            Sector17Code::CommercialWholesaleTrade => "COMMERCIAL & WHOLESALE TRADE",
            Sector17Code::RetailTrade => "RETAIL TRADE",
            Sector17Code::Banks => "BANKS",
            Sector17Code::FinancialsExBanks => "FINANCIALS (EX BANKS)",
            Sector17Code::RealEstate => "REAL ESTATE",
            Sector17Code::Other => "OTHER",
            Sector17Code::Unknown(_) => "UNKNOWN",
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
                { "Code": "1" },
                { "Code": "2" },
                { "Code": "99" },
                { "Code": "100" }
            ]
        });

        #[derive(Debug, Deserialize)]
        struct SectorInfo {
            #[serde(rename = "Code")]
            code: Sector17Code,
        }

        #[derive(Debug, Deserialize)]
        struct Root {
            sectors: Vec<SectorInfo>,
        }

        let root: Root = serde_json::from_value(json_data).unwrap();

        assert_eq!(root.sectors.len(), 4);

        assert_eq!(root.sectors[0].code, Sector17Code::Foods);
        assert_eq!(root.sectors[1].code, Sector17Code::EnergyResources);
        assert_eq!(root.sectors[2].code, Sector17Code::Other);
        assert_eq!(
            root.sectors[3].code,
            Sector17Code::Unknown("100".to_string())
        );
    }

    #[test]
    fn test_en() {
        assert_eq!(Sector17Code::Foods.en_name(), "FOODS");
        assert_eq!(
            Sector17Code::Unknown("100".to_string()).en_name(),
            "UNKNOWN"
        );
    }
}
