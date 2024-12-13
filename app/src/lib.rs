use std::{
    fs::{File, OpenOptions},
    io, process,
};

pub fn init_file(mut args: impl Iterator<Item = String>) -> File {
    args.next();

    let file_name: String = match args.next() {
        Some(arg) => arg,
        None => "tasks.txt".to_string(),
    };

    match OpenOptions::new().read(true).write(true).open(&file_name) {
        Ok(file) => file,
        Err(_) => File::create(&file_name).expect("Could not create file"),
    }
}

pub fn get_choice_from_input() -> usize {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<usize>().unwrap_or_else(|e| {
            eprintln!("The inupt must be a valid positive number between 1 and 4.\n{e}");
            process::exit(1)
        }),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
