use crate::{
    app::{App, Status},
    state::State,
    Result,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
    Frame, Terminal,
};
type Term = Terminal<CrosstermBackend<std::io::Stderr>>;

/// return the central rect for the popup
///
/// Consider first rendering the clean widget to improve popup readability
fn get_popup_rect(area: Rect) -> Rect {
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ],
    )
    .split(area);
    let central_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ],
    )
    .split(layout[1]);
    central_layout[1]
}

/// The main app layout
fn get_layout() -> Layout {
    Layout::new(
        Direction::Vertical,
        [
            // the status bar is at top
            Constraint::Length(3),
            // tasks list with atleast 1 row
            Constraint::Min(1),
            // keymaps available
            Constraint::Length(2),
        ],
    )
}

/// Determine and render the content for status section of app
fn render_status_widget(mode: &Status, f: &mut Frame, size: Rect) {
    f.render_widget(
        Paragraph::new({
            match mode {
                Status::Idle {
                    cursor: _,
                    scroll: _,
                } => "Idle Mode",
                Status::Editing {
                    previous: _,
                    edit: _,
                } => "Editing Mode",
                Status::Exiting => "Exiting",
            }
        })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Blue))
                .border_type(BorderType::Rounded),
        ),
        size,
    );
}

/// Determine and render the content for keymap section of app
fn render_keymap_widget(mode: &Status, f: &mut Frame, size: Rect) {
    f.render_widget(
        Paragraph::new({
            match mode {
                Status::Idle{
            cursor: _,
            scroll: _,
        } => "e:Edit \u{ff5c} x:Delete \u{ff5c} i:New \u{ff5c} q:Quit  \u{ff5c} Enter:Mark complete \u{ff5c} \u{2191}/\u{2193}:Select",
                Status::Editing{edit: _, previous: _} => "enter - submit task, esc - cancel",
                Status::Exiting => "",
            }
        })
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Blue))
                .border_type(BorderType::Rounded),
        ),
        size,
    );
}

/// UI for the popup to be shown when editing or adding a new task
fn render_editing_widget(f: &mut Frame, data: &str, area: Rect) {
    f.render_widget(
        Paragraph::new({
            if data.is_empty() {
                "Enter your task details"
            } else {
                data
            }
        })
        .fg({
            if data.is_empty() {
                Color::DarkGray
            } else {
                Color::White
            }
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Blue))
                .padding(Padding::horizontal(1))
                .title("Editing Task"),
        ),
        get_popup_rect(area),
    );
}

/// popup to confirm exit
fn render_exiting_widget(f: &mut Frame, area: Rect) {
    let center_popup = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(33),
            Constraint::Min(4),
            Constraint::Percentage(33),
        ],
    )
    .split(get_popup_rect(area));
    f.render_widget(
        Paragraph::new("y to quit, n to cancel")
            .fg(Color::White)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(Color::Red))
                    .title_alignment(Alignment::Center)
                    .title("Are you sure you want to quit?"),
            ),
        center_popup[1],
    );
}

/// UI when user is neither editing nor exiting a task
fn render_idle_widget(f: &mut Frame, idx: &Option<usize>, app: &App, state: &State, size: Rect) {
    let tasks = state.get_str_tasks(idx.as_ref());
    let tasks = if tasks.is_empty() {
        vec![Line::from("Tasks which you add will show up here")]
    } else {
        tasks.iter().map(|line| Line::from(line.as_str())).collect()
    };
    if let Status::Idle { cursor: _, scroll } = app.status {
        f.render_widget(
            Paragraph::new(tasks)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green))
                        .border_type(BorderType::Rounded)
                        .title("Tasks")
                        .padding(Padding::horizontal(1))
                        .fg({
                            if state.tasks.is_empty() {
                                Color::DarkGray
                            } else {
                                Color::LightBlue
                            }
                        }),
                )
                .scroll((scroll as u16, 0)),
            size,
        );
    }
}

/// Show the final ui in the terminal based on existing state
pub fn ui(terminal: &mut Term, app: &mut App, state: &State) -> Result<()> {
    terminal.draw(|f| {
        let layout = get_layout().split(f.size());
        render_status_widget(&app.status, f, layout[0]);
        match &app.status {
            Status::Editing { edit, previous: _ } => {
                f.render_widget(Clear, f.size());
                render_editing_widget(f, edit, f.size());
            }
            Status::Idle {
                cursor: idx,
                scroll: _,
            } => {
                render_idle_widget(f, idx, app, state, layout[1]);
            }
            Status::Exiting => {
                render_exiting_widget(f, layout[1]);
            }
        }
        render_keymap_widget(&app.status, f, layout[2]);
    })?;
    Ok(())
}
