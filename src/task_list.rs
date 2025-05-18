use super::task::{Status, Task};
use serde_json;
use std::fs::{self};
use std::io::{self};

pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
    path: String,
}

impl TaskList {
    fn new(path: String) -> Self {
        Self {
            tasks: Vec::<Task>::new(),
            next_id: 1,
            path,
        }
    }

    fn build(tasks: Vec<Task>, path: String) -> Self {
        if tasks.is_empty() {
            return Self::new(path);
        }
        let next_id = tasks.last().unwrap().id() + 1;
        Self {
            tasks,
            next_id,
            path,
        }
    }

    pub fn next_id(&self) -> u32 {
        self.next_id
    }

    pub fn add(&mut self, description: &str) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
        self.write_into().unwrap();
    }

    pub fn update(&mut self, id: u32, description: &str) -> bool {
        if let Some(task) = self.get(id) {
            task.set_description(description);
            self.write_into().unwrap();
            return true;
        }
        false
    }

    pub fn toggle_status(&mut self, status: Status, id: u32) -> bool {
        if let Some(task) = self.get(id) {
            task.set_status(status);
            self.write_into().unwrap();
            return true;
        }
        false
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(index) = self.position(id) {
            self.tasks.remove(index);
            self.write_into().unwrap();
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
            Some(status) => self
                .tasks
                .iter()
                .filter(|task| task.status() == status)
                .collect(),
            None => self.tasks.iter().collect(),
        }
    }

    fn write_into(&self) -> Result<(), io::Error> {
        let content = serde_json::to_string(&self.tasks)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn create_empty_task_list(tasks_file_path: &str) -> Result<Self, io::Error> {
        fs::write(tasks_file_path, "[]")?;
        Ok(Self::new(tasks_file_path.to_string()))
    }

    pub fn read_task_list(tasks_file_path: &str) -> Result<TaskList, io::Error> {
        let contents = fs::read_to_string(tasks_file_path)?;

        let tasks = serde_json::from_str(&contents)?;
        Ok(Self::build(tasks, tasks_file_path.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_TASKS_FILE: &str = ".tasks_test.json";

    fn setup_empty() -> TaskList {
        let _ = fs::remove_file(TEST_TASKS_FILE);
        let task_list = TaskList::create_empty_task_list(TEST_TASKS_FILE).unwrap();
        task_list
    }

    fn setup_with_tasks() -> TaskList {
        let _ = fs::remove_file(TEST_TASKS_FILE);
        let tasks = vec![
            Task::new(1, "Leetcode"),
            Task::new(2, "Backend"),
            Task::new(3, "Frontend"),
        ];
        let task_list = TaskList::build(tasks, TEST_TASKS_FILE.to_string());
        task_list.write_into().unwrap();
        task_list
    }

    fn teardown() {
        let _ = fs::remove_file(TEST_TASKS_FILE);
    }

    #[test]
    fn create_new_task_list() {
        teardown();
        let new_file_path = TEST_TASKS_FILE;
        let task_list = TaskList::create_empty_task_list(new_file_path).unwrap();

        assert!(fs::metadata(new_file_path).is_ok());

        assert_eq!(task_list.tasks.len(), 0);
        assert_eq!(task_list.next_id, 1);
        assert_eq!(task_list.path, new_file_path);

        teardown();
    }

    #[test]
    fn test_build_empty_tasks() {
        let tasks = Vec::<Task>::new();
        let task_list = TaskList::build(tasks, "path".to_string());

        assert_eq!(task_list.next_id, 1);
        assert_eq!(task_list.tasks.len(), 0);
    }

    #[test]
    fn test_build_with_tasks() {
        let tasks = vec![
            Task::new(1, "Task 1"),
            Task::new(2, "Task 2"),
            Task::new(3, "Task 3"),
        ];

        let task_list = TaskList::build(tasks, "path".to_string());

        assert_eq!(task_list.next_id, 4);
        assert_eq!(task_list.tasks.len(), 3);
    }

    #[test]
    fn test_read_write_task_list() {
        let tasks = vec![
            Task::new(1, "Leetcode"),
            Task::new(2, "Backend"),
            Task::new(3, "Frontend"),
        ];
        let original = TaskList::build(tasks, TEST_TASKS_FILE.to_string());
        original.write_into().unwrap();

        let from_file = TaskList::read_task_list(TEST_TASKS_FILE).unwrap();
        assert_eq!(original.tasks.len(), from_file.tasks.len());
        assert_eq!(original.next_id, from_file.next_id);

        for i in 0..original.tasks.len() {
            assert_eq!(original.tasks[i].id(), from_file.tasks[i].id());
            assert_eq!(
                original.tasks[i].description(),
                from_file.tasks[i].description()
            );
            assert_eq!(original.tasks[i].status(), from_file.tasks[i].status());
        }

        teardown();
    }

    #[test]
    fn test_first_read_task_list_empty_file() {
        TaskList::create_empty_task_list(TEST_TASKS_FILE).unwrap();

        let task_list = TaskList::read_task_list(TEST_TASKS_FILE).unwrap();
        assert_eq!(task_list.tasks.len(), 0);
        assert_eq!(task_list.next_id, 1);

        teardown();
    }

    #[test]
    fn test_add_task() {
        let mut task_list = setup_empty();

        task_list.add("First task");
        assert_eq!(task_list.tasks.len(), 1);
        assert_eq!(task_list.next_id, 2);
        assert_eq!(task_list.tasks[0].description(), "First task");
        assert_eq!(task_list.tasks[0].id(), 1);

        task_list.add("Second task");
        assert_eq!(task_list.tasks.len(), 2);
        assert_eq!(task_list.next_id, 3);

        let from_file = TaskList::read_task_list(TEST_TASKS_FILE).unwrap();
        assert_eq!(from_file.tasks.len(), 2);

        teardown();
    }

    #[test]
    fn test_get_task() {
        let mut task_list = setup_with_tasks();

        let task = task_list.get(2);
        assert!(task.is_some());
        assert_eq!(task.unwrap().description(), "Backend");

        let task = task_list.get(99);
        assert!(task.is_none());

        teardown();
    }

    #[test]
    fn test_update_task_success() {
        let mut task_list = setup_with_tasks();

        let result = task_list.update(2, "Updated Backend");
        assert!(result);

        let task = task_list.get(2).unwrap();
        assert_eq!(task.description(), "Updated Backend");

        let from_file = TaskList::read_task_list(TEST_TASKS_FILE).unwrap();
        let task = from_file.tasks.iter().find(|t| t.id() == 2).unwrap();
        assert_eq!(task.description(), "Updated Backend");

        teardown();
    }

    #[test]
    fn test_update_task_failure() {
        let mut task_list = setup_with_tasks();

        let result = task_list.update(99, "Does not exist");
        assert!(!result);

        teardown();
    }

    #[test]
    fn test_toggle_status() {
        let mut task_list = setup_with_tasks();

        let result = task_list.toggle_status(Status::InProgress, 2);
        assert!(result);

        let task = task_list.get(2).unwrap();
        assert_eq!(task.status(), &Status::InProgress);

        let result = task_list.toggle_status(Status::Done, 2);
        assert!(result);

        let task = task_list.get(2).unwrap();
        assert_eq!(task.status(), &Status::Done);

        let result = task_list.toggle_status(Status::Done, 99);
        assert!(!result);

        teardown();
    }

    #[test]
    fn test_delete_task_success() {
        let mut task_list = setup_with_tasks();
        assert_eq!(task_list.tasks.len(), 3);

        let result = task_list.delete(2);
        assert!(result);
        assert_eq!(task_list.tasks.len(), 2);

        let from_file = TaskList::read_task_list(TEST_TASKS_FILE).unwrap();
        assert_eq!(from_file.tasks.len(), 2);
        assert!(from_file.tasks.iter().find(|t| t.id() == 2).is_none());

        teardown();
    }

    #[test]
    fn test_delete_task_failure() {
        let mut task_list = setup_with_tasks();
        assert_eq!(task_list.tasks.len(), 3);

        let result = task_list.delete(99);
        assert!(!result);
        assert_eq!(task_list.tasks.len(), 3);

        teardown();
    }

    #[test]
    fn test_list_all_tasks() {
        let task_list = setup_with_tasks();

        let tasks = task_list.list(None);
        assert_eq!(tasks.len(), 3);

        teardown();
    }

    #[test]
    fn test_list_filtered_by_status() {
        let mut task_list = setup_with_tasks();

        let todo_tasks = task_list.list(Some(&Status::Todo));
        assert_eq!(todo_tasks.len(), 3);

        task_list.toggle_status(Status::InProgress, 2);

        let todo_tasks = task_list.list(Some(&Status::Todo));
        let in_progress_tasks = task_list.list(Some(&Status::InProgress));
        let done_tasks = task_list.list(Some(&Status::Done));

        assert_eq!(todo_tasks.len(), 2);
        assert_eq!(in_progress_tasks.len(), 1);
        assert_eq!(done_tasks.len(), 0);

        teardown();
    }

    #[test]
    fn test_position() {
        let task_list = setup_with_tasks();

        let pos = task_list.position(2);
        assert_eq!(pos, Some(1));
        let pos = task_list.position(99);
        assert_eq!(pos, None);

        teardown();
    }

    #[test]
    fn test_next_id() {
        let task_list = setup_with_tasks();

        assert_eq!(task_list.next_id(), 4);

        teardown();
    }
}
