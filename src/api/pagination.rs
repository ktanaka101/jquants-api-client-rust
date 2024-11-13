//! Pagination response module.
//!
//! See: [Paging of Responses](https://jpx.gitbook.io/j-quants-en/api-reference/attention#paging-of-responses)

use std::fmt;
use std::future::Future;

use async_stream::try_stream;
use futures::stream;
use futures::StreamExt;
use serde::de::DeserializeOwned;

use crate::JQuantsError;

use super::builder::JQuantsBuilder;

/// Trait for types that have a pagination key.
pub trait HasPaginationKey {
    /// Get the pagination key.
    fn get_pagination_key(&self) -> Option<&str>;
}

/// Trait for types that can merge pages.
pub trait MergePage: Sized {
    /// Merge the pages.
    fn merge_page(
        page: Result<Vec<Self>, crate::JQuantsError>,
    ) -> Result<Self, crate::JQuantsError>;
}

/// Trait for paginatable responses.
pub trait Paginatable<R: DeserializeOwned + fmt::Debug + HasPaginationKey + MergePage>:
    JQuantsBuilder<R> + Clone
{
    /// Set the pagination key.
    fn pagination_key(&mut self, pagination_key: impl Into<String>) -> &mut Self;

    /// Fetch the pages stream.
    fn fetch_pages_stream(&self) -> impl stream::Stream<Item = Result<R, JQuantsError>> {
        let stream = try_stream! {
            let mut builder = self.clone();

            loop {
                let response =  builder.send().await?;
                let next_pagination_key = response.get_pagination_key();
                if let Some(key) = next_pagination_key {
                    builder.pagination_key(key.to_string());

                    yield response;
                    continue;
                } else {
                    yield response;
                    break;
                }
            }
        };

        Box::pin(stream)
    }

    /// Fetch all pages.
    fn fetch_all(&self) -> impl Future<Output = Result<Vec<R>, JQuantsError>> {
        async {
            let results: Vec<Result<R, JQuantsError>> = self.fetch_pages_stream().collect().await;
            let mut final_results = Vec::new();
            for result in results {
                match result {
                    Ok(value) => final_results.push(value),
                    Err(e) => return Err(e),
                }
            }
            Ok(final_results)
        }
    }

    /// Fetch all pages and merge them.
    fn fetch_all_and_merge(&self) -> impl Future<Output = Result<R, JQuantsError>> {
        async {
            let results = self.fetch_all().await;
            R::merge_page(results)
        }
    }
}
