use crate::tasks::Task;
use std::{collections::HashMap, fs::read_to_string, process};

pub fn load_task(file_path: String) -> HashMap<usize, Task> {
    match read_to_string(file_path) {
        Ok(tasks_str) => {
            let mut tasks: HashMap<usize, Task> = HashMap::new();

            for line in tasks_str.lines() {
                if let Some(task) = get_task(tasks.len() + 1, line.trim().to_string()) {
                    tasks.insert(tasks.len() + 1, task);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loading_task() {
        let mut expected_result: HashMap<usize, Task> = HashMap::new();

        expected_result.insert(1, Task::new(1, "Buy some milk.".to_string()));
        expected_result.insert(2, Task::new(2, "Eat more healthy".to_string()));
        expected_result.insert(3, Task::new(3, "Drink more water".to_string()));

        assert_eq!(expected_result, load_task("./mocks/tasks.txt".to_string()));
    }
}
