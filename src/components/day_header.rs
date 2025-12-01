use cosmic::iced::alignment;
use cosmic::widget::{column, container};
use cosmic::{widget, Element};

use crate::message::Message;
use crate::styles::today_filled_style;
use crate::ui_constants::{SPACING_TINY, PADDING_SMALL, PADDING_MEDIUM};

/// Configuration for rendering a day header
pub struct DayHeaderConfig {
    pub day_name: String,
    pub day_number: String,
    pub is_today: bool,
    pub day_name_size: u16,
    pub day_number_size: u16,
    pub padding: u16,
}

impl DayHeaderConfig {
    /// Create a day header config for week view (smaller text)
    #[allow(dead_code)] // Reserved for future week view header configuration
    pub fn week_view(day_name: String, day_number: String, is_today: bool) -> Self {
        Self {
            day_name,
            day_number,
            is_today,
            day_name_size: crate::ui_constants::FONT_SIZE_SMALL,
            day_number_size: crate::ui_constants::FONT_SIZE_MEDIUM,
            padding: PADDING_SMALL,
        }
    }

    /// Create a day header config for day view (larger text)
    pub fn day_view(day_name: String, day_number: String, is_today: bool) -> Self {
        Self {
            day_name,
            day_number,
            is_today,
            day_name_size: crate::ui_constants::FONT_SIZE_MEDIUM,
            day_number_size: crate::ui_constants::FONT_SIZE_LARGE,
            padding: PADDING_MEDIUM,
        }
    }
}

/// Render a day header with day name and number
/// The number gets a filled accent background if it's today
pub fn render_day_header(config: DayHeaderConfig) -> Element<'static, Message> {
    let day_number_container = if config.is_today {
        container(
            widget::text(config.day_number).size(config.day_number_size)
        )
        .padding(config.padding)
        .style(|theme: &cosmic::Theme| today_filled_style(theme))
    } else {
        container(widget::text(config.day_number).size(config.day_number_size))
            .padding(config.padding)
    };

    column()
        .spacing(SPACING_TINY)
        .align_x(alignment::Horizontal::Center)
        .push(widget::text(config.day_name).size(config.day_name_size))
        .push(day_number_container)
        .into()
}
