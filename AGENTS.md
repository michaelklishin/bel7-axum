# Instructions for AI Agents

## Overview

`bel7-axum` is a small library providing common Axum web server utilities:
CORS configuration, static file serving, pagination, and WebSocket helpers.

## Build and Test

```bash
cargo build --all-features

cargo fmt --all

cargo nextest run --all-features
cargo clippy --all-features
```

## Repository Layout

 * `src/lib.rs`: crate root
 * `src/pagination.rs`: response wrappers for pagination
 * `src/static_files.rs`: static file embedding that's friendly to SPA routing (requires the `embed` feature)
 * `src/websocket.rs`: WebSocket configuration constants (requires the `websocket` feature)

## Key Dependencies

 * `axum`: web framework
 * `tower-http`: CORS layer
 * `rust-embed`: static file embedding (optional)
 * `serde`: serialization for pagination types

## Target Rust Version

 * This library targets very recent stable Rust

## Rust Code Style

 * Use top-level `use` statements (imports), e.g. `Display` or `fmt::Display` with a `use` statement, not `std::fmt::Display`
 * Never use function-local `use` statements (imports)
 * Add tests to the `tests/` directory, not inline with implementation
 * At the end of each task, run `cargo fmt --all`
 * At the end of each task, run `cargo clippy --all-features` and fix any warnings it might emit

## Comments

 * Only add very important comments, both in tests and in the implementation

## Git Instructions

 * Never add yourself to the list of commit co-authors

## Style Guide

 * Never add full stops to Markdown list items
