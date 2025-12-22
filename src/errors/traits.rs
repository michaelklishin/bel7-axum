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

//! Core error classification traits.
//!
//! These traits provide a common vocabulary for describing error behavior
//! across different error types in your application.

use std::error::Error;

/// Trait for errors that can be classified as recoverable or not.
///
/// Recoverable errors are typically transient failures that may succeed
/// on retry (timeouts, temporary network issues, etc.).
///
/// # Example
///
/// ```
/// use bel7_axum::RecoverableError;
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// enum MyError {
///     #[error("connection timeout")]
///     Timeout,
///     #[error("invalid input: {0}")]
///     InvalidInput(String),
/// }
///
/// impl RecoverableError for MyError {
///     fn is_recoverable(&self) -> bool {
///         matches!(self, MyError::Timeout)
///     }
/// }
/// ```
pub trait RecoverableError: Error {
    /// Returns `true` if this error is potentially recoverable via retry.
    fn is_recoverable(&self) -> bool;
}

/// Trait for errors related to network connections.
///
/// Useful for connection pool management, reconnection logic, and
/// distinguishing between connection failures and application errors.
pub trait ConnectionError: Error {
    /// Returns `true` if this error indicates the connection was closed.
    fn is_connection_closed(&self) -> bool;

    /// Returns `true` if this error indicates a connection timeout.
    fn is_timeout(&self) -> bool {
        false
    }

    /// Returns `true` if this error indicates a connection was refused.
    fn is_connection_refused(&self) -> bool {
        false
    }
}

/// Trait for errors that can provide additional context.
///
/// Useful for building rich error messages with suggestions or help text.
pub trait DiagnosticError: Error {
    /// Returns suggestions for fixing this error, if any.
    fn suggestions(&self) -> Vec<String> {
        Vec::new()
    }

    /// Returns help text for this error, if any.
    fn help(&self) -> Option<String> {
        None
    }

    /// Returns the position in input where this error occurred, if applicable.
    fn position(&self) -> Option<usize> {
        None
    }
}

/// Extension trait for checking common error message patterns.
///
/// Provides heuristic-based error classification by examining error messages.
/// Useful when you don't control the error type but need to classify it.
pub trait ErrorMessageExt {
    /// Check if the error message suggests a timeout.
    fn message_suggests_timeout(&self) -> bool;

    /// Check if the error message suggests a closed connection.
    fn message_suggests_connection_closed(&self) -> bool;

    /// Check if the error message suggests a connection reset.
    fn message_suggests_connection_reset(&self) -> bool;
}

impl<E: Error> ErrorMessageExt for E {
    fn message_suggests_timeout(&self) -> bool {
        let msg = self.to_string().to_lowercase();
        msg.contains("timeout") || msg.contains("timed out")
    }

    fn message_suggests_connection_closed(&self) -> bool {
        let msg = self.to_string().to_lowercase();
        msg.contains("closed") || msg.contains("eof") || msg.contains("end of file")
    }

    fn message_suggests_connection_reset(&self) -> bool {
        let msg = self.to_string().to_lowercase();
        msg.contains("reset") || msg.contains("broken pipe") || msg.contains("connection reset")
    }
}
