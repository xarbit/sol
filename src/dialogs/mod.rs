//! Dialog state and data structures
//!
//! This module contains dialog-related state structs, separating
//! business data from UI state.
//!
//! These types are a foundation for future refactoring to fully separate
//! UI state from business data in the dialog components.

// Allow unused for now - these are foundation types for future refactoring
#![allow(dead_code)]

mod event_dialog;
mod calendar_dialog;

pub use event_dialog::{EventDialogData, EventDialogUiState, EventDialogField};
pub use calendar_dialog::{CalendarDialogData, CalendarDialogMode};
