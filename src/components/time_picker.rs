//! Time picker component with hour and minute selection
//!
//! A reusable time picker widget that displays scrollable columns for
//! selecting hours (0-23) and minutes (0, 5, 10, ... 55).

use chrono::{NaiveTime, Timelike};
use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row, scrollable, text};
use cosmic::Element;

use crate::fl;

/// Render a time picker popup with hour and minute columns
///
/// This is a generic widget that can be used with any message type.
///
/// # Arguments
/// * `current_time` - The currently selected time (None defaults to 09:00)
/// * `on_hour_change` - Callback when hour is selected, receives hour (0-23)
/// * `on_minute_change` - Callback when minute is selected, receives minute (0, 5, 10, ...)
/// * `on_apply` - Callback when the Apply button is pressed to close the picker
///
/// # Example
/// ```ignore
/// let picker = render_time_picker(
///     Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
///     |hour| MyMessage::HourChanged(hour),
///     |minute| MyMessage::MinuteChanged(minute),
///     MyMessage::CloseTimePicker,
/// );
/// ```
pub fn render_time_picker<'a, M: Clone + 'static>(
    current_time: Option<NaiveTime>,
    on_hour_change: impl Fn(u32) -> M + 'static,
    on_minute_change: impl Fn(u32) -> M + 'static,
    on_apply: M,
) -> Element<'a, M> {
    let current_hour = current_time.map(|t| t.hour()).unwrap_or(9);
    let current_minute = current_time.map(|t| t.minute()).unwrap_or(0);

    // Hour column (0-23) - add right padding to avoid scrollbar overlap
    let mut hour_buttons = column().spacing(2).padding([4, 12, 4, 4]);
    for hour in 0..24u32 {
        let is_selected = hour == current_hour;
        let on_hour = on_hour_change(hour);
        hour_buttons = hour_buttons.push(
            button::custom(
                text(format!("{:02}", hour))
                    .size(14)
                    .center()
            )
            .on_press(on_hour)
            .width(Length::Fixed(40.0))
            .padding([4, 8])
            .class(if is_selected {
                cosmic::theme::Button::Suggested
            } else {
                cosmic::theme::Button::Text
            }),
        );
    }

    // Minute column (0, 5, 10, ... 55) - 5 minute increments, add right padding
    let mut minute_buttons = column().spacing(2).padding([4, 12, 4, 4]);
    for minute in (0..60u32).step_by(5) {
        let is_selected = (current_minute / 5) * 5 == minute;
        let on_minute = on_minute_change(minute);
        minute_buttons = minute_buttons.push(
            button::custom(
                text(format!("{:02}", minute))
                    .size(14)
                    .center()
            )
            .on_press(on_minute)
            .width(Length::Fixed(40.0))
            .padding([4, 8])
            .class(if is_selected {
                cosmic::theme::Button::Suggested
            } else {
                cosmic::theme::Button::Text
            }),
        );
    }

    // Scrollable columns for hours and minutes - wider to accommodate scrollbar
    let hour_scroll = scrollable(hour_buttons)
        .height(Length::Fixed(200.0))
        .width(Length::Fixed(62.0));

    let minute_scroll = scrollable(minute_buttons)
        .height(Length::Fixed(200.0))
        .width(Length::Fixed(62.0));

    // Labels
    let hour_label = text("Hour").size(12);
    let minute_label = text("Min").size(12);

    let hour_col = column()
        .spacing(4)
        .align_x(cosmic::iced::Alignment::Center)
        .push(hour_label)
        .push(hour_scroll);

    let minute_col = column()
        .spacing(4)
        .align_x(cosmic::iced::Alignment::Center)
        .push(minute_label)
        .push(minute_scroll);

    // Combine hour and minute columns with separator
    let picker_content = row()
        .spacing(8)
        .push(hour_col)
        .push(text(":").size(20))
        .push(minute_col);

    // Apply button at bottom
    let apply_btn = button::suggested(fl!("button-apply"))
        .on_press(on_apply);

    let full_content = column()
        .spacing(12)
        .align_x(cosmic::iced::Alignment::Center)
        .push(picker_content)
        .push(apply_btn);

    container(full_content)
        .padding(8)
        .max_width(180.0)
        .style(|theme: &cosmic::Theme| {
            let cosmic = theme.cosmic();
            container::Style {
                background: Some(cosmic::iced::Background::Color(
                    cosmic.background.base.into(),
                )),
                border: cosmic::iced::Border {
                    radius: cosmic.corner_radii.radius_m.into(),
                    width: 1.0,
                    color: cosmic.bg_divider().into(),
                },
                shadow: cosmic::iced::Shadow {
                    color: cosmic::iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: cosmic::iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            }
        })
        .into()
}
