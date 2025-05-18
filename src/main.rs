use std::env;
use std::io;
use std::io::ErrorKind;
use task_tracker::task::{Status, Task};
use task_tracker::task_list::TaskList;

const TASK_LIST_PATH: &str = "tasks.json";

fn main() -> Result<(), io::Error> {
    initial_task_list()?;
    let mut args = env::args().skip(1);

    match args.next() {
        Some(command) => execute_command(&command, &mut args),
        None => {
            println!("No command provided...");
            Ok(())
        }
    }
}

fn initial_task_list() -> Result<(), io::Error> {
    if let Err(error) = TaskList::read_task_list(TASK_LIST_PATH) {
        if error.kind() == ErrorKind::NotFound {
            TaskList::create_empty_task_list(TASK_LIST_PATH)?;
        }
    }
    Ok(())
}

fn execute_command(
    command: &str,
    mut rest_args: impl Iterator<Item = String>,
) -> Result<(), io::Error> {
    match command {
        "add" => {
            rest_args
                .next()
                .map(|description| {
                    println!("{description}");
                    TaskList::read_task_list(TASK_LIST_PATH).and_then(|mut tasks| {
                        let task_id = tasks.next_id();
                        tasks.add(&description);
                        println!("Task added successfully (ID: {})", task_id);
                        Ok(())
                    })
                })
                .unwrap_or_else(|| {
                    println!("No description provided...");
                    Ok(())
                })?;
        }
        "update" => {
            rest_args
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No ID provided..."))
                .and_then(|id| {
                    rest_args
                        .next()
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "No description provided...",
                            )
                        })
                        .and_then(|description| {
                            let mut tasks = TaskList::read_task_list(TASK_LIST_PATH)?;
                            tasks.update(id.parse().unwrap(), &description);
                            Ok(())
                        })
                })?;
        }
        "delete" => {
            rest_args
                .next()
                .map(|id| {
                    TaskList::read_task_list(TASK_LIST_PATH).and_then(|mut task_list| {
                        task_list.delete(id.parse().unwrap());
                        Ok(())
                    })
                })
                .unwrap_or_else(|| {
                    println!("No ID provided...");
                    Ok(())
                })?;
        }
        "list" => {
            let status = rest_args.next().map(|status| Status::from_str(&status));
            TaskList::read_task_list(TASK_LIST_PATH)?
                .list(status.as_ref())
                .into_iter()
                .for_each(print_task);
        }
        "mark-in-progress" => {
            rest_args
                .next()
                .map(|id| {
                    TaskList::read_task_list(TASK_LIST_PATH).and_then(|mut task_list| {
                        task_list.toggle_status(Status::InProgress, id.parse().unwrap());
                        Ok(())
                    })
                })
                .unwrap_or_else(|| {
                    println!("No ID provided...");
                    Ok(())
                })?;
        }
        "mark-done" => {
            rest_args
                .next()
                .map(|id| {
                    TaskList::read_task_list(TASK_LIST_PATH).and_then(|mut task_list| {
                        task_list.toggle_status(Status::Done, id.parse().unwrap());
                        Ok(())
                    })
                })
                .unwrap_or_else(|| {
                    println!("No ID provided...");
                    Ok(())
                })?;
        }
        _ => {
            println!("Invalid command...");
        }
    };
    Ok(())
}

fn print_task(task: &Task) {
    println!("============================================================");
    println!("ID: {}", task.id());
    println!("Description: {}", task.description());
    println!("Status: {}", task.status());
    println!(
        "Created At: {}",
        task.created_at().format("%Y/%m/%d %H:%M:%S")
    );
    println!(
        "Updated At: {}",
        task.updated_at().format("%Y/%m/%d %H:%M:%S")
    );
    println!("============================================================");
}
