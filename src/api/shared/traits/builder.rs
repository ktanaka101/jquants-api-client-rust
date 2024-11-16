//! API builder trait.

use std::fmt;

use serde::de::DeserializeOwned;

/// Trait for API builders.
pub trait JQuantsBuilder<R: DeserializeOwned + fmt::Debug> {
    /// Send the request.
    fn send(self) -> impl std::future::Future<Output = Result<R, crate::JQuantsError>>;

    /// Send the request without consuming ownership.
    /// Use only when reusing the builder.
    fn send_ref(&self) -> impl std::future::Future<Output = Result<R, crate::JQuantsError>>;
}
