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

//! Common Axum web server utilities.
//!
//! This crate provides reusable components for Axum-based web servers:
//!
//! - Error handling with [`ApiError`] and [`IntoApiError`]
//! - Static file serving with SPA routing support (requires `embed` feature)
//! - Pagination response wrappers
//! - WebSocket connection helpers (requires `websocket` feature)
//!
//! # Features
//!
//! - `embed` - Enables `rust-embed` based static file serving
//! - `websocket` - Enables WebSocket utilities with timeout handling
//! - `full` - Enables all features

mod errors;
mod pagination;

#[cfg(feature = "embed")]
mod static_files;

#[cfg(feature = "websocket")]
mod websocket;

pub use errors::*;
pub use pagination::*;

#[cfg(feature = "embed")]
pub use static_files::*;

#[cfg(feature = "websocket")]
pub use websocket::*;
