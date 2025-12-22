// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Pagination utilities for API responses.

use serde::{Deserialize, Serialize};

/// A paginated response wrapper.
///
/// Provides consistent pagination metadata for list endpoints.
///
/// # Example
///
/// ```
/// use bel7_axum::PaginatedResponse;
///
/// let items = vec!["a", "b", "c"];
/// let response = PaginatedResponse::new(items, 100, Some(10), 0);
///
/// assert_eq!(response.data.len(), 3);
/// assert_eq!(response.total, 100);
/// assert!(response.has_more);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// This page's items.
    pub data: Vec<T>,

    /// Total number of items across all pages.
    pub total: u64,

    /// Limit per page (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Number of items skipped.
    pub offset: u64,

    /// Whether there are more items after this page.
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response.
    ///
    /// Automatically calculates the `has_more` value based on offset, data length, and total.
    pub fn new(data: Vec<T>, total: u64, limit: Option<u64>, offset: u64) -> Self {
        let returned = data.len() as u64;
        let has_more = offset + returned < total;

        Self {
            data,
            total,
            limit,
            offset,
            has_more,
        }
    }

    /// Creates a paginated response for a single page with all items.
    pub fn single_page(data: Vec<T>) -> Self {
        let total = data.len() as u64;
        Self {
            data,
            total,
            limit: None,
            offset: 0,
            has_more: false,
        }
    }

    /// Maps the data items using the provided function.
    pub fn map<U, F>(self, f: F) -> PaginatedResponse<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResponse {
            data: self.data.into_iter().map(f).collect(),
            total: self.total,
            limit: self.limit,
            offset: self.offset,
            has_more: self.has_more,
        }
    }
}

/// Query parameters for pagination.
/// Meant to be used with [`axum::extract::Query`].
///
/// # Example
///
/// ```ignore
/// use axum::extract::Query;
/// use bel7_axum::PaginationQuery;
///
/// async fn list_items(Query(pagination): Query<PaginationQuery>) -> impl IntoResponse {
///     let limit = pagination.effective_limit(100);
///     let offset = pagination.offset.unwrap_or(0);
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PaginationQuery {
    /// Maximum number of items to return.
    pub limit: Option<u64>,

    /// Number of items to skip.
    pub offset: Option<u64>,
}

impl PaginationQuery {
    /// Returns the effective limit, clamped to a maximum value if needed.
    pub fn effective_limit(&self, max: u64) -> u64 {
        self.limit.unwrap_or(max).min(max)
    }

    /// Returns the offset, defaulting to 0.
    pub fn effective_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }
}
