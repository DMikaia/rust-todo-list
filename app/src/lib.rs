use clearscreen::clear;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Stdin, Write},
    process::{self},
};
use task::{add_task, remove_task, Task};

pub fn init_file(mut args: impl Iterator<Item = String>) -> File {
    args.next();

    let file_name: String = match args.next() {
        Some(arg) => arg,
        None => "tasks.txt".to_string(),
    };

    match OpenOptions::new().read(true).write(true).open(&file_name) {
        Ok(file) => file,
        Err(_) => File::create(&file_name).unwrap_or_else(|e| {
            eprintln!("Could not create file.\n{e}");
            process::exit(1);
        }),
    }
}

pub fn handle_input(stdin: &Stdin) -> Result<String, String> {
    print!("-> ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => {
            eprintln!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn get_choice_from_input(stdin: &Stdin) -> Result<usize, String> {
    println!("Enter your choice: ");
    println!("1 -> Add a new task.");
    println!("2 -> Remove a task.");
    println!("3 -> Save and Quit.");

    match handle_input(stdin) {
        Ok(choice) => Ok(choice
            .parse::<usize>()
            .expect("The value most be a valid number between 1 and 4")),
        Err(e) => Err(e),
    }
}

pub fn handle_add_task(stdin: &Stdin, tasks: &mut HashMap<usize, Task>) -> Result<(), String> {
    println!("\nEnter the new task:");
    let description = match handle_input(stdin) {
        Ok(desc) => desc,
        Err(e) => return Err(e),
    };

    match add_task(tasks, description) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn handle_remove_task(stdin: &Stdin, tasks: &mut HashMap<usize, Task>) -> Result<(), String> {
    println!("\nEnter the id of the task:");
    let id = match handle_input(stdin) {
        Ok(id) => id.parse::<usize>().expect("The id most be a valid number"),
        Err(e) => return Err(e),
    };

    match remove_task(tasks, id) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn wait_and_clear(stdin: &Stdin) {
    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    clear().expect("Could not clear the screen");
}
