//! This crate provides an API client for JQuants.

pub mod api;
pub mod client;
pub mod error;

pub use api::breakdown_trading_data::*;
pub use api::cash_dividend_data::*;
pub use api::daily_stock_prices::*;
pub use api::earnings_calendar::*;
pub use api::financial_statement_details::*;
pub use api::financial_statements::*;
pub use api::futures_prices::*;
pub use api::index_option_prices::*;
pub use api::indicies::*;
pub use api::listed_issue_info::*;
pub use api::morning_session_stock_prices::*;
pub use api::options_prices::*;
pub use api::shared::{
    auth::{id_token::*, refresh_token::*},
    responses::error_response::*,
    traits::{builder::*, pagination::*},
    types::{
        accounting_period::*, futures_code::*, holiday_division::*, index_code::*, issue_type::*,
        margin_code::MarginCode, market_code::*, options_code::*, price_limit::*, section_name::*,
        sector17_code::*, sector33_code::*, type_of_document::*,
    },
};
pub use api::short_sale_by_sector::*;
pub use api::topic_prices::*;
pub use api::trading_by_type_of_investors::*;
pub use api::trading_calendar::*;
pub use api::weekly_margin_trading_outstandings::*;
pub use api::*;
pub use client::{
    free_plan_client::JQuantsFreePlanClient, light_plan_client::JQuantsLightPlanClient,
    premium_plan_client::JQuantsPremiumPlanClient, standard_plan_client::JQuantsStandardPlanClient,
};
pub use error::JQuantsError;
