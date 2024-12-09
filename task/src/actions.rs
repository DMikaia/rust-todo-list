use crate::tasks::Task;
use std::collections::HashMap;

pub fn display_task(tasks: &HashMap<usize, Task>) {
    if tasks.len() == 0 {
        println!("Task list is currently empty.\n");
    }

    println!("Tasks:");
    for (_, task) in tasks.iter() {
        println!("{task}");
    }
}

pub fn add_task(tasks: &mut HashMap<usize, Task>, description: String) {
    if description.len() == 0 {
        eprintln!("A description most be provided\n");
        return;
    }

    tasks.insert(tasks.len() + 1, Task::new(tasks.len() + 1, description));
}

pub fn remove_task(tasks: &mut HashMap<usize, Task>, id: usize) -> Result<Task, String> {
    if tasks.len() == 0 {
        eprintln!();
        return Err("The current task list is empty.".to_string());
    }

    if id == 0 || id > tasks.len() {
        eprintln!();
        return Err(format!("The id most be in range of 1 to {}\n", tasks.len()));
    }

    if let Some(task) = tasks.remove(&id) {
        Ok(task)
    } else {
        Err("Task not found.".to_string())
    }
}
