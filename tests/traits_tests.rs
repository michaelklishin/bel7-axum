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

use bel7_axum::{ErrorMessageExt, RecoverableError};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("timeout occurred")]
struct TimeoutError;

#[derive(Error, Debug)]
#[error("invalid data")]
struct ValidationError;

impl RecoverableError for TimeoutError {
    fn is_recoverable(&self) -> bool {
        true
    }
}

impl RecoverableError for ValidationError {
    fn is_recoverable(&self) -> bool {
        false
    }
}

#[test]
fn test_recoverable_classification() {
    let timeout = TimeoutError;
    let validation = ValidationError;

    assert!(timeout.is_recoverable());
    assert!(!validation.is_recoverable());
}

#[test]
fn test_message_heuristics() {
    #[derive(Error, Debug)]
    #[error("connection timeout after 30s")]
    struct Err1;

    #[derive(Error, Debug)]
    #[error("connection closed by peer")]
    struct Err2;

    #[derive(Error, Debug)]
    #[error("connection reset by peer")]
    struct Err3;

    assert!(Err1.message_suggests_timeout());
    assert!(Err2.message_suggests_connection_closed());
    assert!(Err3.message_suggests_connection_reset());
}
