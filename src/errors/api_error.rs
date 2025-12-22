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

//! Axum-specific error utilities.
//!
//! Provides types and traits for converting application errors
//! into HTTP responses with appropriate status codes.

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use thiserror::Error;

/// Standard JSON error response body.
///
/// This structure is returned for all API errors, providing
/// a consistent format for clients.
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    /// Short error description (e.g., "Not Found", "Bad Request").
    pub error: String,

    /// Optional detailed message. Omitted from JSON if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    /// Create a new error response with just an error message.
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    /// Create a new error response with error and details.
    pub fn with_details(error: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details.into()),
        }
    }
}

/// Common API error type with automatic HTTP status code mapping.
///
/// This enum covers the most common HTTP error scenarios. For custom
/// errors, implement `IntoApiError` on your own error type.
///
/// # Example
///
/// ```
/// use bel7_axum::ApiError;
///
/// fn get_user(id: i64) -> Result<String, ApiError> {
///     if id == 0 {
///         return Err(ApiError::BadRequest("ID cannot be zero".into()));
///     }
///     if id == 999 {
///         return Err(ApiError::NotFound(format!("User {} not found", id)));
///     }
///     Ok(format!("User {}", id))
/// }
/// ```
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    /// 400 Bad Request
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 401 Unauthorized
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 403 Forbidden
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 404 Not Found
    #[error("Not found: {0}")]
    NotFound(String),

    /// 409 Conflict
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 422 Unprocessable Entity
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 500 Internal Server Error.
    /// Details are meant to be logged but not exposed to clients.
    #[error("Internal error: {0}")]
    Internal(String),

    /// 503 Service Unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl ApiError {
    /// Get the HTTP status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Get the error label for the response body.
    pub fn error_label(&self) -> &'static str {
        match self {
            ApiError::BadRequest(_) => "Bad Request",
            ApiError::Unauthorized(_) => "Unauthorized",
            ApiError::Forbidden(_) => "Forbidden",
            ApiError::NotFound(_) => "Not Found",
            ApiError::Conflict(_) => "Conflict",
            ApiError::ValidationError(_) => "Validation Error",
            ApiError::Internal(_) => "Internal Server Error",
            ApiError::ServiceUnavailable(_) => "Service Unavailable",
        }
    }

    /// Check if this is a client error (4xx).
    pub fn is_client_error(&self) -> bool {
        self.status_code().is_client_error()
    }

    /// Check if this is a server error (5xx).
    pub fn is_server_error(&self) -> bool {
        self.status_code().is_server_error()
    }
}

impl ApiError {
    /// Extracts the inner message from this error.
    fn into_message(self) -> String {
        match self {
            ApiError::BadRequest(msg)
            | ApiError::Unauthorized(msg)
            | ApiError::Forbidden(msg)
            | ApiError::NotFound(msg)
            | ApiError::Conflict(msg)
            | ApiError::ValidationError(msg)
            | ApiError::Internal(msg)
            | ApiError::ServiceUnavailable(msg) => msg,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let label = self.error_label();

        // For internal errors, don't expose details to clients
        let details = if matches!(self, ApiError::Internal(_)) {
            None
        } else {
            Some(self.into_message())
        };

        let body = ErrorResponse {
            error: label.to_string(),
            details,
        };

        (status, axum::Json(body)).into_response()
    }
}

/// Trait for converting domain errors into API errors.
///
/// Implement this trait on your domain error types to enable
/// automatic conversion to HTTP responses.
///
/// # Example
///
/// ```
/// use bel7_axum::{ApiError, IntoApiError};
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// enum DomainError {
///     #[error("user {0} not found")]
///     UserNotFound(i64),
///     #[error("duplicate email")]
///     DuplicateEmail,
///     #[error("database error: {0}")]
///     Database(String),
/// }
///
/// impl IntoApiError for DomainError {
///     fn into_api_error(self) -> ApiError {
///         match self {
///             DomainError::UserNotFound(id) => {
///                 ApiError::NotFound(format!("User {} not found", id))
///             }
///             DomainError::DuplicateEmail => {
///                 ApiError::Conflict("Email already exists".into())
///             }
///             DomainError::Database(msg) => {
///                 ApiError::Internal(msg)
///             }
///         }
///     }
/// }
/// ```
pub trait IntoApiError {
    /// Convert this error into an API error.
    fn into_api_error(self) -> ApiError;
}

impl<E: IntoApiError> From<E> for ApiError {
    fn from(err: E) -> Self {
        err.into_api_error()
    }
}

/// Helper to create a JSON error response tuple.
///
/// Useful when you need more control over the response.
pub fn json_error(status: StatusCode, error: &str, details: Option<String>) -> Response {
    let body = ErrorResponse {
        error: error.to_string(),
        details,
    };
    (status, axum::Json(body)).into_response()
}
