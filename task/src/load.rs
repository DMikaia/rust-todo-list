use crate::tasks::Task;
use std::{collections::HashMap, fs::File, io::Read, process};

pub fn load_task(file: &mut File) -> HashMap<usize, Task> {
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            let mut tasks: HashMap<usize, Task> = HashMap::new();

            for line in contents.lines() {
                if let Some(task) = get_task(tasks.len() + 1, line.trim().to_string()) {
                    tasks.insert(tasks.len() + 1, task);
                }
            }

            tasks
        }
        Err(e) => {
            eprintln!("Could not read the file.\n{e}");
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

        expected_result.insert(3, Task::new(3, "Drink more water".to_string()));
        expected_result.insert(2, Task::new(2, "Eat more healthy".to_string()));
        expected_result.insert(1, Task::new(1, "Buy some milk.".to_string()));

        match File::open("./mocks/tasks.txt".to_string()) {
            Ok(mut file) => assert_eq!(expected_result, load_task(&mut file)),
            Err(_) => {}
        }
    }
}
