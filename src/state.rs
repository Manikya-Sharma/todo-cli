use std::collections::HashMap;

use crate::{get_id, Id};

/// Structure of a single task
#[derive(Default, serde::Deserialize)]
pub struct Task {
    pub id: Id,
    pub desc: String,
    pub completed: bool,
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
    fn mark_incomplete(&mut self) {
        self.completed = false;
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
    pub fn remove_task(&mut self, id: &Id) -> Option<()> {
        if self.tasks.remove(id).is_some() {
            self.ids.retain(|old_id| old_id != id);
            Some(())
        } else {
            None
        }
    }

    /// delete a particular task at an index from the given state
    pub fn remove_task_by_seq(&mut self, idx: usize) {
        if idx >= self.tasks.len() {
            return;
        }
        self.remove_task(&self.ids[idx].clone());
    }

    /// mark incomplete task complete and vice versa
    ///
    /// returns true if task marked as complete else false
    pub fn toggle_task_status(&mut self, idx: usize) -> Option<bool> {
        let id = self.ids[idx];
        self.toggle_task_status_by_id(id)
    }

    /// mark incomplete task complete and vice versa
    ///
    /// returns true if task marked as complete else false
    pub fn toggle_task_status_by_id(&mut self, id: Id) -> Option<bool> {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.completed {
                task.mark_incomplete();
                Some(false)
            } else {
                task.mark_complete();
                Some(true)
            }
        } else {
            None
        }
    }

    /// get all the tasks as a string
    pub fn get_str_tasks(&self, highlight: Option<&usize>) -> Vec<String> {
        // TODO: This method is too inefficient, use a stateful list instead maybe
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
