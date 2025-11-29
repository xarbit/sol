//! Validation logic for application data
//!
//! This module contains pure validation functions that are separate from
//! UI and state management concerns.
//!
//! These functions are a foundation for future refactoring to move validation
//! logic out of update handlers.

// Allow unused for now - these are foundation functions for future refactoring
#![allow(dead_code)]

use chrono::NaiveDate;

/// Validate and parse a date string in YYYY-MM-DD format
pub fn parse_date(input: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d").ok()
}

/// Validate that an event title is not empty
pub fn validate_event_title(title: &str) -> bool {
    !title.trim().is_empty()
}

/// Validate an email address (basic check)
pub fn validate_email(email: &str) -> bool {
    let email = email.trim();
    !email.is_empty() && email.contains('@') && email.contains('.')
}

/// Ensure end date is not before start date, adjusting if necessary
pub fn ensure_end_after_start(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    if end < start {
        start
    } else {
        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-01-15").is_some());
        assert!(parse_date("invalid").is_none());
        assert!(parse_date("01-15-2024").is_none());
    }

    #[test]
    fn test_validate_event_title() {
        assert!(validate_event_title("Meeting"));
        assert!(!validate_event_title(""));
        assert!(!validate_event_title("   "));
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid"));
        assert!(!validate_email(""));
    }
}
