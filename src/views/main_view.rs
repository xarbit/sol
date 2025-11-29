use cosmic::iced::Length;
use cosmic::widget::{column, divider};
use cosmic::Element;

use crate::cache::CalendarCache;
use crate::components;
use crate::locale::LocalePreferences;
use crate::message::Message;
use crate::models::{WeekState, DayState, YearState};
use crate::views::{self, CalendarView};

/// Render the main content area (toolbar + calendar view)
pub fn render_main_content<'a>(
    cache: &'a CalendarCache,
    week_state: &'a WeekState,
    day_state: &'a DayState,
    year_state: &'a YearState,
    locale: &'a LocalePreferences,
    current_view: CalendarView,
    selected_day: Option<u32>,
    show_week_numbers: bool,
) -> Element<'a, Message> {
    // Render toolbar - use appropriate text for each view
    let period_text: &str = match current_view {
        CalendarView::Year => &year_state.year_text,
        CalendarView::Week => &week_state.week_range_text,
        CalendarView::Day => &day_state.month_year_text,
        CalendarView::Month => cache.current_period_text(),
    };
    let toolbar = components::render_toolbar(period_text);

    // Render current calendar view
    let calendar_view = match current_view {
        CalendarView::Year => views::render_year_view(year_state, locale),
        CalendarView::Month => views::render_month_view(cache.current_state(), selected_day, locale, show_week_numbers),
        CalendarView::Week => views::render_week_view(week_state, locale),
        CalendarView::Day => views::render_day_view(day_state, locale),
    };

    column()
        .spacing(0)
        .push(toolbar)
        .push(divider::horizontal::default())
        .push(calendar_view)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
