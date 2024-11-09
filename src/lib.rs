//! This crate provides an API client for JQuants.

pub mod api;
pub mod client;
pub mod error;

pub use client::{
    free_plan_client::JQuantsFreePlanClient, standard_plan_client::JQuantsStandardPlanClient,
};
pub use error::JQuantsError;
