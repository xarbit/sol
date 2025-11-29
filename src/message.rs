use crate::views::CalendarView;

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(CalendarView),
    PreviousPeriod,
    NextPeriod,
    Today,
    SelectDay(i32, u32, u32), // (year, month, day)
    ToggleSidebar,
    /// Triggered on window resize to sync sidebar with condensed state
    WindowResized,
    ToggleSearch,
    ToggleWeekNumbers,
    ToggleCalendar(String),
    OpenColorPicker(String),
    ChangeCalendarColor(String, String),
    CloseColorPicker,
    MiniCalendarPrevMonth,
    MiniCalendarNextMonth,
    NewEvent,
    ImportICal,
    ExportICal,
    Settings,
    About,
    LaunchUrl(String),
    ToggleContextDrawer,
    Surface(cosmic::surface::Action),
}
