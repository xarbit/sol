use crate::app::CosmicCalendar;
use crate::message::Message;
use crate::styles;
use crate::ui_constants::SIDEBAR_WIDTH;
use cosmic::iced::widget::stack;
use cosmic::iced::{alignment, Length};
use cosmic::widget::{container, divider, row};
use cosmic::Element;

/// Render the responsive layout (sidebar + main content)
pub fn render_layout(app: &CosmicCalendar) -> Element<'_, Message> {
    let is_condensed = app.core.is_condensed();

    // Build base layout with sidebar inline when appropriate
    let base_content = if !is_condensed && app.show_sidebar {
        // Large screen: sidebar inline on left
        render_desktop_with_sidebar(app)
    } else if !is_condensed {
        // Large screen, sidebar hidden
        app.render_main_content()
    } else {
        // Condensed screen: just main content as base
        app.render_main_content()
    };

    // In condensed mode with sidebar toggled on, show it as overlay
    if is_condensed && app.show_sidebar {
        render_mobile_with_overlay(app, base_content)
    } else {
        base_content
    }
}

/// Render desktop layout with inline sidebar
fn render_desktop_with_sidebar(app: &CosmicCalendar) -> Element<'_, Message> {
    row()
        .spacing(0)
        .push(app.render_sidebar())
        .push(divider::vertical::default())
        .push(app.render_main_content())
        .into()
}

/// Render mobile layout with overlay sidebar
fn render_mobile_with_overlay<'a>(
    app: &'a CosmicCalendar,
    base_content: Element<'a, Message>,
) -> Element<'a, Message> {
    let overlay_sidebar = container(
        container(app.render_sidebar()).style(styles::overlay_sidebar_style),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .align_x(alignment::Horizontal::Left);

    stack![base_content, overlay_sidebar].into()
}
