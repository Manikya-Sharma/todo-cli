use std::collections::HashMap;

use crate::{get_id, Id};

/// Structure of a single task
#[derive(serde::Deserialize)]
pub struct Task {
    pub id: Id,
    pub desc: String,
    pub completed: bool,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            desc: String::new(),
            completed: false,
        }
    }
}

/// The overall state of application
pub struct State {
    pub ids: Vec<Id>,
    pub tasks: HashMap<Id, Task>,
}

impl Task {
    fn new(task: &str) -> Self {
        Self {
            id: get_id(),
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
        Self {
            ids: Vec::new(),
            tasks: HashMap::new(),
        }
    }
    /// Add a new task to the given state
    pub fn add_task(&mut self, new_task: &str) {
        let new_id = get_id();
        self.ids.insert(0, new_id);
        self.tasks.insert(new_id, Task::new(new_task));
    }

    /// remove task with given id
    fn remove_task(&mut self, id: &Id) {
        self.ids = self
            .ids
            .iter()
            .filter(|old_id| *old_id != id)
            .map(|old_id| *old_id)
            .collect();
        self.tasks.remove(id);
    }

    /// delete a particular task at an index from the given state
    pub fn remove_task_by_seq(&mut self, idx: usize) {
        if idx >= self.tasks.len() {
            return;
        }
        self.remove_task(&self.ids[idx].clone());
    }

    // TODO: This method should have been used
    pub fn mark_task_complete(&mut self, id: &Id) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.mark_complete();
        }
    }
    // TODO: This method should have been used
    pub fn change_task_description(&mut self, id: &Id, new_desc: &str) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.change_desc(new_desc);
        }
    }

    pub fn get_str_tasks(&self, highlight: Option<&usize>) -> Vec<String> {
        // TODO: This method is too inefficient, use a stateful list instead
        let mut ans: Vec<String> = self
            .get_tasks()
            .iter()
            .map(|task| task.desc.clone())
            .collect();
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

    /// Return all the tasks as a vector of tasks
    pub fn get_tasks(&self) -> Vec<&Task> {
        let mut ans = Vec::new();
        for id in &self.ids {
            ans.push(self.tasks.get(id).unwrap());
        }
        ans
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
