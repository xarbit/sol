use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row};
use cosmic::{widget, Element};

use crate::message::Message;
use crate::models::CalendarState;

pub fn render_mini_calendar(
    calendar_state: &CalendarState,
    selected_day: Option<u32>,
) -> Element<'static, Message> {
    let date = chrono::NaiveDate::from_ymd_opt(calendar_state.year, calendar_state.month, 1).unwrap();
    let month_year = format!("{}", date.format("%B %Y"));

    let header = row()
        .spacing(8)
        .push(
            button::icon(widget::icon::from_name("go-previous-symbolic"))
                .on_press(Message::MiniCalendarPrevMonth)
                .padding(4),
        )
        .push(container(widget::text::body(month_year).size(14)).width(Length::Fill))
        .push(
            button::icon(widget::icon::from_name("go-next-symbolic"))
                .on_press(Message::MiniCalendarNextMonth)
                .padding(4),
        );

    let mut grid = column().spacing(4);

    // Weekday headers (abbreviated)
    let header_row = row()
        .spacing(2)
        .push(widget::text("M").width(Length::Fill).size(11))
        .push(widget::text("T").width(Length::Fill).size(11))
        .push(widget::text("W").width(Length::Fill).size(11))
        .push(widget::text("T").width(Length::Fill).size(11))
        .push(widget::text("F").width(Length::Fill).size(11))
        .push(widget::text("S").width(Length::Fill).size(11))
        .push(widget::text("S").width(Length::Fill).size(11));

    grid = grid.push(header_row);

    // Use pre-calculated weeks from CalendarState
    for week in &calendar_state.weeks {
        let mut week_row = row().spacing(2);
        for day_opt in week {
            if let Some(day) = day_opt {
                let is_today = calendar_state.is_today(*day);
                let is_selected = selected_day == Some(*day);

                let day_button = if is_today {
                    widget::button::suggested((*day).to_string())
                        .on_press(Message::SelectDay(*day))
                        .padding(4)
                        .width(Length::Fixed(32.0))
                } else if is_selected {
                    widget::button::standard((*day).to_string())
                        .on_press(Message::SelectDay(*day))
                        .padding(4)
                        .width(Length::Fixed(32.0))
                } else {
                    widget::button::text((*day).to_string())
                        .on_press(Message::SelectDay(*day))
                        .padding(4)
                        .width(Length::Fixed(32.0))
                };
                week_row = week_row.push(day_button);
            } else {
                week_row = week_row.push(container(widget::text("")).width(Length::Fixed(32.0)));
            }
        }
        grid = grid.push(week_row);
    }

    column().spacing(12).push(header).push(grid).into()
}
