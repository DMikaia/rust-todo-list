use app::{get_choice_from_input, handle_add_task, handle_remove_task, init_file, wait_and_clear};
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
            choice => {
                match choice {
                    1 => handle_add_task(&stdin, &mut tasks).unwrap_or_else(|e| eprintln!("\n{e}")),
                    2 => handle_remove_task(&stdin, &mut tasks)
                        .unwrap_or_else(|e| eprintln!("\n{e}")),
                    _ => eprintln!("\nThe value most be in range of 1 to 3."),
                }
                wait_and_clear(&stdin);
            }
        }
    }

    Ok(())
}
