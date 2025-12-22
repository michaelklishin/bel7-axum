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

use bel7_axum::{ApiError, ErrorResponse};
use http::StatusCode;

#[test]
fn test_status_codes() {
    assert_eq!(
        ApiError::NotFound("x".into()).status_code(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        ApiError::BadRequest("x".into()).status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        ApiError::Internal("x".into()).status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn test_error_classification() {
    assert!(ApiError::NotFound("x".into()).is_client_error());
    assert!(!ApiError::NotFound("x".into()).is_server_error());
    assert!(ApiError::Internal("x".into()).is_server_error());
}

#[test]
fn test_error_response_serialization() {
    let resp = ErrorResponse::with_details("Not Found", "User 123 not found");
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("Not Found"));
    assert!(json.contains("User 123 not found"));
}

#[test]
fn test_error_response_without_details() {
    let resp = ErrorResponse::new("Internal Server Error");
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("Internal Server Error"));
    assert!(!json.contains("details"));
}
