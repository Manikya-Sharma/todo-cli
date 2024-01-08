pub struct App {
    pub status: Status,
}

/// The current status of application
pub enum Status {
    /// Editing state
    ///
    /// edit stores the buffer value for the new task
    ///
    /// previous tells if we are editing a task or creating new one
    Editing {
        edit: String,
        previous: Option<usize>,
    },
    /// Idle state
    Idle,
    /// Exiting state
    ///
    /// This is to avoid abrupt closure and ask before exit
    Exiting,
}

impl App {
    /// Generate new app instance with default values
    pub fn new() -> Self {
        Self {
            status: Status::Idle,
        }
    }
    /// Change status of the app
    pub fn switch_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    /// Add a character to the current task being added/changed
    pub fn add_char(&mut self, ch: char) {
        if let Status::Editing { edit, previous: _ } = &mut self.status {
            edit.push(ch);
        }
    }

    /// Remove a charecter fromt he current task being added/changed
    pub fn pop_char(&mut self) {
        if let Status::Editing { edit, previous: _ } = &mut self.status {
            edit.pop();
        }
    }

    /// Get access to the task which is being written by user while being added
    pub fn get_editing_task(&self) -> String {
        if let Status::Editing { edit, previous: _ } = &self.status {
            edit.to_string()
        } else {
            String::new()
        }
    }

    // TODO is it needed?
    pub fn get_prev_task(&self) -> Option<usize> {
        if let Status::Editing { edit: _, previous } = &self.status {
            *previous
        } else {
            None
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
