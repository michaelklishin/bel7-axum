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

//! WebSocket connection utilities.

use std::time::Duration;

/// Default idle timeout for WebSocket connections (5 minutes).
pub const DEFAULT_WS_IDLE_TIMEOUT: Duration = Duration::from_secs(300);

/// Default maximum message size (100 KB).
pub const DEFAULT_MAX_MESSAGE_SIZE: usize = 100 * 1024;

/// Configuration for WebSocket connections.
#[derive(Debug, Clone)]
pub struct WsConfig {
    /// How long to wait for a message before timing out.
    pub idle_timeout: Duration,

    /// Maximum allowed message size in bytes.
    pub max_message_size: usize,
}

impl Default for WsConfig {
    fn default() -> Self {
        Self {
            idle_timeout: DEFAULT_WS_IDLE_TIMEOUT,
            max_message_size: DEFAULT_MAX_MESSAGE_SIZE,
        }
    }
}

impl WsConfig {
    /// Creates a new configuration with custom values.
    pub fn new(idle_timeout: Duration, max_message_size: usize) -> Self {
        Self {
            idle_timeout,
            max_message_size,
        }
    }

    /// Sets the idle timeout.
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// Sets the maximum message size.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }
}
