# Task Tracker

This is a project for managing tasks, developed using the Rust programming language. The project includes the following features:

- Add tasks
- Update tasks
- Delete tasks
- Mark tasks as in progress, done
- List all tasks and Filter tasks by status

## Usage

### Add a Task

```sh
cargo run -- add "task description"
```
### Delete a Task

```sh
cargo run -- delete 1
```

### Update a Task

```sh
cargo run -- update 1 "new task description"
```

### Mark a Task as In Progress

```sh 
cargo run -- mark-in-progress 1
```

### Mark a Task as Done

```sh
cargo run -- mark-done 1
```

### List Tasks

```sh
cargo run -- list
cargo run -- list --status todo
cargo run -- list --status in-progress
cargo run -- list --status done
```
