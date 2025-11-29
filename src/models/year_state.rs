use crate::models::CalendarState;
use chrono::Datelike;

/// Cached year state for year view
#[derive(Debug, Clone, PartialEq)]
pub struct YearState {
    pub year: i32,
    pub year_text: String, // Pre-formatted year text for display
    pub months: Vec<CalendarState>, // 12 months worth of CalendarState
    pub today: (i32, u32, u32), // (year, month, day)
}

impl YearState {
    /// Create a new YearState for the given year
    pub fn new(year: i32) -> Self {
        let today = chrono::Local::now();
        let today_tuple = (today.year(), today.month(), today.day());

        // Generate CalendarState for all 12 months
        let months = (1..=12)
            .map(|month| CalendarState::new(year, month))
            .collect();

        YearState {
            year,
            year_text: format!("{}", year),
            months,
            today: today_tuple,
        }
    }

    /// Create YearState for current year
    pub fn current() -> Self {
        Self::new(chrono::Local::now().year())
    }

    /// Navigate to previous year
    pub fn previous(&self) -> Self {
        Self::new(self.year - 1)
    }

    /// Navigate to next year
    pub fn next(&self) -> Self {
        Self::new(self.year + 1)
    }

    /// Check if this year is the current year
    pub fn is_current_year(&self) -> bool {
        self.year == self.today.0
    }
}
