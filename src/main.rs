use std::env::{self, Args};
use std::iter::{Skip};
use task_tracker::task::{Status, Task};
use task_tracker::task_list::TaskList;

fn main() {
    let rest_args = env::args().skip(2);
    match env::args().nth(1) {
        Some(command) => {
            handle_command(&command, rest_args);
        },
        None => {
            println!("No command provided...");
        }
    };
}

fn handle_command(command: &str, mut rest_args: Skip<Args>) {
    match command {
        "add" => {
            if let Some(description) = rest_args.nth(0) {
                println!("{description}");
                let mut tasks = TaskList::read_task_list();
                let task_id = tasks.next_id();
                tasks.add(&description);
                println!("Task added successfully (ID: {})", task_id);
            };
        },
        "update" => {
            if let Some(id) = rest_args.nth(0) {
                if let Some(description) = rest_args.nth(1) {
                    TaskList::read_task_list()
                        .update(id.parse().unwrap(), &description);
                } else {
                    panic!("No description provided...");
                }
            } else {
                panic!("No ID provided...");
            }
        },
        "delete" => {
            if let Some(id) = rest_args.nth(0)  {
                TaskList::read_task_list()
                    .delete(id.parse().unwrap());
            } else {
                println!("No ID provided...");
            }

        },
        "list" => {
            if let Some(status) = rest_args.nth(0) {
                let status = Status::from_str(&status);
                TaskList::read_task_list()
                    .list(Some(&status))
                    .into_iter()
                    .for_each(print_task);
            } else {
                TaskList::read_task_list()
                    .list(None)
                    .into_iter()
                    .for_each(|task| print_task(task));
            }
        },
        "mark-in-progress" => {
            if let Some(id) = rest_args.nth(0) {
                TaskList::read_task_list()
                    .toggle_status(Status::InProgress, id.parse().unwrap());
            }
        },
        "mark-in-done" => {
            if let Some(id) = rest_args.nth(0) {
                TaskList::read_task_list()
                    .toggle_status(Status::InProgress, id.parse().unwrap());
            }
        },
        _ => {
            println!("Invalid command...");
        }
    }
}

fn print_task(task: &Task) {
    println!("============================================================");
    println!("ID: {}", task.id());
    println!("Description: {}", task.description());
    println!("Status: {:?}", task.status());
    println!("Created At: {}", task.created_at().format("%Y/%m/%d %H:%M:%S"));
    println!("Updated At: {}", task.updated_at().format("%Y/%m/%d %H:%M:%S"));
    println!("============================================================");
}
