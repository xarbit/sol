//! Export Handler - Import/Export operations for calendar data.
//!
//! This handler manages importing and exporting calendar data in various formats,
//! primarily iCalendar (.ics) format.

use crate::caldav::{AlertTime, CalendarEvent, RepeatFrequency, TravelTime};
use crate::calendars::CalendarManager;
use chrono::{DateTime, Utc};
use icalendar::{Calendar, Component, DatePerhapsTime, Event, EventLike};
use log::{debug, error, info, warn};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Result type for export operations
#[allow(dead_code)] // Part of export API for future use
pub type ExportResult<T> = Result<T, ExportError>;

/// Error types for export operations
#[allow(dead_code)] // Part of export API for future use
#[derive(Debug)]
pub enum ExportError {
    /// File I/O error
    IoError(String),
    /// Invalid file format
    FormatError(String),
    /// Parse error
    ParseError(String),
    /// Calendar not found
    CalendarNotFound(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ExportError::FormatError(msg) => write!(f, "Format error: {}", msg),
            ExportError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ExportError::CalendarNotFound(id) => write!(f, "Calendar not found: {}", id),
        }
    }
}

impl Error for ExportError {}

/// Export Handler - import/export operations.
#[allow(dead_code)] // Foundation for future import/export feature
pub struct ExportHandler;

impl ExportHandler {
    /// Export a single event to iCalendar format
    #[allow(dead_code)] // Part of export API
    pub fn event_to_ical(event: &CalendarEvent) -> Calendar {
        debug!("ExportHandler: Converting event '{}' (uid={}) to iCal", event.summary, event.uid);

        let mut calendar = Calendar::new();

        let mut ical_event = Event::new();
        ical_event.summary(&event.summary);
        ical_event.uid(&event.uid);
        ical_event.starts(event.start);
        ical_event.ends(event.end);

        if let Some(ref location) = event.location {
            ical_event.location(location);
        }

        if let Some(ref notes) = event.notes {
            ical_event.description(notes);
        }

        if let Some(ref url) = event.url {
            ical_event.url(url);
        }

        calendar.push(ical_event);
        debug!("ExportHandler: Event conversion complete");
        calendar
    }

    /// Export all events from a calendar to iCalendar format
    #[allow(dead_code)] // Part of export API
    pub fn calendar_to_ical(
        manager: &CalendarManager,
        calendar_id: &str,
    ) -> ExportResult<Calendar> {
        info!("ExportHandler: Exporting calendar '{}' to iCal format", calendar_id);

        let calendar = manager
            .sources()
            .iter()
            .find(|c| c.info().id == calendar_id)
            .ok_or_else(|| {
                error!("ExportHandler: Calendar '{}' not found", calendar_id);
                ExportError::CalendarNotFound(calendar_id.to_string())
            })?;

        let events = calendar
            .fetch_events()
            .map_err(|e| {
                error!("ExportHandler: Failed to fetch events: {}", e);
                ExportError::IoError(e.to_string())
            })?;

        debug!("ExportHandler: Found {} events to export", events.len());

        let mut ical = Calendar::new();

        for event in events {
            let mut ical_event = Event::new();
            ical_event.summary(&event.summary);
            ical_event.uid(&event.uid);
            ical_event.starts(event.start);
            ical_event.ends(event.end);

            if let Some(ref location) = event.location {
                ical_event.location(location);
            }

            if let Some(ref notes) = event.notes {
                ical_event.description(notes);
            }

            if let Some(ref url) = event.url {
                ical_event.url(url);
            }

            ical.push(ical_event);
        }

        info!("ExportHandler: Successfully exported calendar '{}'", calendar_id);
        Ok(ical)
    }

    /// Export a calendar to an iCalendar file
    #[allow(dead_code)] // Part of export API
    pub fn export_to_file<P: AsRef<Path>>(
        manager: &CalendarManager,
        calendar_id: &str,
        path: P,
    ) -> ExportResult<()> {
        info!("ExportHandler: Exporting calendar '{}' to file {:?}", calendar_id, path.as_ref());

        let ical = Self::calendar_to_ical(manager, calendar_id)?;
        let ical_string = ical.to_string();

        fs::write(&path, ical_string).map_err(|e| {
            error!("ExportHandler: Failed to write file: {}", e);
            ExportError::IoError(e.to_string())
        })?;

        info!("ExportHandler: Successfully exported to {:?}", path.as_ref());
        Ok(())
    }

    /// Export all calendars to a single iCalendar file
    #[allow(dead_code)] // Part of export API
    pub fn export_all_to_file<P: AsRef<Path>>(
        manager: &CalendarManager,
        path: P,
    ) -> ExportResult<()> {
        info!("ExportHandler: Exporting all calendars to file {:?}", path.as_ref());

        let mut combined = Calendar::new();
        let mut total_events = 0;

        for calendar in manager.sources() {
            if !calendar.is_enabled() {
                debug!("ExportHandler: Skipping disabled calendar '{}'", calendar.info().name);
                continue;
            }

            if let Ok(events) = calendar.fetch_events() {
                debug!("ExportHandler: Adding {} events from '{}'", events.len(), calendar.info().name);
                for event in events {
                    let mut ical_event = Event::new();
                    ical_event.summary(&event.summary);
                    ical_event.uid(&event.uid);
                    ical_event.starts(event.start);
                    ical_event.ends(event.end);

                    if let Some(ref location) = event.location {
                        ical_event.location(location);
                    }

                    if let Some(ref notes) = event.notes {
                        ical_event.description(notes);
                    }

                    combined.push(ical_event);
                    total_events += 1;
                }
            }
        }

        let ical_string = combined.to_string();
        fs::write(&path, ical_string).map_err(|e| {
            error!("ExportHandler: Failed to write file: {}", e);
            ExportError::IoError(e.to_string())
        })?;

        info!("ExportHandler: Exported {} events to {:?}", total_events, path.as_ref());
        Ok(())
    }

    /// Read an iCalendar file (placeholder for future import functionality)
    #[allow(dead_code)] // Part of export API
    pub fn read_ical_file<P: AsRef<Path>>(path: P) -> ExportResult<String> {
        info!("ExportHandler: Reading iCal file {:?}", path.as_ref());
        fs::read_to_string(&path).map_err(|e| {
            error!("ExportHandler: Failed to read file: {}", e);
            ExportError::IoError(e.to_string())
        })
    }

    /// Parse an iCalendar file and return a list of events
    #[allow(dead_code)] // Part of import API
    pub fn parse_ical_file<P: AsRef<Path>>(path: P) -> ExportResult<Vec<CalendarEvent>> {
        info!("ExportHandler: Parsing iCal file {:?}", path.as_ref());
        let ical_string = Self::read_ical_file(&path)?;
        Self::parse_ical_string(&ical_string)
    }

    /// Parse an iCalendar string and return a list of events
    #[allow(dead_code)] // Part of import API
    pub fn parse_ical_string(ical_str: &str) -> ExportResult<Vec<CalendarEvent>> {
        debug!("ExportHandler: Parsing iCal string ({} bytes)", ical_str.len());

        let calendar = ical_str.parse::<Calendar>().map_err(|e| {
            error!("ExportHandler: Failed to parse iCalendar: {}", e);
            ExportError::ParseError(e.to_string())
        })?;

        let mut events = Vec::new();
        for component in calendar.components {
            if let icalendar::CalendarComponent::Event(ical_event) = component {
                match Self::ical_event_to_calendar_event(&ical_event) {
                    Ok(event) => events.push(event),
                    Err(e) => {
                        warn!("ExportHandler: Skipping invalid event: {}", e);
                        continue;
                    }
                }
            }
        }

        info!("ExportHandler: Successfully parsed {} events", events.len());
        Ok(events)
    }

    /// Convert an icalendar::Event to a CalendarEvent
    #[allow(dead_code)] // Part of import API
    fn ical_event_to_calendar_event(ical_event: &Event) -> ExportResult<CalendarEvent> {
        // Extract UID (required)
        let uid = ical_event
            .get_uid()
            .ok_or_else(|| {
                error!("ExportHandler: Event missing UID");
                ExportError::ParseError("Event missing UID".to_string())
            })?
            .to_string();

        // Extract summary (required)
        let summary = ical_event
            .get_summary()
            .ok_or_else(|| {
                error!("ExportHandler: Event uid={} missing summary", uid);
                ExportError::ParseError(format!("Event uid={} missing summary", uid))
            })?
            .to_string();

        // Extract start time (required)
        let start_prop = ical_event.get_start().ok_or_else(|| {
            error!("ExportHandler: Event uid={} missing start time", uid);
            ExportError::ParseError(format!("Event uid={} missing start time", uid))
        })?;

        let (start, all_day) = match start_prop {
            DatePerhapsTime::DateTime(cal_dt) => {
                // Convert CalendarDateTime to chrono DateTime<Utc>
                match cal_dt {
                    icalendar::CalendarDateTime::Floating(dt) => {
                        (DateTime::from_naive_utc_and_offset(dt, Utc), false)
                    }
                    icalendar::CalendarDateTime::Utc(dt) => (dt, false),
                    icalendar::CalendarDateTime::WithTimezone { date_time, .. } => {
                        (DateTime::from_naive_utc_and_offset(date_time, Utc), false)
                    }
                }
            }
            DatePerhapsTime::Date(date) => {
                // All-day event - use midnight UTC
                let dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| ExportError::ParseError("Invalid date".to_string()))?;
                (DateTime::from_naive_utc_and_offset(dt, Utc), true)
            }
        };

        // Extract end time (default to start + 1 hour)
        let end = if let Some(end_prop) = ical_event.get_end() {
            match end_prop {
                DatePerhapsTime::DateTime(cal_dt) => {
                    // Convert CalendarDateTime to chrono DateTime<Utc>
                    match cal_dt {
                        icalendar::CalendarDateTime::Floating(dt) => {
                            DateTime::from_naive_utc_and_offset(dt, Utc)
                        }
                        icalendar::CalendarDateTime::Utc(dt) => dt,
                        icalendar::CalendarDateTime::WithTimezone { date_time, .. } => {
                            DateTime::from_naive_utc_and_offset(date_time, Utc)
                        }
                    }
                }
                DatePerhapsTime::Date(date) => {
                    let dt = date
                        .and_hms_opt(0, 0, 0)
                        .ok_or_else(|| ExportError::ParseError("Invalid end date".to_string()))?;
                    DateTime::from_naive_utc_and_offset(dt, Utc)
                }
            }
        } else {
            start + chrono::Duration::hours(1)
        };

        // Extract optional fields
        let location = ical_event.get_location().map(|s| s.to_string());
        let notes = ical_event.get_description().map(|s| s.to_string());
        let url = ical_event.get_url().map(|s| s.to_string());

        debug!("ExportHandler: Parsed event uid={}", uid);

        Ok(CalendarEvent {
            uid,
            summary,
            location,
            all_day,
            start,
            end,
            travel_time: TravelTime::None,
            repeat: RepeatFrequency::Never,
            repeat_until: None,
            exception_dates: vec![],
            invitees: vec![],
            alert: AlertTime::None,
            alert_second: None,
            attachments: vec![],
            url,
            notes,
        })
    }

    /// Import events from a file into a specific calendar
    /// Returns the number of events imported (skips duplicates based on UID)
    #[allow(dead_code)] // Part of import API
    pub fn import_from_file<P: AsRef<Path>>(
        manager: &mut CalendarManager,
        calendar_id: &str,
        path: P,
    ) -> ExportResult<usize> {
        info!("ExportHandler: Importing events from {:?} into calendar '{}'", path.as_ref(), calendar_id);

        // Parse the file
        let events = Self::parse_ical_file(&path)?;

        // Get the target calendar
        let calendar = manager
            .sources_mut()
            .iter_mut()
            .find(|c| c.info().id == calendar_id)
            .ok_or_else(|| {
                error!("ExportHandler: Calendar '{}' not found", calendar_id);
                ExportError::CalendarNotFound(calendar_id.to_string())
            })?;

        // Get existing event UIDs to detect duplicates
        let existing_events = calendar.fetch_events().map_err(|e| {
            error!("ExportHandler: Failed to fetch existing events: {}", e);
            ExportError::IoError(e.to_string())
        })?;
        let existing_uids: std::collections::HashSet<_> =
            existing_events.iter().map(|e| e.uid.as_str()).collect();

        // Import events, skipping duplicates
        let mut imported_count = 0;
        let total_events = events.len();
        for event in events {
            if existing_uids.contains(event.uid.as_str()) {
                debug!("ExportHandler: Skipping duplicate event uid={}", event.uid);
                continue;
            }

            calendar.add_event(event).map_err(|e| {
                error!("ExportHandler: Failed to add event: {}", e);
                ExportError::IoError(e.to_string())
            })?;
            imported_count += 1;
        }

        info!("ExportHandler: Successfully imported {} events (skipped {} duplicates)",
              imported_count, total_events - imported_count);
        Ok(imported_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caldav::{AlertTime, RepeatFrequency, TravelTime};
    use chrono::{TimeZone, Utc};

    fn create_test_event() -> CalendarEvent {
        CalendarEvent {
            uid: "test-export-1".to_string(),
            summary: "Test Export Event".to_string(),
            location: Some("Test Location".to_string()),
            all_day: false,
            start: Utc.with_ymd_and_hms(2025, 12, 1, 10, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2025, 12, 1, 11, 0, 0).unwrap(),
            travel_time: TravelTime::None,
            repeat: RepeatFrequency::Never,
            repeat_until: None,
            exception_dates: vec![],
            invitees: vec![],
            alert: AlertTime::None,
            alert_second: None,
            attachments: vec![],
            url: None,
            notes: Some("Test notes".to_string()),
        }
    }

    #[test]
    fn test_event_to_ical() {
        let event = create_test_event();
        let ical = ExportHandler::event_to_ical(&event);
        let ical_string = ical.to_string();

        assert!(ical_string.contains("BEGIN:VCALENDAR"));
        assert!(ical_string.contains("BEGIN:VEVENT"));
        assert!(ical_string.contains("Test Export Event"));
        assert!(ical_string.contains("END:VEVENT"));
        assert!(ical_string.contains("END:VCALENDAR"));
    }
}
