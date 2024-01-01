use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    app::{App, Status},
    state::State,
    Result,
};

/// Managing all the events occuring in idle state of the app
fn manage_idle_events(app: &mut App, state: &mut State, key: KeyCode) {
    match key {
        KeyCode::Char('q') => {
            app.switch_status(Status::Exiting);
        }
        KeyCode::Char('e') | KeyCode::Char('i') => {
            app.switch_status(Status::Editing(String::new()))
        }
        KeyCode::Char('d') | KeyCode::Char('x') => {
            let idx = app.get_idle_idx();
            state.remove_task(idx);
        }
        KeyCode::Down | KeyCode::Char('j') => app.idle_down(),
        KeyCode::Up | KeyCode::Char('k') => app.idle_up(),
        KeyCode::Enter => {
            let idx = app.get_idle_idx();
            if idx >= state.get_len() {
                return;
            }
            app.switch_status(Status::Editing(state.get_task_text(idx).to_string()));
            state.remove_task(idx);
        }
        _ => {}
    }
}

/// Managing all the events in editing state of the app
fn manage_edit_events(app: &mut App, state: &mut State, key: KeyCode) {
    match key {
        // TODO: Esc will remove the existing task when editing
        KeyCode::Esc => app.switch_status(Status::Idle(None)),
        KeyCode::Enter => {
            let task = app.get_buffer_task();
            if task.trim().is_empty() {
                return;
            }
            state.add_task(&task);
            app.switch_status(Status::Idle(Some(0)))
        }
        KeyCode::Char(ch) => {
            app.add_char(ch);
        }
        KeyCode::Backspace => {
            app.pop_char();
        }
        _ => {}
    }
}

/// Managing all the events in exiting state of the app
fn manage_exiting_events(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Esc | KeyCode::Char('n') => app.switch_status(Status::Idle(None)),
        KeyCode::Char('y') | KeyCode::Char('q') => {
            return true;
        }
        _ => {}
    }
    false
}

/// Local hepler function made for improving modularity of main function
fn helper(app: &mut App, state: &mut State, key: KeyCode) -> bool {
    match app.status {
        Status::Idle(_) => {
            manage_idle_events(app, state, key);
        }
        Status::Editing(_) => {
            manage_edit_events(app, state, key);
        }
        Status::Exiting => {
            return manage_exiting_events(app, key);
        }
    }

    false
}

/// Handles all the inputs fromt the user
///
/// ## Return Type
/// Returns a `Result` type with a `bool` to tell
/// when the loop must break
pub fn handle_events(app: &mut App, state: &mut State) -> Result<bool> {
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(k) = crossterm::event::read()? {
            // press for single keypress in windows
            if k.kind == KeyEventKind::Press {
                return Ok(helper(app, state, k.code));
            }
        }
    }
    Ok(false)
}
