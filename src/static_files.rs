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

//! Static file serving with SPA routing support.

use axum::{
    body::Body,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::Embed;

/// Serves static files from an embedded asset collection with SPA routing.
///
/// This handler:
///
/// - Serves files directly when they exist (e.g., `/assets/app.js`)
/// - Falls back to `index.html` for paths without extensions (SPA routing)
/// - Returns 404 only if `index.html` itself is missing
///
/// # Example
///
/// ```ignore
/// use rust_embed::Embed;
/// use bel7_axum::serve_spa_static;
///
/// #[derive(Embed)]
/// #[folder = "static/"]
/// struct Assets;
///
/// let app = Router::new()
///     .fallback(serve_spa_static::<Assets>);
/// ```
pub async fn serve_spa_static<E: Embed>(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    let path = if path.is_empty() || !path.contains('.') {
        "index.html"
    } else {
        path
    };

    serve_embedded_file::<E>(path)
}

/// Serves a static file from an embedded asset collection.
///
/// Unlike `serve_spa_static`, this doesn't do SPA routing - it returns
/// 404 if the exact file isn't found.
///
/// # Example
///
/// ```ignore
/// use rust_embed::Embed;
/// use bel7_axum::serve_static;
///
/// #[derive(Embed)]
/// #[folder = "static/"]
/// struct Assets;
///
/// let app = Router::new()
///     .fallback(serve_static::<Assets>);
/// ```
pub async fn serve_static<E: Embed>(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match E::get(path) {
        Some(content) => file_response(path, content.data.into_owned()),
        None => not_found_response(),
    }
}

fn serve_embedded_file<E: Embed>(path: &str) -> Response {
    match E::get(path) {
        Some(content) => file_response(path, content.data.into_owned()),
        None => {
            // Fallback to index.html for SPA routing
            match E::get("index.html") {
                Some(content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(Body::from(content.data.into_owned()))
                    .unwrap(),
                None => not_found_response(),
            }
        }
    }
}

fn file_response(path: &str, data: Vec<u8>) -> Response {
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(data))
        .unwrap()
}

fn not_found_response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}
