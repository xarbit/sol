//! Callback definitions for calendar dialog component
//!
//! This module defines the callbacks that the calendar dialog needs,
//! allowing it to be generic over the message type.
//!
//! This is a foundation for future refactoring to make calendar_dialog.rs
//! generic over message type (like time_picker.rs).

// Allow unused for now - foundation for future refactoring
#![allow(dead_code)]

/// Callbacks for the calendar dialog component
pub struct CalendarDialogCallbacks<M> {
    pub on_name_changed: Box<dyn Fn(String) -> M>,
    pub on_color_changed: Box<dyn Fn(String) -> M>,
    pub on_confirm: M,
    pub on_cancel: M,
}

/// Callbacks for the delete calendar confirmation dialog
pub struct DeleteCalendarDialogCallbacks<M> {
    pub on_confirm: M,
    pub on_cancel: M,
}
