use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn from_str(status: &str) -> Status {
        match status {
            "todo" => Status::Todo,
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            _ => panic!("Invalid status..."),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, description: &str) -> Task {
        Task {
            id,
            description: description.into(),
            status: Status::Todo,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.into();
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use serde_json;
    use std::{thread, time::Duration};

    #[test]
    fn create_task() {
        let mut rng = rand::rng();
        let index: u32 = rng.random();
        let task = Task::new(index, "Buy some food.");
        assert_eq!(task.description(), "Buy some food.");
        assert_eq!(task.id(), index);
        assert_eq!(task.status(), &Status::Todo);
    }

    #[test]
    fn getters_work() {
        let task = Task::new(1, "Test getters");
        assert_eq!(task.id(), 1);
        assert_eq!(task.description(), "Test getters");
        assert_eq!(task.status(), &Status::Todo);
        assert!(task.created_at() <= task.updated_at());
    }

    #[test]
    fn set_status_and_updated_at() {
        let mut task = Task::new(2, "Change status");
        let old_updated = task.updated_at().clone();
        thread::sleep(Duration::from_millis(10));
        task.set_status(Status::InProgress);
        assert_eq!(task.status(), &Status::InProgress);
        assert!(task.updated_at() > &old_updated);
    }

    #[test]
    fn set_description_and_updated_at() {
        let mut task = Task::new(3, "Old desc");
        let old_updated = task.updated_at().clone();
        thread::sleep(Duration::from_millis(10));
        task.set_description("New desc");
        assert_eq!(task.description(), "New desc");
        assert!(task.updated_at() > &old_updated);
    }

    #[test]
    fn status_from_str() {
        assert_eq!(Status::from_str("todo"), Status::Todo);
        assert_eq!(Status::from_str("in-progress"), Status::InProgress);
        assert_eq!(Status::from_str("done"), Status::Done);
    }

    #[test]
    #[should_panic]
    fn status_from_str_invalid() {
        Status::from_str("invalid");
    }

    #[test]
    fn clone_and_eq() {
        let task1 = Task::new(4, "Clone me");
        let task2 = task1.clone();
        assert_eq!(task1, task2);
    }

    #[test]
    fn serde_serialize_deserialize() {
        let task = Task::new(5, "Serde test");
        let json = serde_json::to_string(&task).unwrap();
        let task2: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(task, task2);
    }
}
