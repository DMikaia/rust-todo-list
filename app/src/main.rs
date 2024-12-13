use app::{get_choice_from_input, handle_actions, init_file};
use clearscreen::clear;
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

    clear().expect("Could not clear the screen");

    loop {
        display_task(&tasks);
        println!();
        match get_choice_from_input(&stdin)? {
            3 => {
                println!("\nShutting down...\n");
                save_task(&mut target_file, tasks)?;

                break;
            }
            choice => handle_actions(&stdin, choice, &mut tasks),
        }
    }

    Ok(())
}
