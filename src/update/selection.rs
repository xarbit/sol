//! Selection handlers for multi-day event creation
//!
//! These handlers manage the drag selection state for selecting multiple days
//! in the calendar view, enabling Apple Calendar-style multi-day event creation.

use chrono::NaiveDate;
use log::debug;

use crate::app::CosmicCalendar;

/// Start a drag selection at the given date (mouse press on day cell)
pub fn handle_selection_start(app: &mut CosmicCalendar, date: NaiveDate) {
    debug!("handle_selection_start: Starting selection at {}", date);
    app.selection_state.start(date);
}

/// Update the selection end point (mouse move while dragging)
pub fn handle_selection_update(app: &mut CosmicCalendar, date: NaiveDate) {
    if app.selection_state.is_active {
        debug!("handle_selection_update: Updating selection to {}", date);
        app.selection_state.update(date);
    }
}

/// End the selection (mouse release)
/// If multi-day selection, opens the event dialog with the date range
/// If single day, triggers a regular day click
pub fn handle_selection_end(app: &mut CosmicCalendar) {
    debug!("handle_selection_end: Ending selection");

    if let Some(range) = app.selection_state.end() {
        if range.is_multi_day() {
            debug!(
                "handle_selection_end: Multi-day selection from {} to {} ({} days)",
                range.start,
                range.end,
                range.day_count()
            );
            // Open event dialog with the selected date range
            open_event_dialog_with_range(app, range.start, range.end);
        } else {
            debug!("handle_selection_end: Single day selection at {}", range.start);
            // Single day - just select the day (same as clicking)
            app.set_selected_date(range.start);
        }
    }
}

/// Cancel the current selection
pub fn handle_selection_cancel(app: &mut CosmicCalendar) {
    debug!("handle_selection_cancel: Cancelling selection");
    app.selection_state.cancel();
}

/// Open the event dialog with a pre-filled date range
fn open_event_dialog_with_range(app: &mut CosmicCalendar, start: NaiveDate, end: NaiveDate) {
    use chrono::NaiveTime;
    use cosmic::widget::{calendar::CalendarModel, text_editor};
    use crate::app::EventDialogState;
    use crate::caldav::{AlertTime, RepeatFrequency, TravelTime};

    debug!("open_event_dialog_with_range: Opening dialog for {} to {}", start, end);

    // Use selected calendar or first available
    let calendar_id = app
        .selected_calendar_id
        .clone()
        .or_else(|| {
            app.calendar_manager
                .sources()
                .first()
                .map(|c| c.info().id.clone())
        })
        .unwrap_or_default();

    // For multi-day events from drag selection, default to all-day
    app.event_dialog = Some(EventDialogState {
        editing_uid: None,
        title: String::new(),
        location: String::new(),
        all_day: true, // Multi-day selections default to all-day events
        start_date: start,
        start_date_input: start.format("%Y-%m-%d").to_string(),
        start_time: None,
        start_time_input: "09:00".to_string(),
        end_date: end,
        end_date_input: end.format("%Y-%m-%d").to_string(),
        end_time: None,
        end_time_input: "10:00".to_string(),
        travel_time: TravelTime::None,
        repeat: RepeatFrequency::Never,
        calendar_id,
        invitees: vec![],
        invitee_input: String::new(),
        alert: AlertTime::None,
        alert_second: None,
        attachments: vec![],
        url: String::new(),
        notes_content: text_editor::Content::new(),
        editing_field: None,
        start_date_picker_open: false,
        start_date_calendar: CalendarModel::new(start, start),
        end_date_picker_open: false,
        end_date_calendar: CalendarModel::new(end, end),
        start_time_picker_open: false,
        end_time_picker_open: false,
    });
}
