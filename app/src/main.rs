use app::{get_choice_from_input, init_file};
use std::{collections::HashMap, env, fs::File};
use task::{display_task, load_task, save_task, Task};

fn main() -> Result<(), String> {
    let mut target_file: File = init_file(env::args());
    let tasks: HashMap<usize, Task> = load_task(&mut target_file);

    loop {
        println!();
        match get_choice_from_input() {
            1 => display_task(&tasks),
            2 => println!("Choice = 2"),
            3 => println!("Choice = 3"),
            4 => {
                println!("Shutting down...");
                save_task(&mut target_file, tasks)?;

                break;
            }
            _ => {
                eprintln!("Only enter a value between 1 and 4.");
            }
        }
        println!();
    }

    Ok(())
}
