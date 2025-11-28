use super::calendar_source::{CalendarInfo, CalendarSource, CalendarType};
use super::config::CalendarManagerConfig;
use crate::caldav::CalendarEvent;
use crate::storage::LocalStorage;
use std::error::Error;
use std::path::PathBuf;

/// A local calendar stored on disk
#[derive(Debug)]
pub struct LocalCalendar {
    info: CalendarInfo,
    storage: LocalStorage,
    storage_path: PathBuf,
}

impl LocalCalendar {
    /// Create a new local calendar
    pub fn new(id: String, name: String) -> Self {
        let mut info = CalendarInfo::new(id.clone(), name, CalendarType::Local);
        let storage_path = Self::get_calendar_path(&id);
        let storage = LocalStorage::load_from_file(&storage_path).unwrap_or_default();

        // Load saved configuration if it exists
        if let Ok(config) = CalendarManagerConfig::load() {
            if let Some(saved) = config.get_calendar(&id) {
                info.name = saved.name.clone();
                info.color = saved.color.clone();
                info.enabled = saved.enabled;
            }
        }

        LocalCalendar {
            info,
            storage,
            storage_path,
        }
    }

    /// Create a local calendar with a custom color (used for defaults)
    pub fn with_color(id: String, name: String, color: String) -> Self {
        let mut calendar = Self::new(id.clone(), name);

        // Only set the default color if no saved config exists
        let has_saved_config = CalendarManagerConfig::load()
            .ok()
            .map(|cfg| cfg.get_calendar(&id).is_some())
            .unwrap_or(false);

        if !has_saved_config {
            calendar.info.color = color;
        }

        calendar
    }

    /// Get the file path for a calendar by ID
    fn get_calendar_path(id: &str) -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("sol-calendar");
        path.push("calendars");
        std::fs::create_dir_all(&path).ok();
        path.push(format!("{}.json", id));
        path
    }

    /// Get all events from this calendar
    pub fn get_events(&self) -> &[CalendarEvent] {
        self.storage.get_events()
    }

    /// Get events for a specific date
    pub fn get_events_for_date(&self, date: chrono::NaiveDate) -> Vec<&CalendarEvent> {
        self.storage.get_events_for_date(date)
    }

    /// Get events for a specific month
    pub fn get_events_for_month(&self, year: i32, month: u32) -> Vec<&CalendarEvent> {
        self.storage.get_events_for_month(year, month)
    }
}

impl CalendarSource for LocalCalendar {
    fn info(&self) -> &CalendarInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut CalendarInfo {
        &mut self.info
    }

    fn fetch_events(&self) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
        Ok(self.storage.get_events().to_vec())
    }

    fn add_event(&mut self, event: CalendarEvent) -> Result<(), Box<dyn Error>> {
        self.storage.add_event(event);
        Ok(())
    }

    fn update_event(&mut self, event: CalendarEvent) -> Result<(), Box<dyn Error>> {
        self.storage.update_event(event);
        Ok(())
    }

    fn delete_event(&mut self, uid: &str) -> Result<(), Box<dyn Error>> {
        self.storage.remove_event(uid);
        Ok(())
    }

    fn sync(&mut self) -> Result<(), Box<dyn Error>> {
        // For local calendars, sync means saving to disk
        self.storage.save_to_file(&self.storage_path)?;
        Ok(())
    }

    fn supports_read(&self) -> bool {
        true
    }

    fn supports_write(&self) -> bool {
        true
    }
}
