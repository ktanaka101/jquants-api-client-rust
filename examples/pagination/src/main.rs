//! This example demonstrates how to fetch paginated data using the `fetch_pages_stream`, `fetch_all`, and `fetch_all_and_merge` methods.
//!
//! - The `fetch_pages_stream` method returns a stream of paginated responses. The stream will yield each page of data as it is fetched. The stream will continue fetching pages until there are no more pages to fetch.
//! - The `fetch_all` method will fetch all pages and return a vector of responses.
//! - The `fetch_all_and_merge` method will fetch all pages and merge them into a single response.
//!
//! The example fetches daily stock prices for a specific stock code and date.
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
//! # JQuants API Reference
//!
//! - [JQuants API documentation](https://jpx.gitbook.io/j-quants-en)

use futures::stream::StreamExt;
use jquants_api_client::{
    DailyStockPricesApi, DailyStockPricesStandardPlanResponse, JQuantsError, JQuantsFreePlanClient,
    Paginatable,
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

    let client = JQuantsFreePlanClient::new_from_refresh_token(refresh_token);
    fetch_pages_stream(client).await?;
    // or
    // fetch_all(client).await?;
    // or
    // fetch_all_and_merge(client).await?;

    Ok(())
}

/// Fetch pages stream.
#[allow(dead_code)]
async fn fetch_pages_stream(client: JQuantsFreePlanClient) -> Result<(), JQuantsError> {
    let mut stream = client
        .get_daily_stock_prices()
        .code("27890")
        .date("2024-08-01")
        .fetch_pages_stream();
    while let Some(response) = stream.next().await {
        let response = response?;

        for daily_quote in response.daily_quotes {
            println!(
                "Date: {}, Code: {}",
                daily_quote.common.date, daily_quote.common.code,
            );
        }
        println!("Pagination key: {:?}", response.pagination_key);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            break;
        }
    }

    Ok(())
}

/// Fetch all pages.
#[allow(dead_code)]
async fn fetch_all(
    client: JQuantsFreePlanClient,
) -> Result<Vec<DailyStockPricesStandardPlanResponse>, JQuantsError> {
    client
        .get_daily_stock_prices()
        .code("27890")
        .date("2024-08-01")
        .fetch_all()
        .await
}

/// Fetch all pages and merge them.
#[allow(dead_code)]
async fn fetch_all_and_merge(
    client: JQuantsFreePlanClient,
) -> Result<DailyStockPricesStandardPlanResponse, JQuantsError> {
    client
        .get_daily_stock_prices()
        .code("27890")
        .date("2024-08-01")
        .fetch_all_and_merge()
        .await
}
