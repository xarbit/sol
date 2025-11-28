mod day;
mod month;
mod sidebar;
mod week;

pub use month::render_month_view;
pub use sidebar::render_sidebar;

// Re-export these for future use
#[allow(unused_imports)]
pub use day::render_day_view;
#[allow(unused_imports)]
pub use week::render_week_view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarView {
    Month,
    Week,
    Day,
}
