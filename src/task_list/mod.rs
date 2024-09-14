use std::{fs::File, io::*};

pub struct Task {
    description: String,
}

pub fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();

    println!("The new task description : ");
    stdin()
        .read_line(&mut description)
        .expect("Error reading description");

    let task = Task {
        description: description.to_string().trim().to_owned(),
    };

    tasks.push(task);
    println!("Task added successfully");
}

pub fn remove_task(tasks: &mut Vec<Task>) {
    let mut task_index = String::new();

    println!("Task index : ");
    stdin()
        .read_line(&mut task_index)
        .expect("Error reading description");

    let task_index: usize = match task_index.trim().parse() {
        Ok(index) => index,
        Err(_) => return,
    };

    if task_index == 0 || task_index > tasks.len() {
        println!("Invalid task index");
    } else {
        tasks.remove(task_index - 1);
        println!("Task has been successfully removed");
    }
}

pub fn view_task(tasks: &mut Vec<Task>) {
    if tasks.len() == 0 {
        println!("No tasks to display");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            print!("{} - {}\n", index + 1, task.description);
        }
    }
}

pub fn save_task(tasks: &mut Vec<Task>) {
    let mut file = File::create("tasks.txt").expect("Could not create task file");

    for (i, task) in tasks.iter().enumerate() {
        if i < tasks.len() - 1 {
            write!(file, "{}\n", task.description).expect("Could not write task description");
        } else {
            write!(file, "{}", task.description).expect("Could not write task description");
        }
    }
}

pub fn load_task() -> Vec<Task> {
    let file: std::result::Result<File, Error> = File::open("tasks.txt");
    let mut tasks: Vec<Task> = Vec::new();

    if let Ok(file) = file {
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(description) = line {
                tasks.push(Task { description });
            }
        }
    }

    tasks
}
