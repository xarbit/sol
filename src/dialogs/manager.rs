//! Dialog Manager - Centralized dialog/popup state management.
//!
//! This module provides a unified system for managing all dialogs, popups,
//! and modal overlays in the application. It ensures:
//!
//! - Only one dialog can be open at a time
//! - Opening a new dialog automatically closes any existing one
//! - Escape key closes the current dialog
//! - Consistent focus management across the app
//!
//! # Architecture
//!
//! ```text
//! User Action (click, keyboard)
//!         │
//!         ▼
//! Message::Dialog(DialogAction)
//!         │
//!         ▼
//! DialogManager::handle_action()
//!         │
//!         ├── Opens new dialog (closes existing)
//!         ├── Closes current dialog
//!         └── Updates dialog state
//! ```
//!
//! # Note on Event Dialogs
//!
//! Event dialogs are currently managed through the legacy `event_dialog` field
//! in `CosmicCalendar` because `text_editor::Content` doesn't implement `Clone`.
//! The centralized `Message::CloseDialog` handler closes all legacy dialog fields.

use log::{debug, info};

/// Identifies which dialog is currently active.
/// Only one dialog can be open at a time.
///
/// Note: Event dialogs are not included here because they contain
/// `text_editor::Content` which doesn't implement Clone/PartialEq.
/// They are managed through the legacy `event_dialog` field.
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    /// No dialog is open
    None,
    /// Color picker for a specific calendar
    ColorPicker {
        calendar_id: String,
    },
    /// Create new calendar dialog
    CalendarCreate {
        name: String,
        color: String,
    },
    /// Edit existing calendar dialog
    CalendarEdit {
        calendar_id: String,
        name: String,
        color: String,
    },
    /// Delete calendar confirmation dialog
    CalendarDelete {
        calendar_id: String,
        calendar_name: String,
    },
    /// Event dialog is open (state managed by legacy field)
    /// This variant exists to track that an event dialog is open,
    /// but the actual data is in `app.event_dialog`
    EventDialogOpen,
}

impl Default for ActiveDialog {
    fn default() -> Self {
        Self::None
    }
}

impl ActiveDialog {
    /// Check if any dialog is currently open
    pub fn is_open(&self) -> bool {
        !matches!(self, ActiveDialog::None)
    }

    /// Check if a specific dialog type is open
    pub fn is_color_picker(&self) -> bool {
        matches!(self, ActiveDialog::ColorPicker { .. })
    }

    pub fn is_calendar_dialog(&self) -> bool {
        matches!(
            self,
            ActiveDialog::CalendarCreate { .. }
                | ActiveDialog::CalendarEdit { .. }
                | ActiveDialog::CalendarDelete { .. }
        )
    }

    pub fn is_event_dialog(&self) -> bool {
        matches!(self, ActiveDialog::EventDialogOpen)
    }

    /// Get the color picker calendar ID if open
    pub fn color_picker_calendar_id(&self) -> Option<&str> {
        match self {
            ActiveDialog::ColorPicker { calendar_id } => Some(calendar_id),
            _ => None,
        }
    }
}

/// Actions that can be performed on dialogs
#[derive(Debug, Clone)]
pub enum DialogAction {
    /// Close any open dialog
    Close,
    /// Close dialog and perform cleanup/confirm action
    CloseAndConfirm,
    /// Open color picker for a calendar
    OpenColorPicker(String),
    /// Open create calendar dialog
    OpenCalendarCreate { default_color: String },
    /// Open edit calendar dialog
    OpenCalendarEdit {
        calendar_id: String,
        name: String,
        color: String,
    },
    /// Open delete calendar confirmation
    OpenCalendarDelete {
        calendar_id: String,
        calendar_name: String,
    },
    /// Mark event dialog as open (actual state is in legacy field)
    MarkEventDialogOpen,
    /// Update calendar dialog name
    CalendarNameChanged(String),
    /// Update calendar dialog color
    CalendarColorChanged(String),
}

/// Dialog Manager handles all dialog state transitions
pub struct DialogManager;

impl DialogManager {
    /// Open a new dialog, closing any existing one
    pub fn open(current: &mut ActiveDialog, new_dialog: ActiveDialog) {
        if current.is_open() {
            debug!("DialogManager: Closing existing dialog before opening new one");
        }
        info!("DialogManager: Opening {:?}", std::mem::discriminant(&new_dialog));
        *current = new_dialog;
    }

    /// Close the current dialog
    pub fn close(current: &mut ActiveDialog) {
        if current.is_open() {
            info!("DialogManager: Closing dialog");
            *current = ActiveDialog::None;
        }
    }

    /// Close dialog if Escape was pressed
    /// Returns true if a dialog was closed
    pub fn handle_escape(current: &mut ActiveDialog) -> bool {
        if current.is_open() {
            info!("DialogManager: Escape pressed, closing dialog");
            *current = ActiveDialog::None;
            true
        } else {
            false
        }
    }

    /// Handle a dialog action
    pub fn handle_action(current: &mut ActiveDialog, action: DialogAction) {
        match action {
            DialogAction::Close => {
                Self::close(current);
            }
            DialogAction::CloseAndConfirm => {
                // The confirm logic should be handled by the caller
                // This just signals the dialog should close after confirmation
                Self::close(current);
            }
            DialogAction::OpenColorPicker(calendar_id) => {
                Self::open(current, ActiveDialog::ColorPicker { calendar_id });
            }
            DialogAction::OpenCalendarCreate { default_color } => {
                Self::open(
                    current,
                    ActiveDialog::CalendarCreate {
                        name: String::new(),
                        color: default_color,
                    },
                );
            }
            DialogAction::OpenCalendarEdit {
                calendar_id,
                name,
                color,
            } => {
                Self::open(
                    current,
                    ActiveDialog::CalendarEdit {
                        calendar_id,
                        name,
                        color,
                    },
                );
            }
            DialogAction::OpenCalendarDelete {
                calendar_id,
                calendar_name,
            } => {
                Self::open(
                    current,
                    ActiveDialog::CalendarDelete {
                        calendar_id,
                        calendar_name,
                    },
                );
            }
            DialogAction::MarkEventDialogOpen => {
                Self::open(current, ActiveDialog::EventDialogOpen);
            }
            DialogAction::CalendarNameChanged(name) => {
                match current {
                    ActiveDialog::CalendarCreate { name: n, .. }
                    | ActiveDialog::CalendarEdit { name: n, .. } => {
                        *n = name;
                    }
                    _ => {}
                }
            }
            DialogAction::CalendarColorChanged(color) => {
                match current {
                    ActiveDialog::CalendarCreate { color: c, .. }
                    | ActiveDialog::CalendarEdit { color: c, .. } => {
                        *c = color;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_none_by_default() {
        let dialog = ActiveDialog::default();
        assert!(!dialog.is_open());
    }

    #[test]
    fn test_open_closes_existing() {
        let mut dialog = ActiveDialog::ColorPicker {
            calendar_id: "test".to_string(),
        };
        assert!(dialog.is_open());

        DialogManager::open(
            &mut dialog,
            ActiveDialog::CalendarCreate {
                name: String::new(),
                color: "#FF0000".to_string(),
            },
        );

        assert!(dialog.is_calendar_dialog());
        assert!(!dialog.is_color_picker());
    }

    #[test]
    fn test_escape_closes_dialog() {
        let mut dialog = ActiveDialog::ColorPicker {
            calendar_id: "test".to_string(),
        };

        let closed = DialogManager::handle_escape(&mut dialog);

        assert!(closed);
        assert!(!dialog.is_open());
    }

    #[test]
    fn test_escape_does_nothing_when_no_dialog() {
        let mut dialog = ActiveDialog::None;

        let closed = DialogManager::handle_escape(&mut dialog);

        assert!(!closed);
        assert!(!dialog.is_open());
    }
}
