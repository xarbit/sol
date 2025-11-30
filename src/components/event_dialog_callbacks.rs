//! Callback definitions for event dialog component
//!
//! This module defines the callbacks that the event dialog needs,
//! allowing it to be generic over the message type.
//!
//! This is a foundation for future refactoring to make event_dialog.rs
//! generic over message type (like time_picker.rs).

// Allow unused for now - foundation for future refactoring
#![allow(dead_code)]

use chrono::NaiveDate;
use cosmic::widget::text_editor;

use crate::caldav::{AlertTime, RepeatFrequency, TravelTime};
use crate::app::EventDialogField;

/// Callbacks for the event dialog component
///
/// This struct contains all the message generators needed by the event dialog,
/// allowing it to be generic over the message type M.
pub struct EventDialogCallbacks<M> {
    // Basic fields
    pub on_title_changed: Box<dyn Fn(String) -> M>,
    pub on_location_changed: Box<dyn Fn(String) -> M>,
    pub on_toggle_edit: Box<dyn Fn(EventDialogField, bool) -> M>,

    // Date/time
    pub on_all_day_toggled: Box<dyn Fn(bool) -> M>,
    pub on_start_date_changed: Box<dyn Fn(NaiveDate) -> M>,
    pub on_toggle_start_date_picker: M,
    pub on_start_date_calendar_prev: M,
    pub on_start_date_calendar_next: M,
    pub on_start_time_hour_changed: Box<dyn Fn(u32) -> M>,
    pub on_start_time_minute_changed: Box<dyn Fn(u32) -> M>,
    pub on_toggle_start_time_picker: M,
    pub on_end_date_changed: Box<dyn Fn(NaiveDate) -> M>,
    pub on_toggle_end_date_picker: M,
    pub on_end_date_calendar_prev: M,
    pub on_end_date_calendar_next: M,
    pub on_end_time_hour_changed: Box<dyn Fn(u32) -> M>,
    pub on_end_time_minute_changed: Box<dyn Fn(u32) -> M>,
    pub on_toggle_end_time_picker: M,

    // Schedule
    pub on_travel_time_changed: Box<dyn Fn(TravelTime) -> M>,
    pub on_repeat_changed: Box<dyn Fn(RepeatFrequency) -> M>,

    // Calendar selection
    pub on_calendar_changed: Box<dyn Fn(String) -> M>,

    // Alert
    pub on_alert_changed: Box<dyn Fn(AlertTime) -> M>,

    // Invitees
    pub on_invitee_input_changed: Box<dyn Fn(String) -> M>,
    pub on_add_invitee: M,
    pub on_remove_invitee: Box<dyn Fn(usize) -> M>,

    // Additional
    pub on_url_changed: Box<dyn Fn(String) -> M>,
    pub on_notes_action: Box<dyn Fn(text_editor::Action) -> M>,

    // Dialog actions
    pub on_confirm: M,
    pub on_cancel: M,
}
