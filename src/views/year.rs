use cosmic::iced::{alignment, Border, Length};
use cosmic::widget::{column, container, row};
use cosmic::{widget, Element};

use crate::locale::LocalePreferences;
use crate::localized_names;
use crate::message::Message;
use crate::models::YearState;
use crate::ui_constants::{
    FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE, PADDING_SMALL, PADDING_TINY,
    SPACING_SMALL, SPACING_MEDIUM, SPACING_TINY, SPACING_XXS, COLOR_DAY_CELL_BORDER, BORDER_WIDTH_THIN
};

pub fn render_year_view(year_state: &YearState, locale: &LocalePreferences) -> Element<'static, Message> {
    let mut year_layout = column()
        .spacing(SPACING_MEDIUM)
        .padding(PADDING_SMALL)
        .width(Length::Fill)
        .height(Length::Fill);

    // Create a 3x4 grid of month mini-calendars (4 rows of 3 months each)
    for row_index in 0..4 {
        let mut month_row = row()
            .spacing(SPACING_MEDIUM)
            .width(Length::Fill)
            .height(Length::Fill);

        for col_index in 0..3 {
            let month_index = row_index * 3 + col_index;
            if month_index < 12 {
                let month_calendar = render_mini_month(&year_state.months[month_index], year_state, locale, month_index + 1);
                month_row = month_row.push(month_calendar);
            }
        }

        year_layout = year_layout.push(month_row);
    }

    container(year_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Render a single mini month calendar for the year view
fn render_mini_month(
    month_state: &crate::models::CalendarState,
    year_state: &YearState,
    _locale: &LocalePreferences,
    month: usize,
) -> Element<'static, Message> {
    let mut mini_calendar = column()
        .spacing(SPACING_TINY)
        .padding(PADDING_SMALL)
        .width(Length::Fill);

    // Month name header
    let month_name = localized_names::get_month_name(month as u32);
    mini_calendar = mini_calendar.push(
        container(widget::text(month_name).size(FONT_SIZE_LARGE))
            .width(Length::Fill)
            .center_x(Length::Fill)
    );

    // Weekday headers (abbreviated, single letter for space)
    let weekday_names = localized_names::get_weekday_names_short();
    let mut header_row = row().spacing(SPACING_XXS);
    for weekday in &weekday_names {
        let first_char = weekday.chars().next().unwrap_or(' ').to_string();
        header_row = header_row.push(
            container(widget::text(first_char).size(FONT_SIZE_SMALL))
                .width(Length::Fill)
                .center_x(Length::Fill)
        );
    }
    mini_calendar = mini_calendar.push(header_row);

    // Day grid
    for week in &month_state.weeks {
        let mut week_row = row().spacing(SPACING_XXS);
        for day_opt in week {
            if let Some(day) = day_opt {
                let is_today = year_state.today == (year_state.year, month as u32, *day);

                let day_container = if is_today {
                    container(widget::text(format!("{}", day)).size(FONT_SIZE_SMALL))
                        .width(Length::Fill)
                        .padding(PADDING_TINY)
                        .center_x(Length::Fill)
                        .align_y(alignment::Vertical::Center)
                        .style(|theme: &cosmic::Theme| {
                            container::Style {
                                text_color: Some(theme.cosmic().accent_color().into()),
                                background: Some(cosmic::iced::Background::Color(
                                    theme.cosmic().accent_color().into()
                                )),
                                border: Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        })
                } else {
                    container(widget::text(format!("{}", day)).size(FONT_SIZE_SMALL))
                        .width(Length::Fill)
                        .padding(PADDING_TINY)
                        .center_x(Length::Fill)
                        .align_y(alignment::Vertical::Center)
                };

                week_row = week_row.push(day_container);
            } else {
                week_row = week_row.push(
                    container(widget::text(""))
                        .width(Length::Fill)
                        .padding(PADDING_TINY)
                );
            }
        }
        mini_calendar = mini_calendar.push(week_row);
    }

    container(mini_calendar)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &cosmic::Theme| {
            container::Style {
                border: Border {
                    width: BORDER_WIDTH_THIN,
                    color: COLOR_DAY_CELL_BORDER,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}
