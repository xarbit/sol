//! Calendar management handlers (create, edit, delete, toggle, color)

use crate::app::{CalendarDialogMode, CalendarDialogState, CosmicCalendar, DeleteCalendarDialogState};
use crate::services::{CalendarHandler, NewCalendarData, UpdateCalendarData};
use log::{debug, error, info, warn};

/// Toggle a calendar's enabled state and save configuration
pub fn handle_toggle_calendar(app: &mut CosmicCalendar, id: String) {
    debug!("handle_toggle_calendar: Toggling calendar '{}'", id);

    match CalendarHandler::toggle_enabled(&mut app.calendar_manager, &id) {
        Ok(new_state) => {
            info!("Calendar '{}' toggled to enabled={}", id, new_state);
            // Refresh events to show/hide events from toggled calendar
            app.refresh_cached_events();
        }
        Err(e) => {
            error!("Failed to toggle calendar '{}': {}", id, e);
        }
    }
}

/// Change a calendar's color and save configuration
pub fn handle_change_calendar_color(app: &mut CosmicCalendar, id: String, color: String) {
    debug!("handle_change_calendar_color: Changing color for '{}' to '{}'", id, color);

    match CalendarHandler::change_color(&mut app.calendar_manager, &id, color.clone()) {
        Ok(()) => {
            info!("Calendar '{}' color changed to '{}'", id, color);
            // Close the color picker after selection
            app.color_picker_open = None;
            // Refresh events to update event colors in views
            app.refresh_cached_events();
            // Also update selected calendar color if this was the selected calendar
            app.update_selected_calendar_color();
        }
        Err(e) => {
            error!("Failed to change calendar color: {}", e);
        }
    }
}

/// Open the calendar dialog in Create mode
pub fn handle_open_calendar_dialog_create(app: &mut CosmicCalendar) {
    debug!("handle_open_calendar_dialog_create: Opening create dialog");

    let default_color = CalendarHandler::default_color();

    app.calendar_dialog = Some(CalendarDialogState {
        mode: CalendarDialogMode::Create,
        name: String::new(),
        color: default_color,
    });
}

/// Open the calendar dialog in Edit mode for a specific calendar
pub fn handle_open_calendar_dialog_edit(app: &mut CosmicCalendar, calendar_id: String) {
    debug!("handle_open_calendar_dialog_edit: Opening edit dialog for '{}'", calendar_id);

    match CalendarHandler::get_info(&app.calendar_manager, &calendar_id) {
        Ok((name, color, _enabled)) => {
            app.calendar_dialog = Some(CalendarDialogState {
                mode: CalendarDialogMode::Edit {
                    calendar_id: calendar_id.clone(),
                },
                name,
                color,
            });
        }
        Err(e) => {
            warn!("Cannot edit calendar '{}': {}", calendar_id, e);
        }
    }
}

/// Confirm the calendar dialog (Create or Edit)
pub fn handle_confirm_calendar_dialog(app: &mut CosmicCalendar) {
    let Some(dialog) = app.calendar_dialog.take() else {
        return;
    };

    let name = dialog.name.trim();
    if name.is_empty() {
        warn!("handle_confirm_calendar_dialog: Empty name, ignoring");
        return;
    }

    match dialog.mode {
        CalendarDialogMode::Create => {
            debug!("handle_confirm_calendar_dialog: Creating calendar '{}'", name);

            match CalendarHandler::create(
                &mut app.calendar_manager,
                NewCalendarData {
                    name: name.to_string(),
                    color: dialog.color,
                },
            ) {
                Ok(id) => {
                    info!("Calendar '{}' created with id '{}'", name, id);
                    // Select the new calendar
                    app.selected_calendar_id = Some(id);
                    app.update_selected_calendar_color();
                }
                Err(e) => {
                    error!("Failed to create calendar: {}", e);
                }
            }
        }
        CalendarDialogMode::Edit { calendar_id } => {
            debug!("handle_confirm_calendar_dialog: Updating calendar '{}'", calendar_id);

            match CalendarHandler::update(
                &mut app.calendar_manager,
                &calendar_id,
                UpdateCalendarData {
                    name: Some(name.to_string()),
                    color: Some(dialog.color),
                    enabled: None,
                },
            ) {
                Ok(()) => {
                    info!("Calendar '{}' updated", calendar_id);
                    // Refresh events to update colors
                    app.refresh_cached_events();
                    // Update selected calendar color if this was the selected calendar
                    app.update_selected_calendar_color();
                }
                Err(e) => {
                    error!("Failed to update calendar '{}': {}", calendar_id, e);
                }
            }
        }
    }
}

/// Open the delete calendar confirmation dialog for the selected calendar
pub fn handle_delete_selected_calendar(app: &mut CosmicCalendar) {
    let Some(ref calendar_id) = app.selected_calendar_id else {
        debug!("handle_delete_selected_calendar: No calendar selected");
        return;
    };
    handle_request_delete_calendar(app, calendar_id.clone());
}

/// Open the delete calendar confirmation dialog for a specific calendar
pub fn handle_request_delete_calendar(app: &mut CosmicCalendar, calendar_id: String) {
    debug!("handle_request_delete_calendar: Requesting delete for '{}'", calendar_id);

    // Get calendar info using the handler
    let calendar_name = match CalendarHandler::get_info(&app.calendar_manager, &calendar_id) {
        Ok((name, _, _)) => name,
        Err(_) => calendar_id.clone(),
    };

    app.delete_calendar_dialog = Some(DeleteCalendarDialogState {
        calendar_id,
        calendar_name,
    });
}

/// Confirm and delete the calendar
pub fn handle_confirm_delete_calendar(app: &mut CosmicCalendar) {
    let Some(dialog) = app.delete_calendar_dialog.take() else {
        return;
    };

    debug!("handle_confirm_delete_calendar: Deleting '{}'", dialog.calendar_id);

    match CalendarHandler::delete(&mut app.calendar_manager, &dialog.calendar_id) {
        Ok(()) => {
            info!("Calendar '{}' deleted", dialog.calendar_id);

            // If we deleted the selected calendar, select another one
            if app.selected_calendar_id.as_ref() == Some(&dialog.calendar_id) {
                app.selected_calendar_id = CalendarHandler::get_first_calendar_id(&app.calendar_manager);
                app.update_selected_calendar_color();
            }

            // Refresh events in case any events from the deleted calendar were displayed
            app.refresh_cached_events();
        }
        Err(e) => {
            error!("Failed to delete calendar '{}': {}", dialog.calendar_id, e);
        }
    }
}
