use app::{get_choice_from_input, handle_add_task, init_file};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{stdin, Stdin},
};
use task::{display_task, load_task, save_task, Task};

fn main() -> Result<(), String> {
    let mut target_file: File = init_file(env::args());
    let mut tasks: HashMap<usize, Task> = load_task(&mut target_file);
    let stdin: Stdin = stdin();

    println!();
    loop {
        match get_choice_from_input(&stdin)? {
            1 => display_task(&tasks),
            2 => handle_add_task(&stdin, &mut tasks).unwrap_or_else(|e| eprintln!("\n{e}")),
            3 => println!("Choice = 3"),
            4 => {
                println!("Shutting down...\n");
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
