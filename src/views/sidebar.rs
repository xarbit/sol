use cosmic::iced::Length;
use cosmic::widget::{column, container, divider, scrollable};
use cosmic::Element;

use crate::calendars::CalendarSource;
use crate::components::{render_calendar_list, render_mini_calendar};
use crate::message::Message;
use crate::models::CalendarState;

pub fn render_sidebar<'a>(
    calendar_state: &CalendarState,
    calendars: &'a [Box<dyn CalendarSource>],
    selected_day: Option<u32>,
    color_picker_open: Option<&'a String>,
) -> Element<'a, Message> {
    let mini_calendar = render_mini_calendar(calendar_state, selected_day);

    // Use the calendar list component
    let calendars_section = render_calendar_list(calendars, color_picker_open);

    // Scrollable top section with calendars
    let scrollable_content = scrollable(
        column()
            .spacing(20)
            .padding(16)
            .push(calendars_section)
    );

    // Bottom section with mini calendar (fixed at bottom)
    let bottom_section = column()
        .spacing(0)
        .push(divider::horizontal::default())
        .push(container(mini_calendar).padding(16));

    // Combine: scrollable top + fixed bottom
    let sidebar_layout = column()
        .spacing(0)
        .push(container(scrollable_content).height(Length::Fill))
        .push(bottom_section);

    container(sidebar_layout)
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .into()
}
