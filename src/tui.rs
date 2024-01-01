use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    events::handle_events,
    files::{check_existing_metadata, enter_data_to_file, read_data_from_file},
    state::State,
    ui::ui,
    Result,
};

/// Run all the startup routines for creating an alternate terminal window
fn startup() -> Result<()> {
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

/// Shutdown routines which must be called to return the terminal back to its original state
fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Run the main loop for tui application
fn implement_tui() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut state;
    if check_existing_metadata() {
        state = match read_data_from_file() {
            Ok(s) => s,
            Err(_) => {
                println!("There was an error in reading metadata");
                State::new()
            }
        }
    } else {
        state = State::new();
    }
    let mut app = App::new();
    loop {
        ui(&mut terminal, &app, &state)?;
        if handle_events(&mut app, &mut state)? {
            // upload the new tasks
            enter_data_to_file(&state)?;
            break;
        }
    }
    Ok(())
}

/// The wrapper function which runs the complete application
pub fn run() -> Result<()> {
    startup()?;
    let result = implement_tui();
    shutdown()?;
    result?;
    Ok(())
}
