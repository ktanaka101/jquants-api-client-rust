//! This crate provides an API client for JQuants.

pub mod api;
pub mod client;
pub mod error;

pub use client::{
    free_plan_client::JQuantsFreePlanClient, light_plan_client::JQuantsLightPlanClient,
    premium_plan_client::JQuantsPremiumPlanClient, standard_plan_client::JQuantsStandardPlanClient,
};
pub use error::JQuantsError;
