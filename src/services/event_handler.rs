//! Event Handler - Middleware for event operations.
//!
//! The EventHandler sits between the UI/update layer and the protocol layer.
//! It provides a unified interface for event CRUD operations, handling:
//!
//! - Routing events to the correct protocol (local vs remote)
//! - Event validation before saving
//! - Sync and conflict resolution
//! - Cache invalidation coordination
//!
//! # Architecture
//!
//! ```text
//! UI (views/components)
//!         │
//!         ▼
//! Update Handlers (update/event.rs)
//!         │
//!         ▼
//! ┌───────────────────────────────────────┐
//! │         EventHandler (this module)     │
//! │  - Validates events                    │
//! │  - Routes to correct calendar/protocol │
//! │  - Handles sync                        │
//! └───────────────────────────────────────┘
//!         │
//!         ▼
//! CalendarManager → CalendarSource → Protocol
//! ```

use crate::caldav::CalendarEvent;
use crate::calendars::CalendarManager;
use std::error::Error;

/// Result type for event handler operations
pub type EventResult<T> = Result<T, EventError>;

/// Error types for event operations
#[derive(Debug)]
pub enum EventError {
    /// Event validation failed
    ValidationError(String),
    /// Calendar not found
    CalendarNotFound(String),
    /// Event not found
    EventNotFound(String),
    /// Storage/protocol error
    StorageError(String),
    /// Sync error
    SyncError(String),
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            EventError::CalendarNotFound(id) => write!(f, "Calendar not found: {}", id),
            EventError::EventNotFound(uid) => write!(f, "Event not found: {}", uid),
            EventError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            EventError::SyncError(msg) => write!(f, "Sync error: {}", msg),
        }
    }
}

impl Error for EventError {}

/// Event Handler - centralized middleware for event operations.
///
/// This struct provides methods for all event CRUD operations,
/// routing them through the appropriate calendar and protocol.
pub struct EventHandler;

impl EventHandler {
    /// Validate an event before saving.
    /// Returns Ok(()) if valid, or an error describing what's wrong.
    pub fn validate_event(event: &CalendarEvent) -> EventResult<()> {
        // Title/summary is required
        if event.summary.trim().is_empty() {
            return Err(EventError::ValidationError(
                "Event title is required".to_string(),
            ));
        }

        // End time must be after start time
        if event.end < event.start {
            return Err(EventError::ValidationError(
                "End time must be after start time".to_string(),
            ));
        }

        // UID must not be empty
        if event.uid.is_empty() {
            return Err(EventError::ValidationError(
                "Event UID is required".to_string(),
            ));
        }

        Ok(())
    }

    /// Add a new event to a calendar.
    ///
    /// This method:
    /// 1. Validates the event
    /// 2. Finds the target calendar
    /// 3. Adds the event via the calendar's protocol
    /// 4. Syncs the calendar
    pub fn add_event(
        calendar_manager: &mut CalendarManager,
        calendar_id: &str,
        event: CalendarEvent,
    ) -> EventResult<()> {
        // Validate event
        Self::validate_event(&event)?;

        // Find the calendar
        let calendar = calendar_manager
            .sources_mut()
            .iter_mut()
            .find(|c| c.info().id == calendar_id)
            .ok_or_else(|| EventError::CalendarNotFound(calendar_id.to_string()))?;

        // Add event via calendar (which routes to protocol)
        calendar
            .add_event(event)
            .map_err(|e| EventError::StorageError(e.to_string()))?;

        // Sync to persist
        calendar
            .sync()
            .map_err(|e| EventError::SyncError(e.to_string()))?;

        Ok(())
    }

    /// Update an existing event.
    ///
    /// This method:
    /// 1. Validates the event
    /// 2. Finds and removes the old event (from any calendar)
    /// 3. Adds the updated event to the target calendar
    /// 4. Syncs affected calendars
    pub fn update_event(
        calendar_manager: &mut CalendarManager,
        calendar_id: &str,
        event: CalendarEvent,
    ) -> EventResult<()> {
        // Validate event
        Self::validate_event(&event)?;

        let uid = event.uid.clone();

        // First, delete the old event from wherever it exists
        Self::delete_event(calendar_manager, &uid)?;

        // Add to the (possibly new) target calendar
        let calendar = calendar_manager
            .sources_mut()
            .iter_mut()
            .find(|c| c.info().id == calendar_id)
            .ok_or_else(|| EventError::CalendarNotFound(calendar_id.to_string()))?;

        calendar
            .add_event(event)
            .map_err(|e| EventError::StorageError(e.to_string()))?;

        calendar
            .sync()
            .map_err(|e| EventError::SyncError(e.to_string()))?;

        Ok(())
    }

    /// Delete an event by UID from all calendars.
    ///
    /// This searches all calendars for the event and deletes it.
    pub fn delete_event(
        calendar_manager: &mut CalendarManager,
        uid: &str,
    ) -> EventResult<()> {
        let mut deleted = false;

        for calendar in calendar_manager.sources_mut().iter_mut() {
            match calendar.delete_event(uid) {
                Ok(()) => {
                    // Sync after successful deletion
                    let _ = calendar.sync();
                    deleted = true;
                    break;
                }
                Err(_) => continue, // Event not in this calendar, try next
            }
        }

        if !deleted {
            // Not necessarily an error - event might have already been deleted
            // Just log it and continue
        }

        Ok(())
    }

    /// Find an event by UID across all calendars.
    ///
    /// Returns the event and the calendar ID it was found in.
    pub fn find_event(
        calendar_manager: &CalendarManager,
        uid: &str,
    ) -> EventResult<(CalendarEvent, String)> {
        for calendar in calendar_manager.sources() {
            if let Ok(events) = calendar.fetch_events() {
                if let Some(event) = events.iter().find(|e| e.uid == uid) {
                    return Ok((event.clone(), calendar.info().id.clone()));
                }
            }
        }

        Err(EventError::EventNotFound(uid.to_string()))
    }

    /// Sync all calendars.
    pub fn sync_all(calendar_manager: &mut CalendarManager) -> EventResult<()> {
        for calendar in calendar_manager.sources_mut().iter_mut() {
            if calendar.is_enabled() {
                calendar
                    .sync()
                    .map_err(|e| EventError::SyncError(e.to_string()))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caldav::{AlertTime, RepeatFrequency, TravelTime};
    use chrono::{TimeZone, Utc};

    fn create_test_event(uid: &str, summary: &str) -> CalendarEvent {
        CalendarEvent {
            uid: uid.to_string(),
            summary: summary.to_string(),
            location: None,
            all_day: false,
            start: Utc.with_ymd_and_hms(2025, 11, 30, 10, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2025, 11, 30, 11, 0, 0).unwrap(),
            travel_time: TravelTime::None,
            repeat: RepeatFrequency::Never,
            invitees: vec![],
            alert: AlertTime::None,
            alert_second: None,
            attachments: vec![],
            url: None,
            notes: None,
        }
    }

    #[test]
    fn test_validate_event_success() {
        let event = create_test_event("test-1", "Valid Event");
        assert!(EventHandler::validate_event(&event).is_ok());
    }

    #[test]
    fn test_validate_event_empty_title() {
        let event = create_test_event("test-1", "");
        let result = EventHandler::validate_event(&event);
        assert!(matches!(result, Err(EventError::ValidationError(_))));
    }

    #[test]
    fn test_validate_event_empty_uid() {
        let mut event = create_test_event("", "Test Event");
        event.uid = String::new();
        let result = EventHandler::validate_event(&event);
        assert!(matches!(result, Err(EventError::ValidationError(_))));
    }

    #[test]
    fn test_validate_event_end_before_start() {
        let mut event = create_test_event("test-1", "Test Event");
        event.end = Utc.with_ymd_and_hms(2025, 11, 30, 9, 0, 0).unwrap(); // Before start
        let result = EventHandler::validate_event(&event);
        assert!(matches!(result, Err(EventError::ValidationError(_))));
    }
}
