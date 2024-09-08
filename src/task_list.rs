use super::task::{Status, Task};
use std::fs::{self};
use std::io::{self, ErrorKind};
use serde_json;
const TASKS_FILE: &str = "tasks.json";

pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    fn new() -> Self {
        Self {
            tasks: Vec::<Task>::new(),
            next_id: 1,
        }
    }

    fn build(tasks: Vec<Task>) -> Self {
        if tasks.is_empty() {
            return Self::new();
        }
        let next_id = tasks.last().unwrap().id() + 1;
        Self {
            tasks,
            next_id
        }
    }

    pub fn next_id(&self) -> u32 {
        self.next_id
    }

    pub fn add(&mut self, description: &str) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
        let _ = Self::write_task_list(&self.tasks);
    }

    pub fn update(&mut self, id: u32, description: &str) -> bool{
        if let Some(task) = self.get(id) {
            task.set_description(description);
            let _ = Self::write_task_list(&self.tasks);
            return true;
        }
        false
    }

    pub fn toggle_status(&mut self, status: Status, id: u32) -> bool {
        if let Some(task) = self.get(id) {
            task.set_status(status);
            let _ = Self::write_task_list(&self.tasks);
            return true;
        }
        false
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(index) = self.position(id) {
            self.tasks.remove(index);
            let _ = Self::write_task_list(&self.tasks);
            return true;
        }
        false
    }

    pub fn get(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id() == id)
    }

    fn position(&self, id: u32) -> Option<usize> {
        self.tasks.iter().position(|task| task.id() == id)
    }


    pub fn list(&self, option: Option<&Status>) -> Vec<&Task> {
        match option {
            Some(status) => self.tasks.iter()
                .filter(|task| task.status() == status)
                .collect(),
            None => self.tasks.iter().collect(),
        }
    }


    fn write_task_list(tasks: &Vec<Task>) -> Result<(), io::Error> {
        let content = serde_json::to_string(&tasks)?;
        fs::write(TASKS_FILE, content)?;
        Ok(())
    }

    pub fn read_task_list() -> TaskList {
        match fs::read_to_string(TASKS_FILE) {
            Ok(contents) => {
                if let Ok(tasks) = serde_json::from_str(&contents) {
                    Self::build(tasks)
                } else {
                    panic!("Error parsing tasks file...");
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let _ = fs::write(TASKS_FILE, "[]");
                    Self::new()
                }
                _ => panic!("Error reading tasks file: {:?}", error),
            },
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_read_task_list() {
        let _ = fs::remove_file(TASKS_FILE);
        let tasks = TaskList::read_task_list().tasks;
        assert_eq!(tasks, Vec::<Task>::new());
    }

    #[test]
    fn test_add_task() {
        let mut task_list = TaskList::read_task_list();
        task_list.add("testing");
        let new_tasks = TaskList::read_task_list();
        assert_eq!(task_list.tasks.len(), new_tasks.tasks.len())
    }

    #[test]
    fn test_delete_task() {
        let mut task_list = TaskList::read_task_list();
        task_list.delete(1);
        assert_eq!(task_list.tasks.len(), 0);
    }

    #[test]
    fn test_update_task() {
        let mut task_list = TaskList::read_task_list();
        task_list.add("testing");
        task_list.update(1, "updated");
        let task = task_list.tasks.iter().find(|task| task.id() == 1).unwrap();
        assert_eq!(task.description(), "updated");
    }

    #[test]
    fn test_list_tasks() {
        let mut task_list = TaskList::read_task_list();
        task_list.add("testing");
        task_list.add("testing");
        task_list.add("testing");
        let tasks = task_list.list(None);
        assert_eq!(tasks.len(), 3);
    }

}