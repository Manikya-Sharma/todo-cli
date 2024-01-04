/// Manage the status of the TUI application
pub mod app;
/// Manage the args passed in cli
pub mod args;
/// Manage tui event handling
pub mod events;
/// File management module for storing anf managing tasks
pub mod files;
/// The current state of tasks as a buffer in tui
pub mod state;
/// wrapper for all tui related functions
pub mod tui;
/// user interface for the tui
pub mod ui;

/// Generic wrapper for Result type
///
/// This will allow easy propagation of errors till the main function
/// irrespectove of error type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Id = i32;

pub fn get_id() -> Id {
    fastrand::i32(1000..10000)
}
