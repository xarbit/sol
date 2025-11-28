mod app;
mod cache;
mod caldav;
mod calendars;
mod components;
mod layout;
mod message;
mod models;
mod storage;
mod styles;
mod ui_constants;
mod update;
mod views;

use app::CosmicCalendar;
use cosmic::app::Settings;

/// Application entry point
pub fn main() -> cosmic::iced::Result {
    cosmic::app::run::<CosmicCalendar>(Settings::default(), ())
}
