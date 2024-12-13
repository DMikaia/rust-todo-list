use crate::tasks::Task;
use std::collections::HashMap;

fn parse_and_sort_tasks(tasks: &HashMap<usize, Task>) -> Vec<&Task> {
    let mut tasks_vec: Vec<&Task> = tasks.iter().map(|t| t.1).collect();
    tasks_vec.sort_by(|a, b| a.get_id().partial_cmp(&b.get_id()).unwrap());

    tasks_vec
}

pub fn display_task(tasks: &HashMap<usize, Task>) {
    if tasks.len() == 0 {
        println!("Task list is currently empty.\n");
    }

    let tasks_vec: Vec<&Task> = parse_and_sort_tasks(tasks);

    println!("Tasks:");
    for &task in tasks_vec.iter() {
        println!("{}", task);
    }
}

pub fn add_task(tasks: &mut HashMap<usize, Task>, description: String) -> Result<(), String> {
    if description.len() == 0 {
        return Err("A description most be provided".to_string());
    }

    tasks.insert(tasks.len() + 1, Task::new(tasks.len() + 1, description));

    Ok(())
}

pub fn remove_task(tasks: &mut HashMap<usize, Task>, id: usize) -> Result<Task, String> {
    if tasks.len() == 0 {
        return Err("The current task list is empty.".to_string());
    }

    if let Some(task) = tasks.remove(&id) {
        Ok(task)
    } else {
        Err("Task not found.".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adding_task_success() {
        let mut tasks: HashMap<usize, Task> = HashMap::new();

        let mut expected_result: HashMap<usize, Task> = HashMap::new();
        expected_result.insert(1, Task::new(1, "Test".to_string()));

        match add_task(&mut tasks, "Test".to_string()) {
            Ok(()) => assert_eq!(expected_result, tasks),
            Err(_) => {}
        }
    }

    #[test]
    fn adding_task_with_an_empty_description() {
        let mut tasks: HashMap<usize, Task> = HashMap::new();

        match add_task(&mut tasks, "".to_string()) {
            Ok(()) => {}
            Err(e) => assert_eq!("A description most be provided".to_string(), e),
        }
    }

    #[test]
    fn removing_task_success() {
        let mut tasks: HashMap<usize, Task> = HashMap::new();
        tasks.insert(1, Task::new(1, "Test number 1".to_string()));
        tasks.insert(2, Task::new(2, "Test number 2".to_string()));

        if let Ok(removed_task) = remove_task(&mut tasks, 2) {
            assert_eq!(Task::new(2, "Test number 2".to_string()), removed_task);
        }
    }

    #[test]
    fn removing_task_with_an_empty_list() {
        let mut tasks: HashMap<usize, Task> = HashMap::new();

        match remove_task(&mut tasks, 1) {
            Ok(_) => {}
            Err(e) => assert_eq!("The current task list is empty.".to_string(), e),
        }
    }

    #[test]
    fn removing_task_not_found() {
        let mut tasks: HashMap<usize, Task> = HashMap::new();
        tasks.insert(1, Task::new(1, "Test number 1".to_string()));

        match remove_task(&mut tasks, 2) {
            Ok(_) => {}
            Err(e) => assert_eq!("Task not found.".to_string(), e),
        }
    }
}
