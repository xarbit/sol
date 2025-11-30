use crate::app::CosmicCalendar;
use crate::components::{render_calendar_dialog, render_delete_calendar_dialog, render_event_dialog};
use crate::message::Message;
use crate::styles;
use crate::ui_constants::SIDEBAR_WIDTH;
use cosmic::iced::widget::stack;
use cosmic::iced::{alignment, Length};
use cosmic::widget::{container, divider, mouse_area, row};
use cosmic::Element;

/// Render the responsive layout (sidebar + main content)
#[allow(deprecated)]
pub fn render_layout(app: &CosmicCalendar) -> Element<'_, Message> {
    let is_condensed = app.core.is_condensed();
    // Check both legacy fields and active_dialog for whether a dialog is open
    let has_dialog_open = app.active_dialog.is_open()
        || app.event_dialog.is_some()
        || app.calendar_dialog.is_some()
        || app.delete_calendar_dialog.is_some()
        || app.color_picker_open.is_some();

    // Build base layout with sidebar inline when appropriate
    let base_content = if !is_condensed && app.show_sidebar {
        // Large screen: sidebar inline on left
        render_desktop_with_sidebar(app)
    } else if !is_condensed {
        // Large screen, sidebar hidden - wrap main content for dialog close
        wrap_main_content_for_dialog_close(app.render_main_content(), has_dialog_open)
    } else {
        // Condensed screen: just main content as base - wrap for dialog close
        wrap_main_content_for_dialog_close(app.render_main_content(), has_dialog_open)
    };

    // In condensed mode with sidebar toggled on, show it as overlay
    let with_sidebar = if is_condensed && app.show_sidebar {
        render_mobile_with_overlay(app, base_content)
    } else {
        base_content
    };

    // Show dialog overlays based on active_dialog state
    render_dialog_overlay(app, with_sidebar)
}

/// Render dialog overlay based on legacy dialog fields
/// Note: This uses the legacy dialog fields which are kept in sync with ActiveDialog
/// in the update handlers. This allows gradual migration.
fn render_dialog_overlay<'a>(
    app: &'a CosmicCalendar,
    base: Element<'a, Message>,
) -> Element<'a, Message> {
    // Show dialog overlays if any are open (event dialog takes priority)
    #[allow(deprecated)]
    if let Some(ref dialog_state) = app.event_dialog {
        let dialog = render_event_dialog(dialog_state, app.calendar_manager.sources());
        return stack![base, dialog].into();
    }

    #[allow(deprecated)]
    if let Some(ref dialog_state) = app.calendar_dialog {
        let dialog = render_calendar_dialog(dialog_state);
        return stack![base, dialog].into();
    }

    #[allow(deprecated)]
    if let Some(ref dialog_state) = app.delete_calendar_dialog {
        let dialog = render_delete_calendar_dialog(dialog_state);
        return stack![base, dialog].into();
    }

    base
}

/// Wrap main content with a mouse_area to close dialogs when clicking outside
fn wrap_main_content_for_dialog_close<'a>(
    content: Element<'a, Message>,
    dialog_open: bool,
) -> Element<'a, Message> {
    if dialog_open {
        mouse_area(content)
            .on_press(Message::CloseDialog)
            .into()
    } else {
        content
    }
}

/// Render desktop layout with inline sidebar
fn render_desktop_with_sidebar(app: &CosmicCalendar) -> Element<'_, Message> {
    let main_content = wrap_main_content_for_dialog_close(
        app.render_main_content(),
        app.active_dialog.is_open(),
    );

    row()
        .spacing(0)
        .push(app.render_sidebar())
        .push(divider::vertical::default())
        .push(main_content)
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
