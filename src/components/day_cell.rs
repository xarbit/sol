use cosmic::iced::{alignment, Length};
use cosmic::widget::{container, mouse_area};
use cosmic::{widget, Element};

use crate::message::Message;
use crate::styles::{today_outlined_style, selected_day_style, day_cell_style};
use crate::ui_constants::PADDING_DAY_CELL;

pub fn render_day_cell(
    year: i32,
    month: u32,
    day: u32,
    is_today: bool,
    is_selected: bool,
    is_weekend: bool,
) -> Element<'static, Message> {
    // Create single mouse_area with styled container - reduces widget count
    let day_text = if is_today || is_selected {
        widget::text::title4(day.to_string())
    } else {
        widget::text(day.to_string())
    };

    // Build styled container based on state
    let styled_container = if is_today {
        // Today: outlined with accent color border (not filled)
        container(day_text)
            .padding(PADDING_DAY_CELL)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(|theme: &cosmic::Theme| today_outlined_style(theme))
    } else if is_selected {
        // Selected: filled with accent color
        container(day_text)
            .padding(PADDING_DAY_CELL)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(|theme: &cosmic::Theme| selected_day_style(theme))
    } else {
        // Normal day - light border with optional weekend background
        container(day_text)
            .padding(PADDING_DAY_CELL)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(move |_theme: &cosmic::Theme| day_cell_style(is_weekend))
    };

    // Single mouse_area wrapping the styled container
    mouse_area(styled_container)
        .on_press(Message::SelectDay(year, month, day))
        .into()
}
