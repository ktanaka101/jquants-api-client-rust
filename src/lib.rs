//! This crate provides an API client for JQuants.

pub mod api;
pub mod client;
pub mod error;

pub use api::daily_stock_prices::*;
pub use api::listed_issue_info::*;
pub use api::morning_session_stock_prices::*;
pub use api::shared::{
    auth::{id_token::*, refresh_token::*},
    responses::error_response::*,
    traits::{builder::*, pagination::*},
    types::{issue_type::*, market_code::*, section_name::*, sector17_code::*, sector33_code::*},
};
pub use api::short_sale_by_sector::*;
pub use api::trading_by_type_of_investors::*;
pub use api::weekly_margin_trading_outstandings::*;
pub use client::{
    free_plan_client::JQuantsFreePlanClient, light_plan_client::JQuantsLightPlanClient,
    premium_plan_client::JQuantsPremiumPlanClient, standard_plan_client::JQuantsStandardPlanClient,
};
pub use error::JQuantsError;
