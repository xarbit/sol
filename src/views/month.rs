use chrono::{Datelike, NaiveDate};
use cosmic::iced::{alignment, Length};
use cosmic::widget::{column, container, row};
use cosmic::{widget, Element};

use crate::components::{render_day_cell, render_day_cell_with_events, DayCellConfig, DisplayEvent};
use crate::fl;
use crate::locale::LocalePreferences;
use crate::localized_names;
use crate::message::Message;
use crate::models::CalendarState;
use crate::ui_constants::{
    FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, PADDING_SMALL, PADDING_MONTH_GRID,
    SPACING_TINY, WEEK_NUMBER_WIDTH
};

/// Events grouped by day for display in the month view
pub struct MonthViewEvents<'a> {
    /// Events for each day, keyed by day number (1-31)
    pub events_by_day: &'a std::collections::HashMap<u32, Vec<DisplayEvent>>,
    /// Quick event editing state: (date, text, calendar_color)
    pub quick_event: Option<(NaiveDate, &'a str, &'a str)>,
}

pub fn render_month_view<'a>(
    calendar_state: &CalendarState,
    selected_day: Option<u32>,
    locale: &LocalePreferences,
    show_week_numbers: bool,
    events: Option<MonthViewEvents<'a>>,
) -> Element<'a, Message> {
    let mut grid = column().spacing(SPACING_TINY).padding(PADDING_MONTH_GRID);

    // Weekday headers with optional week number column
    let mut header_row = row().spacing(SPACING_TINY);

    // Week number header (only if enabled)
    if show_week_numbers {
        header_row = header_row.push(
            container(widget::text(fl!("week-abbr")).size(FONT_SIZE_SMALL))
                .width(Length::Fixed(WEEK_NUMBER_WIDTH))
                .padding(PADDING_SMALL)
                .align_y(alignment::Vertical::Center)
        );
    }

    // Weekday headers - use iterator to avoid repetition
    let weekday_names = localized_names::get_weekday_names_full();
    for weekday in weekday_names {
        header_row = header_row.push(
            container(widget::text(weekday).size(FONT_SIZE_MEDIUM))
                .width(Length::Fill)
                .padding(PADDING_SMALL)
                .center_x(Length::Fill),
        );
    }

    grid = grid.push(header_row);

    // Get week numbers for the month
    let week_numbers = calendar_state.week_numbers();

    // Use pre-calculated weeks from CalendarState cache
    for (week_index, week) in calendar_state.weeks.iter().enumerate() {
        let mut week_row = row().spacing(SPACING_TINY).height(Length::Fill);

        // Week number cell (only if enabled)
        if show_week_numbers {
            let week_number = week_numbers.get(week_index).copied().unwrap_or(0);
            week_row = week_row.push(
                container(
                    widget::text(format!("{}", week_number))
                        .size(FONT_SIZE_SMALL)
                )
                .width(Length::Fixed(WEEK_NUMBER_WIDTH))
                .height(Length::Fill)
                .padding(PADDING_SMALL)
                .align_y(alignment::Vertical::Center)
            );
        }

        // Day cells
        for day_opt in week {
            if let Some(day) = day_opt {
                let is_today = calendar_state.is_today(*day);
                let is_selected = selected_day == Some(*day);
                let weekday = calendar_state.get_weekday(*day);
                let is_weekend = locale.is_weekend(weekday);

                // Get events for this day if available
                let day_events: Vec<DisplayEvent> = events
                    .as_ref()
                    .and_then(|e| e.events_by_day.get(day))
                    .cloned()
                    .unwrap_or_default();

                // Check if quick event input should be shown for this day
                let quick_event_data: Option<(String, String)> = events.as_ref().and_then(|e| {
                    e.quick_event.as_ref().and_then(|(date, text, color)| {
                        if date.day() == *day
                            && date.month() == calendar_state.month
                            && date.year() == calendar_state.year
                        {
                            Some((text.to_string(), color.to_string()))
                        } else {
                            None
                        }
                    })
                });

                // Use new cell with events if we have event data, otherwise simple cell
                let cell = if events.is_some() {
                    render_day_cell_with_events(DayCellConfig {
                        year: calendar_state.year,
                        month: calendar_state.month,
                        day: *day,
                        is_today,
                        is_selected,
                        is_weekend,
                        events: day_events,
                        quick_event: quick_event_data,
                    })
                } else {
                    render_day_cell(
                        calendar_state.year,
                        calendar_state.month,
                        *day,
                        is_today,
                        is_selected,
                        is_weekend,
                    )
                };

                week_row = week_row.push(
                    container(cell)
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            } else {
                // Empty cell - minimal structure
                week_row = week_row.push(
                    container(widget::text(""))
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            }
        }
        grid = grid.push(week_row);
    }

    container(grid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
