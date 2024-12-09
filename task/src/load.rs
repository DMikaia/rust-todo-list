use crate::tasks::Task;
use std::{fs::read_to_string, process};

pub fn load_task(file_path: String) -> Vec<Task> {
    match read_to_string(file_path) {
        Ok(tasks_str) => {
            let mut tasks = Vec::new();

            for line in tasks_str.lines() {
                if let Some(task) = get_task(tasks.len() + 1, line.trim().to_string()) {
                    tasks.push(task);
                }
            }

            tasks
        }
        Err(e) => {
            eprintln!("File not found.\n{e}");
            process::exit(1);
        }
    }
}

fn get_task(id: usize, task_str: String) -> Option<Task> {
    match task_str.len() {
        0 => None,
        _ => Some(Task::new(id, task_str)),
    }
}
