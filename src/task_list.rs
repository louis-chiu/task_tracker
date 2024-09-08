use super::task::{Status, Task};
use std::fs::{self};
use std::io::{self, ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json;
const TASKS_FILE: &str = "tasks.json";

struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::<Task>::new(),
            next_id: 1,
        }
    }

    pub fn build(tasks: Vec<Task>) -> Self {
        if tasks.is_empty() {
            return Self::new();
        }
        let next_id = tasks.last().unwrap().id() + 1;
        Self {
            tasks,
            next_id
        }
    }

    pub fn add(&mut self, description: &str) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
        let _ = Self::write_task_list(&self.tasks);
    }

    pub fn update(&mut self, id: u32, description: &str) -> bool{
        if let Some(mut task) = self.get(id) {
            task.set_description(description);
            let _ = Self::write_task_list(&self.tasks);
            return true;
        }
        false
    }

    pub fn toggle_status(&mut self, status: Status, id: u32) -> bool {
        if let Some(mut task) = self.get(id) {
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

    fn index_of(&self, task: Task) -> Option<usize> {
        self.tasks.iter().position(|t| t == &task)
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

    pub fn read_task_list() -> Result<Self, io::Error> {
        match fs::read_to_string(TASKS_FILE) {
            Ok(contents) => {
                let tasks: Vec<Task> = serde_json::from_str(&contents)?;
                Ok(Self::build(tasks))
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    fs::write(TASKS_FILE, "[]")?;
                    Ok(Self::new())
                }
                _ => Err(error),
            },
        }
    }
}





