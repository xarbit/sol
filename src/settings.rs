use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Application-level settings that persist across sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub show_week_numbers: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            show_week_numbers: true, // Show week numbers by default
        }
    }
}

impl AppSettings {
    /// Load settings from disk
    pub fn load() -> Result<Self, io::Error> {
        let path = Self::settings_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)?;
        let settings: AppSettings = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(settings)
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), io::Error> {
        let path = Self::settings_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&path, json)?;
        Ok(())
    }

    /// Get the settings file path
    fn settings_path() -> PathBuf {
        let mut path = dirs::config_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("sol-calendar");
        path.push("settings.json");
        path
    }
}
