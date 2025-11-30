//! Selection state management for multi-day event creation.
//!
//! This module provides state tracking for drag selection across day cells,
//! enabling multi-day event creation similar to Apple Calendar.
//!
//! # Usage Flow
//!
//! ```text
//! User presses on day cell
//!         │
//!         ▼
//! Message::SelectionStart(date)
//!         │
//!         ▼
//! SelectionState { start: date, end: date, active: true }
//!         │
//!         ▼
//! User drags over other days
//!         │
//!         ▼
//! Message::SelectionUpdate(date)  [for each day entered]
//!         │
//!         ▼
//! SelectionState { start: original, end: current, active: true }
//!         │
//!         ▼
//! User releases mouse
//!         │
//!         ▼
//! Message::SelectionEnd
//!         │
//!         ├── Single day selected → Open quick event or select day
//!         └── Multiple days selected → Open event dialog with date range
//! ```

use chrono::NaiveDate;
use log::debug;

/// State for tracking drag selection across day cells.
///
/// This is a transient UI state, not a dialog, so it lives directly
/// in CosmicCalendar rather than in ActiveDialog.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// The date where the selection started (mouse press)
    pub start_date: Option<NaiveDate>,
    /// The current end date of the selection (follows mouse)
    pub end_date: Option<NaiveDate>,
    /// Whether a drag selection is currently active
    pub is_active: bool,
}

impl SelectionState {
    /// Create a new empty selection state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new selection at the given date
    pub fn start(&mut self, date: NaiveDate) {
        debug!("SelectionState: Starting selection at {}", date);
        self.start_date = Some(date);
        self.end_date = Some(date);
        self.is_active = true;
    }

    /// Update the selection end point (during drag)
    pub fn update(&mut self, date: NaiveDate) {
        if self.is_active {
            debug!("SelectionState: Updating selection end to {}", date);
            self.end_date = Some(date);
        }
    }

    /// End the selection and return the selected range if valid
    pub fn end(&mut self) -> Option<SelectionRange> {
        if !self.is_active {
            return None;
        }

        let range = self.get_range();
        debug!("SelectionState: Ending selection with range {:?}", range);
        self.reset();
        range
    }

    /// Cancel the current selection
    pub fn cancel(&mut self) {
        debug!("SelectionState: Cancelling selection");
        self.reset();
    }

    /// Reset the selection state
    pub fn reset(&mut self) {
        self.start_date = None;
        self.end_date = None;
        self.is_active = false;
    }

    /// Get the current selection range (normalized so start <= end)
    pub fn get_range(&self) -> Option<SelectionRange> {
        match (self.start_date, self.end_date) {
            (Some(start), Some(end)) => Some(SelectionRange::new(start, end)),
            _ => None,
        }
    }

    /// Check if a date is within the current selection
    pub fn contains(&self, date: NaiveDate) -> bool {
        self.get_range()
            .map(|r| r.contains(date))
            .unwrap_or(false)
    }

    /// Check if the current selection spans multiple days
    pub fn is_multi_day(&self) -> bool {
        self.get_range()
            .map(|r| r.is_multi_day())
            .unwrap_or(false)
    }
}

/// A normalized date range with start <= end
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    /// The earliest date in the selection
    pub start: NaiveDate,
    /// The latest date in the selection
    pub end: NaiveDate,
}

impl SelectionRange {
    /// Create a new range, normalizing so start <= end
    pub fn new(date1: NaiveDate, date2: NaiveDate) -> Self {
        if date1 <= date2 {
            Self { start: date1, end: date2 }
        } else {
            Self { start: date2, end: date1 }
        }
    }

    /// Check if a date is within this range (inclusive)
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.start && date <= self.end
    }

    /// Check if this range spans multiple days
    pub fn is_multi_day(&self) -> bool {
        self.start != self.end
    }

    /// Get the number of days in this range
    pub fn day_count(&self) -> i64 {
        (self.end - self.start).num_days() + 1
    }

    /// Iterate over all dates in this range
    pub fn dates(&self) -> impl Iterator<Item = NaiveDate> {
        let start = self.start;
        let end = self.end;
        (0..=((end - start).num_days())).map(move |i| start + chrono::Duration::days(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_state_start() {
        let mut state = SelectionState::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        state.start(date);

        assert!(state.is_active);
        assert_eq!(state.start_date, Some(date));
        assert_eq!(state.end_date, Some(date));
    }

    #[test]
    fn test_selection_state_update() {
        let mut state = SelectionState::new();
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 18).unwrap();

        state.start(start);
        state.update(end);

        assert!(state.is_active);
        assert_eq!(state.start_date, Some(start));
        assert_eq!(state.end_date, Some(end));
    }

    #[test]
    fn test_selection_state_end() {
        let mut state = SelectionState::new();
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 18).unwrap();

        state.start(start);
        state.update(end);
        let range = state.end();

        assert!(!state.is_active);
        assert!(range.is_some());
        let range = range.unwrap();
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }

    #[test]
    fn test_selection_range_normalized() {
        let early = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let late = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();

        // Forward selection
        let range1 = SelectionRange::new(early, late);
        assert_eq!(range1.start, early);
        assert_eq!(range1.end, late);

        // Backward selection (dragging up/left)
        let range2 = SelectionRange::new(late, early);
        assert_eq!(range2.start, early);
        assert_eq!(range2.end, late);
    }

    #[test]
    fn test_selection_range_contains() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let range = SelectionRange::new(start, end);

        assert!(range.contains(start));
        assert!(range.contains(end));
        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 12).unwrap()));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 1, 9).unwrap()));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 1, 16).unwrap()));
    }

    #[test]
    fn test_selection_range_is_multi_day() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let single = SelectionRange::new(date1, date1);
        assert!(!single.is_multi_day());

        let multi = SelectionRange::new(date1, date2);
        assert!(multi.is_multi_day());
    }

    #[test]
    fn test_selection_range_day_count() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let range = SelectionRange::new(start, end);

        assert_eq!(range.day_count(), 6); // 10, 11, 12, 13, 14, 15
    }

    #[test]
    fn test_selection_range_dates_iterator() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 12).unwrap();
        let range = SelectionRange::new(start, end);

        let dates: Vec<_> = range.dates().collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 1, 11).unwrap());
        assert_eq!(dates[2], NaiveDate::from_ymd_opt(2024, 1, 12).unwrap());
    }

    #[test]
    fn test_selection_state_cancel() {
        let mut state = SelectionState::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        state.start(date);
        assert!(state.is_active);

        state.cancel();
        assert!(!state.is_active);
        assert!(state.start_date.is_none());
        assert!(state.end_date.is_none());
    }
}
