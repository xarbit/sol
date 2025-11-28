use chrono::{Datelike, NaiveDate};

/// Cached day state for day view
#[derive(Debug, Clone, PartialEq)]
pub struct DayState {
    pub date: NaiveDate,
    pub day_text: String,      // Pre-formatted "Monday"
    pub date_number: String,   // Pre-formatted "15"
    pub month_year_text: String, // Pre-formatted "January 2024"
    pub today: NaiveDate,
}

impl DayState {
    /// Create a new DayState for the given date
    pub fn new(date: NaiveDate) -> Self {
        let today = chrono::Local::now().date_naive();

        let day_text = format!("{}", date.format("%A")); // "Monday"
        let date_number = format!("{}", date.format("%d")); // "15"
        let month_year_text = format!("{}", date.format("%B %Y")); // "January 2024"

        DayState {
            date,
            day_text,
            date_number,
            month_year_text,
            today,
        }
    }

    /// Create DayState for today
    pub fn current() -> Self {
        Self::new(chrono::Local::now().date_naive())
    }

    /// Navigate to previous day
    pub fn previous(&self) -> Self {
        Self::new(self.date - chrono::Duration::days(1))
    }

    /// Navigate to next day
    pub fn next(&self) -> Self {
        Self::new(self.date + chrono::Duration::days(1))
    }

    /// Check if this day is today
    pub fn is_today(&self) -> bool {
        self.date == self.today
    }
}
