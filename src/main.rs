mod app;
mod cache;
mod caldav;
mod calendars;
mod color_constants;
mod components;
mod database;
mod dialogs;
mod keyboard;
mod layout;
mod layout_constants;
mod locale;
mod localize;
mod localized_names;
mod logging;
mod menu_action;
mod message;
mod models;
mod protocols;
mod services;
mod settings;
mod storage;
mod styles;
mod ui_constants;
mod update;
mod validation;
mod views;

use app::CosmicCalendar;
use cosmic::app::Settings;
use log::info;

/// Application entry point
pub fn main() -> cosmic::iced::Result {
    // Initialize centralized logging
    logging::init();

    info!("Sol Calendar starting...");

    // Initialize localization system
    localize::init();

    info!("Localization initialized, launching application");
    cosmic::app::run::<CosmicCalendar>(Settings::default(), ())
}
