//! Standard plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesStandardPlanResponse},
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoStandardPlanResponse},
        morning_session_stock_prices::{
            MorningSessionStockPricesApi, MorningSessionStockPricesStandardPlanResponse,
        },
        weekly_margin_trading_outstandings::{
            WeeklyMarginTradingOutstandingsApi, WeeklyMarginTradingOutstandingsStandardPlanResponse,
        },
        JQuantsApiClient, JQuantsPlanClient,
    },
    TradingByInvestorTypeApi, TradingByInvestorTypeStandardPlanResponse,
};

/// Standard plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{DailyStockPricesApi, JQuantsBuilder, JQuantsStandardPlanClient, ListedIssueInfoApi, MorningSessionStockPricesApi, TradingByInvestorTypeApi, Paginatable, WeeklyMarginTradingOutstandingsApi};
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsStandardPlanClient::new_from_refresh_token("your_refresh_token".to_string());
///
///     // Get listed issue information.
///     let response = client.get_listed_issue_info().send().await.unwrap();
///
///     // Get stock prices.
///     let response = client.get_daily_stock_prices().send().await.unwrap();
///
///     // Paginate stock prices.
///     let response = client.get_daily_stock_prices().fetch_all().await.unwrap();
///     let response = client.get_daily_stock_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_daily_stock_prices().fetch_pages_stream();
///
///     // Get morning session stock prices.
///     let response = client.morning_session_stock_prices().send().await.unwrap();
///
///     // Paginate morning session stock prices.
///     let response = client.morning_session_stock_prices().fetch_all().await.unwrap();
///     let response = client.morning_session_stock_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.morning_session_stock_prices().fetch_pages_stream();
///
///     // Get trading by investor type.
///     let response = client.get_trading_by_investor_type().send().await.unwrap();
///
///     // Paginate trading by investor type.
///     let response = client.get_trading_by_investor_type().fetch_all().await.unwrap();
///     let response = client.get_trading_by_investor_type().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_trading_by_investor_type().fetch_pages_stream();
///
///     // Get margin trading outstandings.
///     let response = client.get_weekly_margin_trading_outstandings().send().await.unwrap();
///
///     // Paginate margin trading outstandings.
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all().await.unwrap();
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_weekly_margin_trading_outstandings().fetch_pages_stream();
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsStandardPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsStandardPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsStandardPlanClient {
    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsStandardPlanClient {
    type Response = ListedIssueInfoStandardPlanResponse;
}

impl DailyStockPricesApi for JQuantsStandardPlanClient {
    type Response = DailyStockPricesStandardPlanResponse;
}

impl MorningSessionStockPricesApi for JQuantsStandardPlanClient {
    type Response = MorningSessionStockPricesStandardPlanResponse;
}

impl TradingByInvestorTypeApi for JQuantsStandardPlanClient {
    type Response = TradingByInvestorTypeStandardPlanResponse;
}

impl WeeklyMarginTradingOutstandingsApi for JQuantsStandardPlanClient {
    type Response = WeeklyMarginTradingOutstandingsStandardPlanResponse;
}
