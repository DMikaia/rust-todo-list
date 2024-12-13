use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    process::{self},
};

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

pub fn get_choice_from_input() -> usize {
    println!("Enter your choice: ");
    println!("1 -> Display all the tasks.");
    println!("2 -> Add a new task.");
    println!("3 -> Remove a task.");
    println!("4 -> Save and Quit.");

    print!("-> ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<usize>().unwrap_or_else(|e| {
            eprintln!("The inupt must be a valid positive number.\n{e}");
            process::exit(1)
        }),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
