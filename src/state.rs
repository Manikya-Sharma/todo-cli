/// Structure of a single task
struct Task {
    desc: String,
    completed: bool,
}

/// The overall state of application
pub struct State {
    tasks: Vec<Task>,
}

impl Task {
    fn new(task: &str) -> Self {
        Self {
            desc: task.to_owned(),
            completed: false,
        }
    }
    fn mark_complete(&mut self) {
        self.completed = true;
    }
    fn change_desc(&mut self, new_desc: &str) {
        self.desc = new_desc.to_owned();
    }
}

impl State {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    /// Gives the total number of tasks
    pub fn get_len(&self) -> usize {
        self.tasks.len()
    }
    /// Add a new task to the given state
    pub fn add_task(&mut self, new_task: &str) {
        self.tasks.insert(0, Task::new(new_task))
    }
    /// ## Panics
    /// Panics when idx out of bounds
    pub fn get_task_text(&self, idx: usize) -> &str {
        &self.tasks.get(idx).unwrap().desc
    }
    /// delete a particular task at an index from the given state
    pub fn remove_task(&mut self, idx: usize) {
        if idx >= self.tasks.len() {
            return;
        }
        self.tasks.remove(idx);
    }
    // TODO: This method should have been used
    pub fn mark_task_complete(&mut self, idx: usize) {
        if let Some(task) = self.tasks.get_mut(idx) {
            task.mark_complete();
        }
    }
    // TODO: This method should have been used
    pub fn change_task_description(&mut self, idx: usize, new_desc: &str) {
        if let Some(task) = self.tasks.get_mut(idx) {
            task.change_desc(new_desc);
        }
    }
    /// Return all the tasks as a vector of stringg
    pub fn get_str_tasks(&self, highlight: &Option<usize>) -> Vec<String> {
        // TODO: This method is too inefficient, use a stateful list instead
        let mut ans = Vec::new();
        for task in &self.tasks {
            ans.push(task.desc.to_string());
        }
        if ans.is_empty() {
            return Vec::new();
        }
        if let Some(idx) = highlight {
            if *idx >= ans.len() {
                let len = ans.len();
                ans[len - 1].insert_str(0, "->    ");
            } else {
                for (i, elem) in ans.iter_mut().enumerate() {
                    if i == *idx {
                        elem.insert_str(0, "->    ");
                    }
                }
            }
        }
        ans
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
