use cosmic::widget::{button, row};
use cosmic::{widget, Element};

use crate::message::Message;
use crate::ui_constants::{ICON_NEXT, ICON_PREVIOUS, SPACING_MEDIUM, PADDING_SMALL, PADDING_STANDARD};

/// Render the calendar toolbar with navigation controls
pub fn render_toolbar(period_text: &str) -> Element<'_, Message> {
    row()
        .padding(PADDING_STANDARD)
        .spacing(SPACING_MEDIUM)
        .push(
            button::icon(widget::icon::from_name(ICON_PREVIOUS))
                .on_press(Message::PreviousPeriod)
                .padding(PADDING_SMALL)
        )
        .push(
            button::icon(widget::icon::from_name(ICON_NEXT))
                .on_press(Message::NextPeriod)
                .padding(PADDING_SMALL)
        )
        .push(widget::text::title4(period_text))
        .into()
}
