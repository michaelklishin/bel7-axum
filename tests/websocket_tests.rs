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

use bel7_axum::WsConfig;
use std::time::Duration;

#[test]
fn test_default_config() {
    let config = WsConfig::default();
    assert_eq!(config.idle_timeout, Duration::from_secs(300));
    assert_eq!(config.max_message_size, 100 * 1024);
}

#[test]
fn test_builder_pattern() {
    let config = WsConfig::default()
        .with_idle_timeout(Duration::from_secs(60))
        .with_max_message_size(50 * 1024);

    assert_eq!(config.idle_timeout, Duration::from_secs(60));
    assert_eq!(config.max_message_size, 50 * 1024);
}
