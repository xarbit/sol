use cosmic::iced::{alignment, Background, Border, Color, Length};
use cosmic::widget::{container, mouse_area};
use cosmic::{widget, Element};

use crate::message::Message;

// Pre-defined border radius to avoid allocation on every render
const BORDER_RADIUS: [f32; 4] = [4.0, 4.0, 4.0, 4.0];
const LIGHT_BORDER_COLOR: Color = Color::from_rgba(0.5, 0.5, 0.5, 0.2);

pub fn render_day_cell(
    day: u32,
    is_today: bool,
    is_selected: bool,
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
            .padding([4, 8, 0, 0])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(|theme: &cosmic::Theme| container::Style {
                background: None,
                border: Border {
                    color: theme.cosmic().accent_color().into(),
                    width: 2.0,
                    radius: BORDER_RADIUS.into(),
                },
                ..Default::default()
            })
    } else if is_selected {
        // Selected: filled with accent color
        container(day_text)
            .padding([4, 8, 0, 0])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(|theme: &cosmic::Theme| container::Style {
                background: Some(Background::Color(theme.cosmic().accent_color().into())),
                border: Border {
                    radius: BORDER_RADIUS.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    } else {
        // Normal day - light border
        container(day_text)
            .padding([4, 8, 0, 0])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .style(|_theme: &cosmic::Theme| container::Style {
                background: None,
                border: Border {
                    color: LIGHT_BORDER_COLOR.into(),
                    width: 1.0,
                    radius: BORDER_RADIUS.into(),
                },
                ..Default::default()
            })
    };

    // Single mouse_area wrapping the styled container
    mouse_area(styled_container)
        .on_press(Message::SelectDay(day))
        .into()
}
