use std::collections::HashMap;

use tui_widget_list::Listable;

use crate::{get_id, ui::render_list_item, Id};

/// Structure of a single task
#[derive(Default, serde::Deserialize)]
pub struct Task {
    pub id: Id,
    pub desc: String,
    pub completed: bool,
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

/// wrapper for a task as a list item
pub struct ListItem {
    pub task: Task,
    pub selected: bool,
}

impl Listable for &ListItem {
    fn height(&self) -> usize {
        1
    }
    fn highlight(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}

impl ratatui::widgets::Widget for &ListItem {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        render_list_item(self, area, buf);
    }
}

impl ListItem {
    pub fn from(task: &Task) -> Self {
        Self {
            selected: false,
            task: Task {
                completed: task.completed,
                desc: task.desc.clone(),
                id: task.id,
            },
        }
    }
    fn set_selected(&mut self) {
        self.selected = true;
    }
    fn set_unselected(&mut self) {
        self.selected = false;
    }
}

/// The overall state of application
pub struct State {
    pub ids: Vec<Id>,
    pub tasks: HashMap<Id, ListItem>,
    /// index of selected task
    pub selected: Option<usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            tasks: HashMap::new(),
            selected: None,
        }
    }

    /// Move app state selection
    pub fn move_selection(&mut self, upwards: bool) {
        if let Some(selected) = &self.selected {
            let next = {
                if upwards {
                    selected.saturating_sub(1)
                } else {
                    *selected + 1
                }
            }
            .clamp(0, self.ids.len() - 1);
            self.tasks
                .get_mut(&self.ids[*selected])
                .unwrap()
                .set_unselected();
            self.tasks.get_mut(&self.ids[next]).unwrap().set_selected();
            self.selected = Some(next);
        } else {
            self.tasks.get_mut(&self.ids[0]).unwrap().set_selected();
            self.selected = Some(0);
        }
    }

    /// Add a new task to the given state
    pub fn add_task(&mut self, new_task: &str) {
        let new_id = get_id();
        self.ids.insert(0, new_id);
        self.tasks
            .insert(new_id, ListItem::from(&Task::new(new_task)));
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
        if let Some(list_item) = self.tasks.get_mut(&id) {
            if list_item.task.completed {
                list_item.task.mark_incomplete();
                Some(false)
            } else {
                list_item.task.mark_complete();
                Some(true)
            }
        } else {
            None
        }
    }

    /// Return all the tasks as a vector of tasks
    pub fn get_tasks(&self) -> Vec<&Task> {
        let mut ans = Vec::new();
        for id in &self.ids {
            ans.push(&self.tasks.get(id).unwrap().task);
        }
        ans
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_selection_change() {
        let mut state = State::new();
        state.add_task("abc");
        state.add_task("123");
        state.add_task("xyz");
        assert!(state.selected.is_none());
        state.move_selection(true);
        assert!(state.selected.is_some());
        assert_eq!(state.selected.unwrap(), 0);
        state.move_selection(false);
        assert_eq!(state.selected.unwrap(), 1);
        assert!(state.tasks.get(&state.ids[1]).unwrap().selected);
    }
}
