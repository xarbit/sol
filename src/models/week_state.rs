use chrono::{Datelike, NaiveDate};

/// Cached week state for week view
#[derive(Debug, Clone, PartialEq)]
pub struct WeekState {
    pub year: i32,
    pub week_number: u32,
    pub days: Vec<NaiveDate>, // 7 days in the week (Mon-Sun)
    pub week_range_text: String, // Pre-formatted "Jan 1 - Jan 7, 2024"
    pub today: NaiveDate,
}

impl WeekState {
    /// Create a new WeekState for the week containing the given date
    pub fn new(date: NaiveDate) -> Self {
        let today = chrono::Local::now().date_naive();

        // Find Monday of the week containing the date
        let weekday = date.weekday();
        let days_since_monday = weekday.num_days_from_monday();
        let monday = date - chrono::Duration::days(days_since_monday as i64);

        // Build the 7 days of the week
        let mut days = Vec::with_capacity(7);
        for i in 0..7 {
            days.push(monday + chrono::Duration::days(i));
        }

        let year = date.year();
        let week_number = date.iso_week().week();

        // Format week range text
        let first_day = &days[0];
        let last_day = &days[6];

        let week_range_text = if first_day.month() == last_day.month() {
            format!(
                "{} {} - {}, {}",
                first_day.format("%b"),
                first_day.day(),
                last_day.day(),
                first_day.year()
            )
        } else if first_day.year() == last_day.year() {
            format!(
                "{} {} - {} {}, {}",
                first_day.format("%b"),
                first_day.day(),
                last_day.format("%b"),
                last_day.day(),
                first_day.year()
            )
        } else {
            format!(
                "{} {}, {} - {} {}, {}",
                first_day.format("%b"),
                first_day.day(),
                first_day.year(),
                last_day.format("%b"),
                last_day.day(),
                last_day.year()
            )
        };

        WeekState {
            year,
            week_number,
            days,
            week_range_text,
            today,
        }
    }

    /// Create WeekState for current week
    pub fn current() -> Self {
        Self::new(chrono::Local::now().date_naive())
    }

    /// Navigate to previous week
    pub fn previous(&self) -> Self {
        Self::new(self.days[0] - chrono::Duration::days(7))
    }

    /// Navigate to next week
    pub fn next(&self) -> Self {
        Self::new(self.days[0] + chrono::Duration::days(7))
    }

    /// Check if a given date is today
    pub fn is_today(&self, date: &NaiveDate) -> bool {
        *date == self.today
    }

    /// Check if this week contains today
    pub fn contains_today(&self) -> bool {
        self.days.iter().any(|d| *d == self.today)
    }
}
