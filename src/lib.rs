use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

const TASKS_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: Uuid,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            id: Uuid::new_v4(),
            description: description.into(),
            status: Status::Todo,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub fn add_task(description: &str) {
    let task = Task::new(description);
    let task_id = &task.id.to_string();
    let mut tasks = read_task_list();
    tasks.push(task);
    write_task_list(&tasks);
    println!("Task added successfully (ID: {})", task_id);
}

fn write_task_list(tasks: &Vec<Task>) {
    let task_list_str = serde_json::to_string(tasks).expect("parsing failed");
    fs::write(TASKS_FILE, task_list_str).expect("write file faild");
}

fn is_task_list_exists() -> bool {
    fs::metadata(TASKS_FILE).is_ok()
}

fn create_task_list() {
    let empty_json = "[]";
    fs::write(TASKS_FILE, empty_json).expect("create file failed");
}

fn read_task_list() -> Vec<Task> {
    if !is_task_list_exists() {
        create_task_list();
    }

    let result_str = fs::read_to_string(TASKS_FILE).expect("read file failed");

    serde_json::from_str(&result_str).expect("parsing json to string failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_read_task_list() {
        let task_list = read_task_list();
        if !is_task_list_exists() {
            assert!(task_list.is_empty());
        }
    }

    #[test]
    fn test_add_task() {
        let tasks = read_task_list();
        add_task("testing");
        let new_tasks = read_task_list();
        assert_eq!(tasks.len() + 1, new_tasks.len())
    }
}
