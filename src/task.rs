// src/task.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
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

