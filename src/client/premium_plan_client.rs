//! Premium plan client implementation for JQuants API.

use crate::{
    api::{
        breakdown_trading_data::BreakdownTradingDataApi,
        daily_stock_prices::{DailyStockPricesApi, DailyStockPricesPremiumPlanResponse},
        financial_statements::FinancialStatementsApi,
        listed_issue_info::{ListedIssueInfoApi, ListedIssueInfoPremiumPlanResponse},
        morning_session_stock_prices::MorningSessionStockPricesApi,
        short_sale_by_sector::ShortSaleBySectorApi,
        weekly_margin_trading_outstandings::WeeklyMarginTradingOutstandingsApi,
        JQuantsApiClient, JQuantsPlanClient,
    },
    CashDividendDataApi, EarningsCalendarApi, FinancialStatementDetailsApi, FuturesPricesApi,
    IndexOptionPricesApi, IndicesApi, TopixPricesApi, TradingByInvestorTypeApi, TradingCalendarApi,
};

/// Premium plan client for J-Quants API.
///
/// # Example
///
/// ```no_run
/// use jquants_api_client::{
///     BreakdownTradingDataApi, CashDividendDataApi, DailyStockPricesApi, EarningsCalendarApi, FinancialStatementsApi,
///     FuturesPricesApi, FinancialStatementDetailsApi, IndexOptionPricesApi, IndicesApi, JQuantsBuilder,
///     JQuantsPremiumPlanClient, ListedIssueInfoApi, MorningSessionStockPricesApi, ShortSaleBySectorApi,
///     TopixPricesApi, TradingByInvestorTypeApi, TradingCalendarApi, Paginatable, WeeklyMarginTradingOutstandingsApi
/// };
///
/// async {
///     // Authenticate with a refresh token.
///     let client = JQuantsPremiumPlanClient::new_from_refresh_token("your_refresh_token".to_string());
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
///     let response = client.get_short_sale_by_sector().send().await.unwrap();
///
///     // Paginate short sale value and ratio by sector.
///     let response = client.get_short_sale_by_sector().fetch_all().await.unwrap();
///     let response = client.get_short_sale_by_sector().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_short_sale_by_sector().fetch_pages_stream();
///
///     // Get trading calendar.
///     let response = client.get_trading_calendar().send().await.unwrap();
///
///     // Get indices.
///     let response = client.get_indices().send().await.unwrap();
///
///     // Paginate  indices.
///     let response = client.get_indices().fetch_all().await.unwrap();
///     let response = client.get_indices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_indices().fetch_pages_stream();
///
///     // Get TOPIX prices.
///     let response = client.get_topix_prices().send().await.unwrap();
///
///     // Paginate TOPIX prices.
///     let response = client.get_topix_prices().fetch_all().await.unwrap();
///     let response = client.get_topix_prices().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_topix_prices().fetch_pages_stream();
///
///     // Get financial statements.
///     let response = client.get_financial_statements().send().await.unwrap();
///
///     // Paginate stock prices.
///     let response = client.get_financial_statements().fetch_all().await.unwrap();
///     let response = client.get_financial_statements().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_financial_statements().fetch_pages_stream();
///
///     // Get financial statement detail.
///     let response = client.get_financial_statement_details().send().await.unwrap();
///
///     // Paginate financial statement detail.
///     let response = client.get_financial_statement_details().fetch_all().await.unwrap();
///     let response = client.get_financial_statement_details().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_financial_statement_details().fetch_pages_stream();
///
///     // Get cash dividend data.
///     let response = client.get_cash_dividend_data().send().await.unwrap();
///
///     // Paginate cash dividend data.
///     let response = client.get_cash_dividend_data().fetch_all().await.unwrap();
///     let response = client.get_cash_dividend_data().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_cash_dividend_data().fetch_pages_stream();
///
///     // Get earnings calendar.
///     let response = client.get_earnings_calendar().send().await.unwrap();
///
///     // Paginate earnings calendar.
///     let response = client.get_earnings_calendar().fetch_all().await.unwrap();
///     let response = client.get_earnings_calendar().fetch_all_and_merge().await.unwrap();
///     let stream = client.get_earnings_calendar().fetch_pages_stream();
///
///     // Get idnex option prices.
///     let response = client.get_index_option_prices("2024-08-01").send().await.unwrap();
///
///     // Paginate idnex option prices.
///     let response = client.get_index_option_prices("2024-08-01").fetch_all().await.unwrap();
///     let response = client.get_index_option_prices("2024-08-01").fetch_all_and_merge().await.unwrap();
///     let stream = client.get_index_option_prices("2024-08-01").fetch_pages_stream();
///
///     // Get futures prices.
///     let response = client.get_futures_prices("2024-08-01").send().await.unwrap();
///
///     // Paginate futures prices.
///     let response = client.get_futures_prices("2024-08-01").fetch_all().await.unwrap();
///     let response = client.get_futures_prices("2024-08-01").fetch_all_and_merge().await.unwrap();
///     let stream = client.get_futures_prices("2024-08-01").fetch_pages_stream();
/// };
/// ```
#[derive(Clone)]
pub struct JQuantsPremiumPlanClient {
    api_client: JQuantsApiClient,
}

impl JQuantsPremiumPlanClient {
    /// Create a new client from a refresh token.
    pub fn new_from_refresh_token(refresh_token: String) -> Self {
        Self {
            api_client: JQuantsApiClient::new_from_refresh_token(refresh_token),
        }
    }
}

impl JQuantsPlanClient for JQuantsPremiumPlanClient {
    fn get_api_client(&self) -> &JQuantsApiClient {
        &self.api_client
    }
}

impl ListedIssueInfoApi for JQuantsPremiumPlanClient {
    type Response = ListedIssueInfoPremiumPlanResponse;
}

impl DailyStockPricesApi for JQuantsPremiumPlanClient {
    type Response = DailyStockPricesPremiumPlanResponse;
}

impl MorningSessionStockPricesApi for JQuantsPremiumPlanClient {}

impl TradingByInvestorTypeApi for JQuantsPremiumPlanClient {}

impl WeeklyMarginTradingOutstandingsApi for JQuantsPremiumPlanClient {}

impl ShortSaleBySectorApi for JQuantsPremiumPlanClient {}

impl BreakdownTradingDataApi for JQuantsPremiumPlanClient {}

impl TradingCalendarApi for JQuantsPremiumPlanClient {}

impl IndicesApi for JQuantsPremiumPlanClient {}

impl TopixPricesApi for JQuantsPremiumPlanClient {}

impl FinancialStatementsApi for JQuantsPremiumPlanClient {}

impl FinancialStatementDetailsApi for JQuantsPremiumPlanClient {}

impl CashDividendDataApi for JQuantsPremiumPlanClient {}

impl EarningsCalendarApi for JQuantsPremiumPlanClient {}

impl IndexOptionPricesApi for JQuantsPremiumPlanClient {}

impl FuturesPricesApi for JQuantsPremiumPlanClient {}
