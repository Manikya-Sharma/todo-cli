use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    app::{App, Status},
    state::State,
    Result,
};

/// Managing all the events occuring in idle state of the app
fn manage_idle_events(app: &mut App, state: &mut State, key: KeyCode) {
    match key {
        // quit
        KeyCode::Char('q') => {
            app.switch_status(Status::Exiting);
        }
        // new task
        KeyCode::Char('i') => app.switch_status(Status::Editing {
            edit: String::new(),
            previous: None,
        }),
        // delete task
        KeyCode::Char('d') | KeyCode::Char('x') => {
            let idx = app.get_idle_idx();
            if let Some(idx) = idx {
                state.remove_task_by_seq(idx);
            }
        }
        // move down
        KeyCode::Down | KeyCode::Char('j') => app.idle_down(state.ids.len() - 1),
        // moev up
        KeyCode::Up | KeyCode::Char('k') => app.idle_up(),
        // edit the task
        KeyCode::Char('e') => {
            let idx = app.get_idle_idx();
            if let Some(idx) = idx {
                if idx >= state.ids.len() {
                    return;
                }
                app.switch_status(Status::Editing {
                    edit: state.tasks.get(&state.ids[idx]).unwrap().desc.clone(),
                    previous: Some(idx),
                });
            }
        }
        // mark task complete
        KeyCode::Enter => {
            let idx = app.get_idle_idx();
            if let Some(idx) = idx {
                if idx > state.ids.len() {
                    return;
                }
                state.toggle_task_status(idx);
            }
        }
        _ => {}
    }
}

/// Managing all the events in editing state of the app
fn manage_edit_events(app: &mut App, state: &mut State, key: KeyCode) {
    match key {
        KeyCode::Esc => app.switch_status(Status::Idle {
            cursor: None,
            scroll: 0,
        }),
        KeyCode::Enter => {
            let task = app.get_editing_task();
            if task.trim().is_empty() {
                return;
            }
            let prev = app.get_prev_task();
            if let Some(idx) = prev {
                // editing already existing task
                state.remove_task_by_seq(idx)
            }
            state.add_task(&task);
            app.switch_status(Status::Idle {
                cursor: Some(0),
                scroll: 0,
            })
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
        KeyCode::Esc | KeyCode::Char('n') => app.switch_status(Status::Idle {
            cursor: None,
            scroll: 0,
        }),
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
        Status::Idle {
            cursor: _,
            scroll: _,
        } => {
            manage_idle_events(app, state, key);
        }
        Status::Editing {
            edit: _,
            previous: _,
        } => {
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
