//! This crate provides an API client for JQuants.

pub mod api;
pub mod client;
pub mod error;

pub use api::builder::JQuantsBuilder;
pub use api::daily_stock_prices::*;
pub use api::error_response::ErrorResponse;
pub use api::listed_issue_info::*;
pub use api::morning_session_stock_prices::*;
pub use api::pagination::*;
pub use api::trading_by_type_of_investors::*;
pub use api::weekly_margin_trading_outstandings::*;
pub use client::{
    free_plan_client::JQuantsFreePlanClient, light_plan_client::JQuantsLightPlanClient,
    premium_plan_client::JQuantsPremiumPlanClient, standard_plan_client::JQuantsStandardPlanClient,
};
pub use error::JQuantsError;
