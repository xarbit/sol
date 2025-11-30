//! Services layer for business logic and middleware.
//!
//! This module contains service components that sit between the UI/update layer
//! and the protocol/storage layer.

mod event_handler;

pub use event_handler::EventHandler;
