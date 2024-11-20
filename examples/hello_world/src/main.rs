//! This example demonstrates how to use the JQuants client to get the stock price of a symbol.
//!
//! # Required Environment Variables
//!
//! To run this example, you need to set the `JQUANTS_REFRESH_TOKEN` environment variable to your JQuants API key.
//!
//! ```sh
//! export JQUANTS_REFRESH_TOKEN=your-api-key // or Modify .env.local file
//! cargo run
//! ```
//!
//! # Obtaining an API Key
//!
//! You can obtain an Refresh Token by signing up at [J-Quants](https://jpx-jquants.com/).
//!
//! # Example output
//!
//! ```sh
//! Date: 2024-08-01, Code: 27890, CompanyName: カルラ, CompanyNameEnglish: Karula Co.,LTD., Sector17Code: RetailTrade, Sector17CodeName: 小売, Sector33Code: RetailTrade, Sector33CodeName: 小売業, ScaleCategory: -, MarketCode: Standard, MarketCodeName: スタンダード
//! ```
//!
//! # JQuants API Reference
//!
//! - [JQuants API documentation](https://jpx.gitbook.io/j-quants-en)

use jquants_api_client::{
    JQuantsBuilder, JQuantsPlanClient, JQuantsPremiumPlanClient, ListedIssueInfoApi,
};
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::from_filename(".env.local");
    let _ = dotenvy::dotenv();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let refresh_token =
        env::var("JQUANTS_REFRESH_TOKEN").expect("JQUANTS_REFRESH_TOKEN must be set");
    if refresh_token == "ThisIsMyRefreshToken" {
        panic!("Please set the JQUANTS_REFRESH_TOKEN environment variable to your JQuants refresh token");
    }

    let client = JQuantsPremiumPlanClient::new_from_refresh_token(refresh_token);
    let listed_issue_info_response = client
        .get_listed_issue_info()
        .code("2789")
        .date("2024-08-01")
        .send()
        .await?;

    for issue_info in listed_issue_info_response.info {
        println!(
            "Date: {}, Code: {}, CompanyName: {}, CompanyNameEnglish: {}, Sector17Code: {:?}, Sector17CodeName: {}, Sector33Code: {:?}, Sector33CodeName: {}, ScaleCategory: {}, MarketCode: {:?}, MarketCodeName: {}",
            issue_info.common.date,
            issue_info.common.code,
            issue_info.common.company_name,
            issue_info.common.company_name_english,
            issue_info.common.sector17_code,
            issue_info.common.sector17_code_name,
            issue_info.common.sector33_code,
            issue_info.common.sector33_code_name,
            issue_info.common.scale_category,
            issue_info.common.market_code,
            issue_info.common.market_code_name,
        );
    }

    Ok(())
}
