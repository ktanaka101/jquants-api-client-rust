//! Light plan client implementation for JQuants API.

use crate::{
    api::{
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesLightPlanResponse},
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoLightPlanResponse},
        morning_session_stock_prices::{
            MorningSessionStockPricesApi, MorningSessionStockPricesLightPlanResponse,
        },
        short_sale_by_sector::{ShortSaleBySectorApi, ShortSaleBySectorLightPlanResponse},
        weekly_margin_trading_outstandings::{
            WeeklyMarginTradingOutstandingsApi, WeeklyMarginTradingOutstandingsLightPlanResponse,
        },
        JQuantsApiClient, JQuantsPlanClient,
    },
    TradingByInvestorTypeApi, TradingByInvestorTypeLightPlanResponse,
};

/// Light plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{DailyStockPricesApi, JQuantsBuilder, JQuantsLightPlanClient, ListedIssueInfoApi, MorningSessionStockPricesApi, ShortSaleBySectorApi, TradingByInvestorTypeApi, Paginatable, WeeklyMarginTradingOutstandingsApi};
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsLightPlanClient::new_from_refresh_token("your_refresh_token".to_string());
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
///
///     // Get short sale value and ratio by sector.
///     let response = client.get_weekly_margin_trading_outstandings().send().await.unwrap();
///
///     // Paginate short sale value and ratio by sector.
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all().await.unwrap();
///     let response = client.get_weekly_margin_trading_outstandings().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_weekly_margin_trading_outstandings().fetch_pages_stream();
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsLightPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsLightPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsLightPlanClient {
    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsLightPlanClient {
    type Response = ListedIssueInfoLightPlanResponse;
}

impl DailyStockPricesApi for JQuantsLightPlanClient {
    type Response = DailyStockPricesLightPlanResponse;
}

impl MorningSessionStockPricesApi for JQuantsLightPlanClient {
    type Response = MorningSessionStockPricesLightPlanResponse;
}

impl TradingByInvestorTypeApi for JQuantsLightPlanClient {
    type Response = TradingByInvestorTypeLightPlanResponse;
}

impl WeeklyMarginTradingOutstandingsApi for JQuantsLightPlanClient {
    type Response = WeeklyMarginTradingOutstandingsLightPlanResponse;
}

impl ShortSaleBySectorApi for JQuantsLightPlanClient {
    type Response = ShortSaleBySectorLightPlanResponse;
}
