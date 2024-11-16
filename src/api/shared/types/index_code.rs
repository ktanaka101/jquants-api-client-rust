//! Indices codes.

use serde::{Deserialize, Serialize};

/// Indices codes.
///
/// [See Reference](https://jpx.gitbook.io/j-quants-en/api-reference/indices/indices-codes)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndexCode {
    /// 0000: TOPIX
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0000")]
    TOPIX,

    /// 0001: Second Section Stock Price Index
    ///
    /// Data Period: From May 7, 2008, to April 1, 2022
    #[serde(rename = "0001")]
    SecondSectionStockPriceIndex,

    /// 0028: TOPIX Core30
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0028")]
    TOPIXCore30,

    /// 0029: TOPIX Large 70
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0029")]
    TOPIXLarge70,

    /// 002A: TOPIX 100
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "002A")]
    TOPIX100,

    /// 002B: TOPIX Mid400
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "002B")]
    TOPIXMid400,

    /// 002C: TOPIX 500
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "002C")]
    TOPIX500,

    /// 002D: TOPIX Small
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "002D")]
    TOPIXSmall,

    /// 002E: TOPIX 1000
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "002E")]
    TOPIX1000,

    /// 002F: TOPIX Small500
    ///
    /// Data Period: (O/H/L/C) Since October 9, 2018
    /// (Closing Price) Since September 3, 2018
    #[serde(rename = "002F")]
    TOPIXSmall500,

    /// 0040: Fishery, Agriculture & Forestry
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0040")]
    FisheryAgricultureForestry,

    /// 0041: Mining
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0041")]
    Mining,

    /// 0042: Construction
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0042")]
    Construction,

    /// 0043: Foods
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0043")]
    Foods,

    /// 0044: Textiles & Apparels
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0044")]
    TextilesApparels,

    /// 0045: Pulp & Paper
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0045")]
    PulpPaper,

    /// 0046: Chemicals
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0046")]
    Chemicals,

    /// 0047: Pharmaceutical
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0047")]
    Pharmaceutical,

    /// 0048: Oil & Coal Products
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0048")]
    OilCoalProducts,

    /// 0049: Rubber Products
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0049")]
    RubberProducts,

    /// 004A: Glass & Ceramics Products
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004A")]
    GlassCeramicsProducts,

    /// 004B: Iron & Steel
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004B")]
    IronSteel,

    /// 004C: Nonferrous Metals
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004C")]
    NonferrousMetals,

    /// 004D: Metal Products
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004D")]
    MetalProducts,

    /// 004E: Machinery
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004E")]
    Machinery,

    /// 004F: Electronic Appliances
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "004F")]
    ElectronicAppliances,

    /// 0050: Transportation Equipment
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0050")]
    TransportationEquipment,

    /// 0051: Precision Instruments
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0051")]
    PrecisionInstruments,

    /// 0052: Other Products
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0052")]
    OtherProducts,

    /// 0053: Electric Power & Gas
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0053")]
    ElectricPowerGas,

    /// 0054: Land Transportation
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0054")]
    LandTransportation,

    /// 0055: Marine Transportation
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0055")]
    MarineTransportation,

    /// 0056: Air Transportation
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0056")]
    AirTransportation,

    /// 0057: Warehousing and Harbor Transportation Service
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0057")]
    WarehousingHarborTransportationService,

    /// 0058: Information & Communication
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0058")]
    InformationCommunication,

    /// 0059: Wholesale Trade
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0059")]
    WholesaleTrade,

    /// 005A: Retail Trade
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005A")]
    RetailTrade,

    /// 005B: Banks
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005B")]
    Banks,

    /// 005C: Securities & Commodity Futures
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005C")]
    SecuritiesCommodityFutures,

    /// 005D: Insurance
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005D")]
    Insurance,

    /// 005E: Other Financing Business
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005E")]
    OtherFinancingBusiness,

    /// 005F: Real Estate
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "005F")]
    RealEstate,

    /// 0060: Services
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0060")]
    Services,

    /// 0070: Tokyo Stock Exchange Growth Market 250 Index (Formerly: Tokyo Stock Exchange Mothers Index)
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0070")]
    GrowthMarket250Index,

    /// 0075: REIT
    ///
    /// Data Period: Since May 7, 2008
    #[serde(rename = "0075")]
    REIT,

    /// 0080: TOPIX-17 FOODS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0080")]
    TOPIX17Foods,

    /// 0081: TOPIX-17 ENERGY RESOURCES
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0081")]
    TOPIX17EnergyResources,

    /// 0082: TOPIX-17 CONSTRUCTION & MATERIALS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0082")]
    TOPIX17ConstructionMaterials,

    /// 0083: TOPIX-17 RAW MATERIALS & CHEMICALS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0083")]
    TOPIX17RawMaterialsChemicals,

    /// 0084: TOPIX-17 PHARMACEUTICAL
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0084")]
    TOPIX17Pharmaceutical,

    /// 0085: TOPIX-17 AUTOMOBILES & TRANSPORTATION EQUIPMENT
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0085")]
    TOPIX17AutomobilesTransportationEquipment,

    /// 0086: TOPIX-17 STEEL & NONFERROUS METALS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0086")]
    TOPIX17SteelNonferrousMetals,

    /// 0087: TOPIX-17 MACHINERY
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0087")]
    TOPIX17Machinery,

    /// 0088: TOPIX-17 ELECTRIC APPLIANCES & PRECISION INSTRUMENTS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0088")]
    TOPIX17ElectricAppliancesPrecisionInstruments,

    /// 0089: TOPIX-17 IT & SERVICES, OTHERS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0089")]
    TOPIX17ITServicesOthers,

    /// 008A: TOPIX-17 ELECTRIC POWER & GAS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008A")]
    TOPIX17ElectricPowerGas,

    /// 008B: TOPIX-17 TRANSPORTATION & LOGISTICS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008B")]
    TOPIX17TransportationLogistics,

    /// 008C: TOPIX-17 COMMERCIAL & WHOLESALE TRADE
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008C")]
    TOPIX17CommercialWholesaleTrade,

    /// 008D: TOPIX-17 RETAIL TRADE
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008D")]
    TOPIX17RetailTrade,

    /// 008E: TOPIX-17 BANKS
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008E")]
    TOPIX17Banks,

    /// 008F: TOPIX-17 FINANCIALS(EX BANKS)
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "008F")]
    TOPIX17FinancialsExBanks,

    /// 0090: TOPIX-17 REAL ESTATE
    ///
    /// Data Period: Since February 2, 2009
    #[serde(rename = "0090")]
    TOPIX17RealEstate,

    /// 0091: JASDAQ INDEX
    ///
    /// Data Period: From May 7, 2008, to April 1, 2022
    #[serde(rename = "0091")]
    JASDAQIndex,

    /// 0500: Tokyo Stock Exchange Prime Market Index
    ///
    /// Data Period: Since June 27, 2022
    #[serde(rename = "0500")]
    PrimeMarketIndex,

    /// 0501: Tokyo Stock Exchange Standard Market Index
    ///
    /// Data Period: Since June 27, 2022
    #[serde(rename = "0501")]
    StandardMarketIndex,

    /// 0502: Tokyo Stock Exchange Growth Market Index
    ///
    /// Data Period: Since June 27, 2022
    #[serde(rename = "0502")]
    GrowthMarketIndex,

    /// 0503: JPX Prime 150 Index
    ///
    /// Data Period: (O/H/L/C) Since July 3, 2023
    /// (Closing Price) Since May 29, 2023
    #[serde(rename = "0503")]
    JPXPrime150Index,

    /// 8100: TOPIX Value
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "8100")]
    TOPIXValue,

    /// 812C: TOPIX 500 Value
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "812C")]
    TOPIX500Value,

    /// 812D: TOPIX Small Value
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "812D")]
    TOPIXSmallValue,

    /// 8200: TOPIX Growth
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "8200")]
    TOPIXGrowth,

    /// 822C: TOPIX 500 Growth
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "822C")]
    TOPIX500Growth,

    /// 822D: TOPIX Small Growth
    ///
    /// Data Period: Since February 9, 2009
    #[serde(rename = "822D")]
    TOPIXSmallGrowth,

    /// 8501: Tokyo Stock Exchange REIT Office Index
    ///
    /// Data Period: (O/H/L/C) Since March 8, 2010
    /// (Closing Price) Since March 1, 2010
    #[serde(rename = "8501")]
    REITOfficeIndex,

    /// 8502: Tokyo Stock Exchange REIT Residential Index
    ///
    /// Data Period: (O/H/L/C) Since March 8, 2010
    /// (Closing Price) Since March 1, 2010
    #[serde(rename = "8502")]
    REITResidentialIndex,

    /// 8503: Tokyo Stock Exchange REIT Retail & Logistics, Others Index
    ///
    /// Data Period: (O/H/L/C) Since March 8, 2010
    /// (Closing Price) Since March 1, 2010
    #[serde(rename = "8503")]
    REITRetailLogisticsOthersIndex,

    /// Handles unexpected or unknown index codes.
    #[serde(untagged)]
    Unknown(String),
}
