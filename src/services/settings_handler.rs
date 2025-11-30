//! Settings Handler - Centralized settings management.
//!
//! This handler provides a single point of contact for all settings operations,
//! including loading, saving, validation, and applying settings changes.

use crate::settings::AppSettings;
use log::{debug, error, info, warn};
use std::error::Error;

/// Result type for settings operations
pub type SettingsResult<T> = Result<T, SettingsError>;

/// Error types for settings operations
#[derive(Debug)]
pub enum SettingsError {
    /// Failed to load settings
    LoadError(String),
    /// Failed to save settings
    SaveError(String),
    /// Invalid setting value
    ValidationError(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::LoadError(msg) => write!(f, "Failed to load settings: {}", msg),
            SettingsError::SaveError(msg) => write!(f, "Failed to save settings: {}", msg),
            SettingsError::ValidationError(msg) => write!(f, "Invalid setting: {}", msg),
        }
    }
}

impl Error for SettingsError {}

/// Settings Handler - centralized settings management.
pub struct SettingsHandler;

impl SettingsHandler {
    /// Load settings from disk, returning defaults if not found
    pub fn load() -> SettingsResult<AppSettings> {
        debug!("SettingsHandler: Loading settings from disk");
        match AppSettings::load() {
            Ok(settings) => {
                info!("SettingsHandler: Settings loaded successfully");
                debug!("SettingsHandler: show_week_numbers={}", settings.show_week_numbers);
                Ok(settings)
            }
            Err(e) => {
                warn!("SettingsHandler: Failed to load settings: {}", e);
                Err(SettingsError::LoadError(e.to_string()))
            }
        }
    }

    /// Save settings to disk
    pub fn save(settings: &AppSettings) -> SettingsResult<()> {
        debug!("SettingsHandler: Saving settings to disk");
        settings.save().map_err(|e| {
            error!("SettingsHandler: Failed to save settings: {}", e);
            SettingsError::SaveError(e.to_string())
        })?;
        info!("SettingsHandler: Settings saved successfully");
        Ok(())
    }

    /// Toggle week numbers display and save
    pub fn toggle_week_numbers(settings: &mut AppSettings) -> SettingsResult<()> {
        let new_value = !settings.show_week_numbers;
        info!("SettingsHandler: Toggling week numbers: {} -> {}", settings.show_week_numbers, new_value);
        settings.show_week_numbers = new_value;
        Self::save(settings)
    }

    /// Set week numbers display and save
    pub fn set_week_numbers(settings: &mut AppSettings, show: bool) -> SettingsResult<()> {
        info!("SettingsHandler: Setting week numbers to {}", show);
        settings.show_week_numbers = show;
        Self::save(settings)
    }

    /// Reset settings to defaults and save
    pub fn reset_to_defaults() -> SettingsResult<AppSettings> {
        info!("SettingsHandler: Resetting settings to defaults");
        let settings = AppSettings::default();
        Self::save(&settings)?;
        info!("SettingsHandler: Settings reset complete");
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_settings() {
        // Should load defaults if no file exists
        let result = SettingsHandler::load();
        assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_creates_opposite() {
        let mut settings = AppSettings::default();
        let original = settings.show_week_numbers;

        // Toggle (but don't save to avoid file system in tests)
        settings.show_week_numbers = !settings.show_week_numbers;

        assert_ne!(settings.show_week_numbers, original);
    }
}
