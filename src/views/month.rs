use cosmic::iced::Length;
use cosmic::widget::{column, container, row};
use cosmic::{widget, Element};

use crate::components::render_day_cell;
use crate::message::Message;
use crate::models::CalendarState;
use crate::ui_constants::{FONT_SIZE_MEDIUM, PADDING_SMALL, PADDING_MONTH_GRID, SPACING_TINY, WEEKDAYS_FULL};

pub fn render_month_view(
    calendar_state: &CalendarState,
    selected_day: Option<u32>,
) -> Element<'static, Message> {
    let mut grid = column().spacing(SPACING_TINY).padding(PADDING_MONTH_GRID);

    // Weekday headers - use iterator to avoid repetition
    let mut header_row = row().spacing(SPACING_TINY);
    for weekday in WEEKDAYS_FULL {
        header_row = header_row.push(
            container(widget::text(weekday).size(FONT_SIZE_MEDIUM))
                .width(Length::Fill)
                .padding(PADDING_SMALL)
                .center_x(Length::Fill),
        );
    }

    grid = grid.push(header_row);

    // Use pre-calculated weeks from CalendarState cache
    for week in &calendar_state.weeks {
        let mut week_row = row().spacing(SPACING_TINY).height(Length::Fill);
        for day_opt in week {
            if let Some(day) = day_opt {
                let is_today = calendar_state.is_today(*day);
                let is_selected = selected_day == Some(*day);

                // Directly push cell without extra container wrapper
                week_row = week_row.push(
                    container(render_day_cell(*day, is_today, is_selected))
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
