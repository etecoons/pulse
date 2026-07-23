//! Pulse-specific configuration.
//!
//! The common fields (port, site title, pin, max attempts, lockout time,
//! cookie age, trust-proxy, base URL, feature toggles) are read via
//! `shared_backend::server::ServerConfig::from_env("PULSE")`. The fields
//! that are unique to pulse (refresh interval, monitor_* toggles,
//! enable_coffee) are layered on top.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    /// Shared server config (port, pin, lockout, base URL, feature toggles).
    pub server: Arc<shared_backend::server::ServerConfig>,
    /// Seconds between system-metric refreshes.
    pub refresh_interval: u64,
    /// Whether to collect CPU stats.
    pub monitor_cpu: bool,
    /// Whether to collect memory stats.
    pub monitor_memory: bool,
    /// Whether to collect storage stats.
    pub monitor_storage: bool,
    /// Whether to collect network stats.
    pub monitor_network: bool,
    /// Whether to collect GPU stats.
    pub monitor_gpu: bool,
    /// Whether to show the Buy Me a Coffee link in the footer.
    pub enable_coffee: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        #[cfg(not(test))]
        {
            dotenvy::from_path("/app/data/.env").ok();
            dotenvy::dotenv().ok();
        }

        let server = Arc::new(shared_backend::server::ServerConfig::from_env("PULSE"));

        let refresh_interval = std::env::var("PULSE_REFRESH_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2);

        Self {
            server,
            refresh_interval,
            monitor_cpu: parse_optout_bool("PULSE_MONITOR_CPU", true),
            monitor_memory: parse_optout_bool("PULSE_MONITOR_MEMORY", true),
            monitor_storage: parse_optout_bool("PULSE_MONITOR_STORAGE", true),
            monitor_network: parse_optout_bool("PULSE_MONITOR_NETWORK", true),
            monitor_gpu: parse_optout_bool("PULSE_MONITOR_GPU", true),
            enable_coffee: parse_optout_bool("PULSE_ENABLE_COFFEE", true),
        }
    }
}

/// Parse a `true`/`false`/`on`/`off` env var, returning `default` on
/// missing or unrecognised values. Matches the convention used by
/// `shared_backend::server::ServerConfig`.
fn parse_optout_bool(key: &str, default: bool) -> bool {
    match std::env::var(key).ok().as_deref() {
        Some("true" | "TRUE" | "True" | "on" | "ON" | "On") => true,
        Some("false" | "FALSE" | "False" | "off" | "OFF" | "Off") => false,
        _ => default,
    }
}
