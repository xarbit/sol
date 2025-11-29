use cosmic::widget::{button, row};
use cosmic::{widget, Element};

use crate::message::Message;
use crate::ui_constants::{ICON_NEXT, ICON_PREVIOUS, SPACING_MEDIUM, PADDING_TINY, PADDING_SMALL};

/// Render the calendar toolbar with navigation controls
/// primary_text is displayed bold, secondary_text is displayed in normal weight
pub fn render_toolbar(primary_text: &str, secondary_text: &str) -> Element<'static, Message> {
    let primary = primary_text.to_string();
    let secondary = secondary_text.to_string();

    row()
        .padding(PADDING_SMALL)
        .spacing(SPACING_MEDIUM)
        .align_y(cosmic::iced::Alignment::Center)
        .push(
            button::icon(widget::icon::from_name(ICON_PREVIOUS))
                .on_press(Message::PreviousPeriod)
                .padding(PADDING_TINY)
        )
        .push(
            button::icon(widget::icon::from_name(ICON_NEXT))
                .on_press(Message::NextPeriod)
                .padding(PADDING_TINY)
        )
        .push(
            row()
                .spacing(SPACING_MEDIUM)
                .align_y(cosmic::iced::Alignment::Center)
                .push(widget::text::title4(primary))
                .push(widget::text::body(secondary))
        )
        .into()
}
