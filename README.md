# J-Quants API Client

[![Build Status](https://github.com/ktanaka101/jquants-api-client-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/ktanaka101/jquants-api-client-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/jquants-api-client.svg)](https://crates.io/crates/jquants-api-client)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Table of Contents

- [J-Quants API Client](#j-quants-api-client)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [What is J-Quants?](#what-is-j-quants)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Supported APIs](#supported-apis)
    - [Authentication](#authentication)
    - [Data Retrieval](#data-retrieval)
  - [Client Usage Example](#client-usage-example)
    - [Standard Fetch](#standard-fetch)
    - [Pagination](#pagination)
    - [Additional Examples](#additional-examples)
  - [Testing](#testing)
  - [Contributing](#contributing)
    - [Reporting Issues](#reporting-issues)
  - [Documents](#documents)
  - [Feedback](#feedback)
  - [License](#license)

## Overview

The J-Quants API Client is a Rust implementation designed to provide seamless access to various J-Quants API endpoints. It facilitates efficient data retrieval and manipulation through asynchronous processing, ensuring fast and reliable communication.

## What is J-Quants?

J-Quants is a financial data platform provided by **JPX Market Innovation & Research, Inc. (JPXI)**.

It offers access to a wide range of financial data through its API, enabling developers, analysts, and investors to integrate and analyze financial information effectively.

To use the J-Quants API, you must register for an account.
A Free plan is available, allowing users to access the API without any cost.
You can register for an account below.

- [J-Quants Website (English)](https://jpx-jquants.com/?lang=en)
- [J-Quants Website (Japanese)](https://jpx-jquants.com/?lang=ja)

For an in-depth understanding of the data provided, service usage, and API specifications, please refer to the following resources:

- [API Specifications (English)](https://jpx.gitbook.io/j-quants-en)
- [API Specifications (Japanese)](https://jpx.gitbook.io/j-quants-ja)

## Features

- **Asynchronous Processing:** Ensures fast and efficient data communication.
- **Comprehensive API Support:** Access to authentication, data retrieval, and more.
- **Pagination Handling:** Easily manage paginated data responses.
- **Error Handling:** Robust error management for reliable operations.
- **Plan-Specific, Type-Safe Clients:** Provides clients tailored to each subscription plan, allowing for type-safe usage specific to individual plans.
- **Automatic ID Token Refresh:** Automatically handles the renewal of ID tokens, ensuring uninterrupted API access without manual intervention.
- **Secure Authentication Management:** Does not store email addresses and passwords in memory; they are immediately discarded after use to ensure security.

## Prerequisites

- **Rust:** Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- **Tokio:** Asynchronous runtime used by the client.

## Installation

Ensure Rust is installed on your system. Then, add the `jquants-api-client` to your `Cargo.toml`:

```toml
[dependencies]
jquants-api-client = "0.1.0"
```

## Supported APIs

The J-Quants API Client currently supports the following APIs:

### Authentication

| API Endpoint          | Description   | Status      |
| --------------------- | ------------- | ----------- |
| `/token/auth_user`    | Refresh Token | Implemented |
| `/token/auth_refresh` | ID Token      | Implemented |

### Data Retrieval

| API Endpoint                      | Description                          | Status      |
| --------------------------------- | ------------------------------------ | ----------- |
| `/listed/info`                    | Listed Issue Information             | Implemented |
| `/prices/daily_quotes`            | Stock Prices (OHLC)                  | Implemented |
| `/fins/statements`                | Financial Data                       | Implemented |
| `/fins/announcement`              | Earnings Calendar                    | Implemented |
| `/markets/trading_calendar`       | Trading Calendar                     | Implemented |
| `/markets/trades_spec`            | Trading by Type of Investors         | Implemented |
| `/indices/topix`                  | TOPIX Prices (OHLC)                  | Implemented |
| `/indices`                        | Indices (OHLC)                       | Implemented |
| `/option/index_option`            | Index Option Prices (OHLC)           | Implemented |
| `/derivatives/futures`            | Futures (OHLC)                       | Implemented |
| `/derivatives/options`            | Options (OHLC)                       | Implemented |
| `/markets/weekly_margin_interest` | Margin Trading Outstandings          | Implemented |
| `/markets/short_selling`          | Short Sale Value and Ratio by Sector | Implemented |
| `/markets/breakdown`              | Breakdown Trading Data               | Implemented |
| `/prices/prices_am`               | Morning Session Stock Prices (OHLC)  | Implemented |
| `/fins/dividend`                  | Cash Dividend Data                   | Implemented |
| `/fins/fs_details`                | Financial Statement Data (BS/PL)     | Implemented |

## Client Usage Example

Below are examples of how to use the J-Quants API client.

### Standard Fetch

This example demonstrates a basic fetch operation to retrieve listed issue information.

```rust
use jquants_api_client::{
    JQuantsFreePlanClient, ListedIssueInfoApi,
};
use tokio;

#[tokio::main]
async fn main() {
    // Replace with your actual refresh token.
    // Ensure you keep your refresh token secure and do not expose it in public repositories.
    let refresh_token = "YOUR_REFRESH_TOKEN";

    // Initialize the client with the refresh token
    let client = JQuantsFreePlanClient::new_from_refresh_token(refresh_token);

    // Make the API call to get listed issue information for code "2789"
    match client.get_listed_issue_info().code("2789").send().await {
        Ok(info) => println!("Listed Information: {:?}", info),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Pagination

This example demonstrates how to handle paginated responses when retrieving daily stock prices.

```rust
use futures::stream::StreamExt;
use jquants_api_client::{
    JQuantsFreePlanClient, DailyStockPricesApi, Paginatable,
};
use tokio;

#[tokio::main]
async fn main() {
    // Replace with your actual refresh token.
    // Ensure you keep your refresh token secure and do not expose it in public repositories.
    let refresh_token = "YOUR_REFRESH_TOKEN";

    // Initialize the client with the refresh token
    let client = JQuantsFreePlanClient::new_from_refresh_token(refresh_token);

    // Create a stream to fetch paginated daily stock prices
    let mut stream = client
        .get_daily_stock_prices()
        .code("27890")
        .date("2024-08-01")
        .fetch_pages_stream();

    // Iterate through each page in the stream
    while let Some(response) = stream.next().await {
        match response {
            Ok(daily_stock_prices_response) => {
                println!("{daily_stock_prices_response:?}");
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}
```

### Additional Examples

For more detailed examples, please refer to the [examples directory](./examples/) in the repository.

Note: For security reasons, never commit or expose your refresh tokens or any other sensitive information in your code repositories.

## Testing

Run unit tests using the following command:

```sh
cargo test
```

To run specific tests or see detailed output, you can use:

```sh
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! To ensure a smooth process, please follow these guidelines:

1. **Fork the Repository:** Click the "Fork" button at the top of this page to create your own fork.
2. **Clone Your Fork:**
   ```sh
   git clone https://github.com/your-username/jquants-api-client-rust.git
   cd jquants-api-client-rust
   ```
3. **Create a Branch:**
   ```sh
   git checkout -b feature/your-feature-name
   ```
4. **Make Your Changes:** Implement your feature or fix.
5. **Run Tests:** Ensure all tests pass.
   ```sh
   cargo test
   ```
6. **Commit Your Changes:**
   ```sh
   git commit -m "Add feature: your feature description"
   ```
7. **Push to Your Fork:**
   ```sh
   git push origin feature/your-feature-name
   ```
8. **Create a Pull Request:** Go to the original repository and create a pull request from your fork.

**Note:** Never commit or expose your refresh tokens or any other sensitive information in your code repositories.

### Reporting Issues

If you encounter any bugs or have feature suggestions, please open an issue on the [Issues page](https://github.com/ktanaka101/jquants-api-client-rust/issues).

## Documents

Comprehensive documentation is available [here](https://docs.rs/jquants-api-client).

For API reference, visit the [API Specifications (English)](https://jpx.gitbook.io/j-quants-en), [API Specifications (Japanese)](https://jpx.gitbook.io/j-quants-ja).

## Feedback

We value your feedback! If you have any suggestions or encounter issues, please open an issue or reach out directly.

## License

This project is licensed under the [MIT License](LICENSE).
