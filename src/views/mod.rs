mod day;
mod main_view;
mod month;
mod sidebar;
mod week;

pub use day::render_day_view;
pub use main_view::render_main_content;
pub use month::render_month_view;
pub use sidebar::render_sidebar;
pub use week::render_week_view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarView {
    Month,
    Week,
    Day,
}
