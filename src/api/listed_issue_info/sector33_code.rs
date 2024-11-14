use serde::Deserialize;

/// Represents the 33 sector codes.
///
/// See: https://jpx.gitbook.io/j-quants-ja/api-reference/listed_info/sector33code
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Sector33Code {
    /// code: 0050
    /// * en: Fishery, Agriculture & Forestry
    /// * ja: 水産・農林業
    #[serde(rename = "0050")]
    FisheryAgricultureForestry,

    /// code: 1050
    /// * en: Mining
    /// * ja: 鉱業
    #[serde(rename = "1050")]
    Mining,

    /// code: 2050
    /// * en: Construction
    /// * ja: 建設業
    #[serde(rename = "2050")]
    Construction,

    /// code: 3050
    /// * en: Foods
    /// * ja: 食料品
    #[serde(rename = "3050")]
    Foods,

    /// code: 3100
    /// * en: Textiles & Apparels
    /// * ja: 繊維製品
    #[serde(rename = "3100")]
    TextilesApparels,

    /// code: 3150
    /// * en: Pulp & Paper
    /// * ja: パルプ・紙
    #[serde(rename = "3150")]
    PulpPaper,

    /// code: 3200
    /// * en: Chemicals
    /// * ja: 化学
    #[serde(rename = "3200")]
    Chemicals,

    /// code: 3250
    /// * en: Pharmaceutical
    /// * ja: 医薬品
    #[serde(rename = "3250")]
    Pharmaceutical,

    /// code: 3300
    /// * en: Oil & Coal Products
    /// * ja: 石油･石炭製品
    #[serde(rename = "3300")]
    OilCoalProducts,

    /// code: 3350
    /// * en: Rubber Products
    /// * ja: ゴム製品
    #[serde(rename = "3350")]
    RubberProducts,

    /// code: 3400
    /// * en: Glass & Ceramics Products
    /// * ja: ガラス･土石製品
    #[serde(rename = "3400")]
    GlassCeramicsProducts,

    /// code: 3450
    /// * en: Iron & Steel
    /// * ja: 鉄鋼
    #[serde(rename = "3450")]
    IronSteel,

    /// code: 3500
    /// * en: Nonferrous Metals
    /// * ja: 非鉄金属
    #[serde(rename = "3500")]
    NonferrousMetals,

    /// code: 3550
    /// * en: Metal Products
    /// * ja: 金属製品
    #[serde(rename = "3550")]
    MetalProducts,

    /// code: 3600
    /// * en: Machinery
    /// * ja: 機械
    #[serde(rename = "3600")]
    Machinery,

    /// code: 3650
    /// * en: Electric Appliances
    /// * ja: 電気機器
    #[serde(rename = "3650")]
    ElectricAppliances,

    /// code: 3700
    /// * en: Transportation Equipment
    /// * ja: 輸送用機器
    #[serde(rename = "3700")]
    TransportationEquipment,

    /// code: 3750
    /// * en: Precision Instruments
    /// * ja: 精密機器
    #[serde(rename = "3750")]
    PrecisionInstruments,

    /// code: 3800
    /// * en: Other Products
    /// * ja: その他製品
    #[serde(rename = "3800")]
    OtherProducts,

    /// code: 4050
    /// * en: Electric Power & Gas
    /// * ja: 電気･ガス業
    #[serde(rename = "4050")]
    ElectricPowerGas,

    /// code: 5050
    /// * en: Land Transportation
    /// * ja: 陸運業
    #[serde(rename = "5050")]
    LandTransportation,

    /// code: 5100
    /// * en: Marine Transportation
    /// * ja: 海運業
    #[serde(rename = "5100")]
    MarineTransportation,

    /// code: 5150
    /// * en: Air Transportation
    /// * ja: 空運業
    #[serde(rename = "5150")]
    AirTransportation,

    /// code: 5200
    /// * en: Warehousing & Harbor Transportation Services
    /// * ja: 倉庫･運輸関連業
    #[serde(rename = "5200")]
    WarehousingHarborTransportationServices,

    /// code: 5250
    /// * en: Information & Communication
    /// * ja: 情報･通信業
    #[serde(rename = "5250")]
    InformationCommunication,

    /// code: 6050
    /// * en: Wholesale Trade
    /// * ja: 卸売業
    #[serde(rename = "6050")]
    WholesaleTrade,

    /// code: 6100
    /// * en: Retail Trade
    /// * ja: 小売業
    #[serde(rename = "6100")]
    RetailTrade,

    /// code: 7050
    /// * en: Banks
    /// * ja: 銀行業
    #[serde(rename = "7050")]
    Banks,

    /// code: 7100
    /// * en: Securities & Commodity Futures
    /// * ja: 証券･商品先物取引業
    #[serde(rename = "7100")]
    SecuritiesCommodityFutures,

    /// code: 7150
    /// * en: Insurance
    /// * ja: 保険業
    #[serde(rename = "7150")]
    Insurance,

    /// code: 7200
    /// * en: Other Financing Business
    /// * ja: その他金融業
    #[serde(rename = "7200")]
    OtherFinancingBusiness,

    /// code: 8050
    /// * en: Real Estate
    /// * ja: 不動産業
    #[serde(rename = "8050")]
    RealEstate,

    /// code: 9050
    /// * en: Services
    /// * ja: サービス業
    #[serde(rename = "9050")]
    Services,

    /// code: 9999
    /// * en: Other
    /// * ja: その他
    #[serde(rename = "9999")]
    Other,

    /// Unknown code
    /// Takes this value if new code is added to the API.
    /// Please use this value until enum variants are added via library updates.
    #[serde(untagged)]
    Unknown(String),
}

impl Sector33Code {
    /// English name of the sector.
    ///
    /// The document's English expressions are hardcoded.
    /// See: https://jpx.gitbook.io/j-quants-en/api-reference/listed_info/sector33code
    pub fn en_name(&self) -> &'static str {
        match self {
            Sector33Code::FisheryAgricultureForestry => "Fishery, Agriculture & Forestry",
            Sector33Code::Mining => "Mining",
            Sector33Code::Construction => "Construction",
            Sector33Code::Foods => "Foods",
            Sector33Code::TextilesApparels => "Textiles & Apparels",
            Sector33Code::PulpPaper => "Pulp & Paper",
            Sector33Code::Chemicals => "Chemicals",
            Sector33Code::Pharmaceutical => "Pharmaceutical",
            Sector33Code::OilCoalProducts => "Oil & Coal Products",
            Sector33Code::RubberProducts => "Rubber Products",
            Sector33Code::GlassCeramicsProducts => "Glass & Ceramics Products",
            Sector33Code::IronSteel => "Iron & Steel",
            Sector33Code::NonferrousMetals => "Nonferrous Metals",
            Sector33Code::MetalProducts => "Metal Products",
            Sector33Code::Machinery => "Machinery",
            Sector33Code::ElectricAppliances => "Electric Appliances",
            Sector33Code::TransportationEquipment => "Transportation Equipment",
            Sector33Code::PrecisionInstruments => "Precision Instruments",
            Sector33Code::OtherProducts => "Other Products",
            Sector33Code::ElectricPowerGas => "Electric Power & Gas",
            Sector33Code::LandTransportation => "Land Transportation",
            Sector33Code::MarineTransportation => "Marine Transportation",
            Sector33Code::AirTransportation => "Air Transportation",
            Sector33Code::WarehousingHarborTransportationServices => {
                "Warehousing & Harbor Transportation Services"
            }
            Sector33Code::InformationCommunication => "Information & Communication",
            Sector33Code::WholesaleTrade => "Wholesale Trade",
            Sector33Code::RetailTrade => "Retail Trade",
            Sector33Code::Banks => "Banks",
            Sector33Code::SecuritiesCommodityFutures => "Securities & Commodity Futures",
            Sector33Code::Insurance => "Insurance",
            Sector33Code::OtherFinancingBusiness => "Other Financing Business",
            Sector33Code::RealEstate => "Real Estate",
            Sector33Code::Services => "Services",
            Sector33Code::Other => "Other",
            Sector33Code::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sector_code33_deserialize() {
        let json_data = json!({
            "sectors": [
                { "Code": "0050", "Name": "水産・農林業" },
                { "Code": "1050", "Name": "鉱業" },
                { "Code": "9999", "Name": "その他" },
                { "Code": "10000", "Name": "未知のセクター" }
            ]
        });

        #[derive(Debug, Deserialize)]
        struct SectorInfo {
            #[serde(rename = "Code")]
            code: Sector33Code,
        }

        #[derive(Debug, Deserialize)]
        struct Root {
            sectors: Vec<SectorInfo>,
        }

        let root: Root = serde_json::from_value(json_data).unwrap();

        assert_eq!(root.sectors.len(), 4);

        assert_eq!(
            root.sectors[0].code,
            Sector33Code::FisheryAgricultureForestry
        );
        assert_eq!(root.sectors[1].code, Sector33Code::Mining);
        assert_eq!(root.sectors[2].code, Sector33Code::Other);
        assert_eq!(
            root.sectors[3].code,
            Sector33Code::Unknown("10000".to_string())
        );
    }

    #[test]
    fn test_en() {
        assert_eq!(
            Sector33Code::FisheryAgricultureForestry.en_name(),
            "Fishery, Agriculture & Forestry"
        );
        assert_eq!(
            Sector33Code::Unknown("10000".to_string()).en_name(),
            "Unknown"
        );
    }
}
