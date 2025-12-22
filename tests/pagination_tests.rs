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

use bel7_axum::{PaginatedResponse, PaginationQuery};

#[test]
fn test_paginated_response_has_more() {
    let resp: PaginatedResponse<i32> = PaginatedResponse::new(vec![1, 2, 3], 10, Some(3), 0);
    assert!(resp.has_more);

    let resp: PaginatedResponse<i32> = PaginatedResponse::new(vec![8, 9, 10], 10, Some(3), 7);
    assert!(!resp.has_more);
}

#[test]
fn test_single_page() {
    let resp = PaginatedResponse::single_page(vec![1, 2, 3]);
    assert_eq!(resp.total, 3);
    assert!(!resp.has_more);
}

#[test]
fn test_map() {
    let resp = PaginatedResponse::single_page(vec![1, 2, 3]);
    let mapped = resp.map(|x| x * 2);
    assert_eq!(mapped.data, vec![2, 4, 6]);
}

#[test]
fn test_pagination_query_defaults() {
    let q = PaginationQuery::default();
    assert_eq!(q.effective_limit(100), 100);
    assert_eq!(q.effective_offset(), 0);
}

#[test]
fn test_pagination_query_clamping() {
    let q = PaginationQuery {
        limit: Some(500),
        offset: Some(10),
    };
    assert_eq!(q.effective_limit(100), 100);
    assert_eq!(q.effective_offset(), 10);
}
