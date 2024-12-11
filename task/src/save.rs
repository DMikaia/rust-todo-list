use crate::tasks::Task;
use std::{collections::HashMap, fs::File, io::Write};

pub fn save_task(file: &mut File, tasks: HashMap<usize, Task>) -> Result<(), String> {
    for (id, (_, task)) in tasks.iter().enumerate() {
        match file.write_fmt(format_args!(
            "{}{}",
            task.get_description(),
            if id + 1 < tasks.len() { "\n" } else { "" }
        )) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                return Err("Could not write into the file.".to_string());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use super::*;

    #[test]
    fn saved_tasks() {
        let mut result: bool = true;

        let mut tasks: HashMap<usize, Task> = HashMap::new();
        tasks.insert(3, Task::new(3, "Drink more water".to_string()));
        tasks.insert(2, Task::new(2, "Eat more healthy".to_string()));
        tasks.insert(1, Task::new(1, "Buy some milk.".to_string()));

        let mut save_file = OpenOptions::new()
            .write(true)
            .open("./mocks/save.txt".to_string())
            .unwrap();

        match save_task(&mut save_file, tasks) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{e}");
                result = false;
            }
        }

        assert_eq!(true, result)
    }
}
