//! Types of documents.

use serde::{Deserialize, Serialize};

/// Types of documents.
///
/// [See Reference](https://jpx.gitbook.io/j-quants-en/api-reference/statements/typeofdocument)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeOfDocument {
    /// Financial Statements
    ///
    /// （Consolidated・JGAAP）
    #[serde(rename = "FYFinancialStatements_Consolidated_JP")]
    FYFinancialStatementsConsolidatedJP,

    /// Financial Statements
    ///
    /// （Consolidated・US GAAP）
    #[serde(rename = "FYFinancialStatements_Consolidated_US")]
    FYFinancialStatementsConsolidatedUS,

    /// Financial Statements
    ///
    /// （NonConsolidated・JGAAP）
    #[serde(rename = "FYFinancialStatements_NonConsolidated_JP")]
    FYFinancialStatementsNonConsolidatedJP,

    /// 1Q Financial Statements
    ///
    /// （Consolidated・JGAAP）
    #[serde(rename = "1QFinancialStatements_Consolidated_JP")]
    Q1FinancialStatementsConsolidatedJP,

    /// 1Q Financial Statements
    ///
    /// （Consolidated・US GAAP）
    #[serde(rename = "1QFinancialStatements_Consolidated_US")]
    Q1FinancialStatementsConsolidatedUS,

    /// 1Q Financial Statements
    ///
    /// （NonConsolidated・JGAAP）
    #[serde(rename = "1QFinancialStatements_NonConsolidated_JP")]
    Q1FinancialStatementsNonConsolidatedJP,

    /// 2Q Financial Statements
    ///
    /// （Consolidated・JGAAP）
    #[serde(rename = "2QFinancialStatements_Consolidated_JP")]
    Q2FinancialStatementsConsolidatedJP,

    /// 2Q Financial Statements
    ///
    /// （Consolidated・US GAAP）
    #[serde(rename = "2QFinancialStatements_Consolidated_US")]
    Q2FinancialStatementsConsolidatedUS,

    /// 2Q Financial Statements
    ///
    /// （NonConsolidated・JGAAP）
    #[serde(rename = "2QFinancialStatements_NonConsolidated_JP")]
    Q2FinancialStatementsNonConsolidatedJP,

    /// 3Q Financial Statements
    ///
    /// （Consolidated・JGAAP）
    #[serde(rename = "3QFinancialStatements_Consolidated_JP")]
    Q3FinancialStatementsConsolidatedJP,

    /// 3Q Financial Statements
    ///
    /// （Consolidated・US GAAP）
    #[serde(rename = "3QFinancialStatements_Consolidated_US")]
    Q3FinancialStatementsConsolidatedUS,

    /// 3Q Financial Statements
    ///
    /// （NonConsolidated・JGAAP）
    #[serde(rename = "3QFinancialStatements_NonConsolidated_JP")]
    Q3FinancialStatementsNonConsolidatedJP,

    /// Other Financial Statements
    ///
    /// （Consolidated・JGAAP）
    #[serde(rename = "OtherPeriodFinancialStatements_Consolidated_JP")]
    OtherPeriodFinancialStatementsConsolidatedJP,

    /// Other Financial Statements
    ///
    /// （Consolidated・US GAAP）
    #[serde(rename = "OtherPeriodFinancialStatements_Consolidated_US")]
    OtherPeriodFinancialStatementsConsolidatedUS,

    /// Other Financial Statements
    ///
    /// （NonConsolidated・JGAAP）
    #[serde(rename = "OtherPeriodFinancialStatements_NonConsolidated_JP")]
    OtherPeriodFinancialStatementsNonConsolidatedJP,

    /// Financial Statements
    ///
    /// （Consolidated・ＪＭＩＳ）
    #[serde(rename = "FYFinancialStatements_Consolidated_JMIS")]
    FYFinancialStatementsConsolidatedJMIS,

    /// 1Q Financial Statements
    ///
    /// （Consolidated・ＪＭＩＳ）
    #[serde(rename = "1QFinancialStatements_Consolidated_JMIS")]
    Q1FinancialStatementsConsolidatedJMIS,

    /// 2Q Financial Statements
    ///
    /// （Consolidated・ＪＭＩＳ）
    #[serde(rename = "2QFinancialStatements_Consolidated_JMIS")]
    Q2FinancialStatementsConsolidatedJMIS,

    /// 3Q Financial Statements
    ///
    /// （Consolidated・ＪＭＩＳ）
    #[serde(rename = "3QFinancialStatements_Consolidated_JMIS")]
    Q3FinancialStatementsConsolidatedJMIS,

    /// Other Financial Statements
    ///
    /// （Consolidated・ＪＭＩＳ）
    #[serde(rename = "OtherPeriodFinancialStatements_Consolidated_JMIS")]
    OtherPeriodFinancialStatementsConsolidatedJMIS,

    /// Financial Statements
    ///
    /// （NonConsolidated・ＩＦＲＳ）
    #[serde(rename = "FYFinancialStatements_NonConsolidated_IFRS")]
    FYFinancialStatementsNonConsolidatedIFRS,

    /// 1Q Financial Statements
    ///
    /// （NonConsolidated・ＩＦＲＳ）
    #[serde(rename = "1QFinancialStatements_NonConsolidated_IFRS")]
    Q1FinancialStatementsNonConsolidatedIFRS,

    /// 2Q Financial Statements
    ///
    /// （NonConsolidated・ＩＦＲＳ）
    #[serde(rename = "2QFinancialStatements_NonConsolidated_IFRS")]
    Q2FinancialStatementsNonConsolidatedIFRS,

    /// 3Q Financial Statements
    ///
    /// （NonConsolidated・ＩＦＲＳ）
    #[serde(rename = "3QFinancialStatements_NonConsolidated_IFRS")]
    Q3FinancialStatementsNonConsolidatedIFRS,

    /// Other Financial Statements
    ///
    /// （NonConsolidated・ＩＦＲＳ）
    #[serde(rename = "OtherPeriodFinancialStatements_NonConsolidated_IFRS")]
    OtherFinancialStatementsNonConsolidatedIFRS,

    /// Financial Statements
    ///
    /// （Consolidated・ＩＦＲＳ）
    #[serde(rename = "FYFinancialStatements_Consolidated_IFRS")]
    FYFinancialStatementsConsolidatedIFRS,

    /// 1Q Financial Statements
    ///
    /// （Consolidated・ＩＦＲＳ）
    #[serde(rename = "1QFinancialStatements_Consolidated_IFRS")]
    Q1FinancialStatementsConsolidatedIFRS,

    /// 2Q Financial Statements
    ///
    /// （Consolidated・ＩＦＲＳ）
    #[serde(rename = "2QFinancialStatements_Consolidated_IFRS")]
    Q2FinancialStatementsConsolidatedIFRS,

    /// 3Q Financial Statements
    ///
    /// （Consolidated・ＩＦＲＳ）
    #[serde(rename = "3QFinancialStatements_Consolidated_IFRS")]
    Q3FinancialStatementsConsolidatedIFRS,

    /// Other Period Financial Statements
    ///
    /// （Consolidated・ＩＦＲＳ）
    #[serde(rename = "OtherPeriodFinancialStatements_Consolidated_IFRS")]
    OtherPeriodFinancialStatementsConsolidatedIFRS,

    /// Financial Statements
    ///
    /// （NonConsolidated・Foreign Stocks）
    #[serde(rename = "FYFinancialStatements_NonConsolidated_Foreign")]
    FYFinancialStatementsNonConsolidatedForeign,

    /// 1Q Financial Statements
    ///
    /// （NonConsolidated・Foreign Stocks）
    #[serde(rename = "1QFinancialStatements_NonConsolidated_Foreign")]
    Q1FinancialStatementsNonConsolidatedForeign,

    /// 2Q Financial Statements
    ///
    /// （NonConsolidated・Foreign Stocks）
    #[serde(rename = "2QFinancialStatements_NonConsolidated_Foreign")]
    Q2FinancialStatementsNonConsolidatedForeign,

    /// 3Q Financial Statements
    ///
    /// （NonConsolidated・Foreign Stocks）
    #[serde(rename = "3QFinancialStatements_NonConsolidated_Foreign")]
    Q3FinancialStatementsNonConsolidatedForeign,

    /// Other Financial Statements
    ///
    /// （NonConsolidated・Foreign Stocks）
    #[serde(rename = "OtherFinancialStatements_NonConsolidated_Foreign")]
    OtherFinancialStatementsNonConsolidatedForeign,

    /// Financial Statements
    ///
    /// （Consolidated・Foreign Stocks）
    #[serde(rename = "FYFinancialStatements_Consolidated_Foreign")]
    FYFinancialStatementsConsolidatedForeign,

    /// 1Q Financial Statements
    ///
    /// （Consolidated・Foreign Stocks）
    #[serde(rename = "1QFinancialStatements_Consolidated_Foreign")]
    Q1FinancialStatementsConsolidatedForeign,

    /// 2Q Financial Statements
    ///
    /// （Consolidated・Foreign Stocks）
    #[serde(rename = "2QFinancialStatements_Consolidated_Foreign")]
    Q2FinancialStatementsConsolidatedForeign,

    /// 3Q Financial Statements
    ///
    /// （Consolidated・Foreign Stocks）
    #[serde(rename = "3QFinancialStatements_Consolidated_Foreign")]
    Q3FinancialStatementsConsolidatedForeign,

    /// Other Financial Statements
    ///
    /// （Consolidated・Foreign Stocks）
    #[serde(rename = "OtherFinancialStatements_Consolidated_Foreign")]
    OtherFinancialStatementsConsolidatedForeign,

    /// Financial Statements（REIT）
    #[serde(rename = "FYFinancialStatements_Consolidated_REIT")]
    FYFinancialStatementsConsolidatedREIT,

    /// Revision of dividend forecast
    #[serde(rename = "DividendForecastRevision")]
    DividendForecastRevision,

    /// Revision of performance forecast
    #[serde(rename = "EarnForecastRevision")]
    EarnForecastRevision,

    /// Revision of dividend forecast (REIT)
    #[serde(rename = "REITDividendForecastRevision")]
    REITDividendForecastRevision,

    /// Revision of performance forecast (REIT)
    #[serde(rename = "REITEarnForecastRevision")]
    REITEarnForecastRevision,

    /// Handles unexpected or unknown document types.
    #[serde(untagged)]
    Unknown(String),
}
