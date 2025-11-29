use chrono::{Datelike, NaiveDate};
use crate::app::CosmicCalendar;
use crate::message::Message;
use crate::views::CalendarView;
use cosmic::app::Task;

/// Handle all application messages and update state
pub fn handle_message(app: &mut CosmicCalendar, message: Message) -> Task<Message> {
    match message {
        Message::ChangeView(view) => {
            // When changing views, sync views to the selected_date so the new view
            // shows the period containing the anchor date
            app.current_view = view;
            app.sync_views_to_selected_date();
        }
        Message::PreviousPeriod => {
            handle_previous_period(app);
        }
        Message::NextPeriod => {
            handle_next_period(app);
        }
        Message::Today => {
            // Today button navigates to today in all views
            app.navigate_to_today();
        }
        Message::SelectDay(year, month, day) => {
            // Set the selected date - this syncs all views automatically
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                app.set_selected_date(date);
            }
        }
        Message::ToggleSidebar => {
            app.show_sidebar = !app.show_sidebar;
        }
        Message::ToggleSearch => {
            app.show_search = !app.show_search;
        }
        Message::ToggleWeekNumbers => {
            app.settings.show_week_numbers = !app.settings.show_week_numbers;
            // Save settings to persist the change
            app.settings.save().ok();
        }
        Message::ToggleCalendar(id) => {
            handle_toggle_calendar(app, id);
        }
        Message::OpenColorPicker(id) => {
            app.color_picker_open = Some(id);
        }
        Message::ChangeCalendarColor(id, color) => {
            handle_change_calendar_color(app, id, color);
        }
        Message::CloseColorPicker => {
            app.color_picker_open = None;
        }
        Message::MiniCalendarPrevMonth => {
            app.navigate_mini_calendar_previous();
        }
        Message::MiniCalendarNextMonth => {
            app.navigate_mini_calendar_next();
        }
        Message::NewEvent => {
            // TODO: Open new event dialog
            println!("New Event requested");
        }
        Message::Settings => {
            // TODO: Open settings dialog
            println!("Settings requested");
        }
        Message::About => {
            app.core.window.show_context = !app.core.window.show_context;
        }
        Message::LaunchUrl(url) => {
            // Open URL in default browser
            let _ = open::that(&url);
        }
        Message::ToggleContextDrawer => {
            app.core.window.show_context = !app.core.window.show_context;
        }
        Message::Surface(action) => {
            return cosmic::task::message(cosmic::Action::Cosmic(
                cosmic::app::Action::Surface(action),
            ));
        }
    }

    Task::none()
}

/// Direction for period navigation
enum NavigationDirection {
    Previous,
    Next,
}

/// Handle period navigation (previous or next) based on current view
fn handle_period_navigation(app: &mut CosmicCalendar, direction: NavigationDirection) {
    let multiplier: i32 = match direction {
        NavigationDirection::Previous => -1,
        NavigationDirection::Next => 1,
    };

    let new_date = match app.current_view {
        CalendarView::Year => {
            // Move by one year
            navigate_by_year(app.selected_date, multiplier)
        }
        CalendarView::Month => {
            // Move by one month
            navigate_by_month(app.selected_date, multiplier)
        }
        CalendarView::Week => {
            // Move by one week
            Some(app.selected_date + chrono::Duration::days(7 * multiplier as i64))
        }
        CalendarView::Day => {
            // Move by one day
            Some(app.selected_date + chrono::Duration::days(multiplier as i64))
        }
    };

    if let Some(date) = new_date {
        app.set_selected_date(date);
    }
}

/// Navigate a date by the given number of years, handling edge cases like Feb 29
fn navigate_by_year(date: NaiveDate, years: i32) -> Option<NaiveDate> {
    let new_year = date.year() + years;
    // Try the same day first, then fall back to day 28 for edge cases
    NaiveDate::from_ymd_opt(new_year, date.month(), date.day().min(28))
        .or_else(|| NaiveDate::from_ymd_opt(new_year, date.month(), 28))
}

/// Navigate a date by the given number of months, handling edge cases
fn navigate_by_month(date: NaiveDate, months: i32) -> Option<NaiveDate> {
    let total_months = date.year() * 12 + date.month() as i32 - 1 + months;
    let new_year = total_months / 12;
    let new_month = (total_months % 12 + 1) as u32;

    // Try the same day first, then fall back to day 28 for edge cases
    NaiveDate::from_ymd_opt(new_year, new_month, date.day().min(28))
        .or_else(|| NaiveDate::from_ymd_opt(new_year, new_month, 28))
}

/// Handle previous period navigation
fn handle_previous_period(app: &mut CosmicCalendar) {
    handle_period_navigation(app, NavigationDirection::Previous);
}

/// Handle next period navigation
fn handle_next_period(app: &mut CosmicCalendar) {
    handle_period_navigation(app, NavigationDirection::Next);
}

/// Toggle a calendar's enabled state and save configuration
fn handle_toggle_calendar(app: &mut CosmicCalendar, id: String) {
    if let Some(calendar) = app
        .calendar_manager
        .sources_mut()
        .iter_mut()
        .find(|c| c.info().id == id)
    {
        calendar.set_enabled(!calendar.is_enabled());
    }
    // Save configuration after toggle
    app.calendar_manager.save_config().ok();
}

/// Change a calendar's color and save configuration
fn handle_change_calendar_color(app: &mut CosmicCalendar, id: String, color: String) {
    if let Some(calendar) = app
        .calendar_manager
        .sources_mut()
        .iter_mut()
        .find(|c| c.info().id == id)
    {
        calendar.info_mut().color = color;
    }
    // Save configuration after color change
    app.calendar_manager.save_config().ok();
    // Close the color picker after selection
    app.color_picker_open = None;
}
