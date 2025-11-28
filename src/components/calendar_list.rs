use cosmic::widget::{column, container, row};
use cosmic::{widget, Element};

use crate::calendars::CalendarSource;
use crate::components::{render_color_indicator, render_quick_color_picker, COLOR_INDICATOR_SIZE};
use crate::message::Message;

/// Render the list of calendars with checkboxes and color pickers
pub fn render_calendar_list<'a>(
    calendars: &'a [Box<dyn CalendarSource>],
    color_picker_open: Option<&String>,
) -> Element<'a, Message> {
    let mut calendar_list = column()
        .spacing(8)
        .padding(12)
        .push(widget::text::body("Calendars").size(14));

    for calendar in calendars {
        let info = calendar.info();
        let is_enabled = calendar.is_enabled();
        let is_picker_open = color_picker_open.map(|id| id == &info.id).unwrap_or(false);

        // Use the color picker component for the indicator
        let color_indicator = render_color_indicator(
            info.id.clone(),
            &info.color,
            COLOR_INDICATOR_SIZE,
        );

        let calendar_row = row()
            .spacing(8)
            .push(widget::checkbox("", is_enabled).on_toggle(move |_| {
                Message::ToggleCalendar(info.id.clone())
            }))
            .push(color_indicator)
            .push(widget::text(&info.name));

        calendar_list = calendar_list.push(calendar_row);

        // Show inline color picker if this calendar's picker is open
        if is_picker_open {
            let color_picker = render_quick_color_picker(info.id.clone(), &info.color);

            calendar_list = calendar_list.push(
                container(color_picker)
                    .padding([4, 0, 4, 36]) // Indent to align with calendar name
            );
        }
    }

    calendar_list.into()
}
