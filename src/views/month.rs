use cosmic::iced::Length;
use cosmic::widget::{column, container, mouse_area, row};
use cosmic::{widget, Element};

use crate::components::render_day_cell;
use crate::message::Message;
use crate::models::CalendarState;

pub fn render_month_view(
    calendar_state: &CalendarState,
    selected_day: Option<u32>,
) -> Element<'static, Message> {
    let mut grid = column().spacing(1).padding(20);

    // Weekday headers
    let header_row = row()
        .spacing(1)
        .push(
            container(widget::text("Monday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Tuesday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Wednesday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Thursday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Friday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Saturday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        )
        .push(
            container(widget::text("Sunday").size(12))
                .width(Length::Fill)
                .padding(8)
                .center_x(Length::Fill),
        );

    grid = grid.push(header_row);

    // Use pre-calculated weeks from CalendarState cache
    for week in &calendar_state.weeks {
        let mut week_row = row().spacing(1).height(Length::Fill);
        for day_opt in week {
            let cell = if let Some(day) = day_opt {
                let is_today = calendar_state.is_today(*day);
                let is_selected = selected_day == Some(*day);

                render_day_cell(*day, is_today, is_selected)
            } else {
                // Empty cell - simplified structure
                container(widget::text(""))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            };

            week_row = week_row.push(container(cell).width(Length::Fill).height(Length::Fill));
        }
        grid = grid.push(week_row);
    }

    container(grid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
