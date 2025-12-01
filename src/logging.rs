//! Logging module - Centralized logging configuration for Sol Calendar.
//!
//! This module provides a unified logging setup for the entire application.
//! It configures the log level, format, and output destination.
//!
//! # Usage
//!
//! Initialize logging at application startup:
//! ```rust
//! logging::init();
//! ```
//!
//! Then use the standard log macros throughout the app:
//! ```rust
//! use log::{debug, info, warn, error, trace};
//!
//! info!("Application started");
//! debug!("Processing event uid={}", uid);
//! warn!("Calendar not found: {}", id);
//! error!("Failed to save: {}", e);
//! ```
//!
//! # Log Levels
//!
//! Control the log level with the `RUST_LOG` environment variable:
//! - `RUST_LOG=error` - Only errors
//! - `RUST_LOG=warn` - Warnings and errors
//! - `RUST_LOG=info` - Info, warnings, and errors (default)
//! - `RUST_LOG=debug` - Debug and above
//! - `RUST_LOG=trace` - All logs including trace
//!
//! You can also filter by module:
//! - `RUST_LOG=sol_calendar::services=debug` - Debug logs for services only
//! - `RUST_LOG=sol_calendar=debug,cosmic=warn` - Mixed levels

use log::{info, LevelFilter};

/// Default log level when RUST_LOG is not set
const DEFAULT_LOG_LEVEL: &str = "info";

/// Application name for log prefix
const APP_NAME: &str = "sol-calendar";

/// Initialize the logging system.
///
/// This should be called once at application startup, before any logging occurs.
/// The log level is controlled by the `RUST_LOG` environment variable.
///
/// By default, third-party libraries (wgpu, cosmic, iced, etc.) are set to `warn`
/// level to reduce log noise, while the application itself uses `info` level.
///
/// # Example
///
/// ```rust
/// fn main() {
///     logging::init();
///     log::info!("Application started");
/// }
/// ```
///
/// # Customizing Log Levels
///
/// Override with RUST_LOG:
/// - `RUST_LOG=debug` - Debug for everything (very verbose)
/// - `RUST_LOG=sol_calendar=debug` - Debug for app only
/// - `RUST_LOG=sol_calendar=debug,wgpu=error` - Debug app, silence wgpu
pub fn init() {
    // Default filter: app at info, noisy libraries at warn/error
    let default_filter = format!(
        "{level},wgpu_core=error,wgpu_hal=error,cosmic=warn,iced=warn,winit=warn,calloop=warn,sctk=warn",
        level = DEFAULT_LOG_LEVEL
    );

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&default_filter)
    )
    .format_timestamp_millis()
    .format_module_path(true)
    .format_target(false)
    .init();

    info!("{} logging initialized", APP_NAME);
}

/// Initialize logging with a custom default level.
///
/// Use this if you want a different default than "info".
///
/// # Arguments
///
/// * `default_level` - The default log level (e.g., "debug", "warn")
#[allow(dead_code)] // Reserved for future log level configuration
pub fn init_with_level(default_level: &str) {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(default_level)
    )
    .format_timestamp_millis()
    .format_module_path(true)
    .format_target(false)
    .init();

    info!("{} logging initialized with level {}", APP_NAME, default_level);
}

/// Check if debug logging is enabled.
///
/// Useful for conditionally computing expensive debug information.
#[allow(dead_code)] // Reserved for conditional debug info
pub fn is_debug_enabled() -> bool {
    log::max_level() >= LevelFilter::Debug
}

/// Check if trace logging is enabled.
///
/// Useful for conditionally computing very expensive trace information.
#[allow(dead_code)] // Reserved for conditional trace info
pub fn is_trace_enabled() -> bool {
    log::max_level() >= LevelFilter::Trace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_log_level() {
        assert_eq!(DEFAULT_LOG_LEVEL, "info");
    }

    #[test]
    fn test_app_name() {
        assert_eq!(APP_NAME, "sol-calendar");
    }
}
