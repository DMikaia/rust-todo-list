mod task_list;

use std::io;
use task_list::*;

fn main() {
    let mut tasks: Vec<Task> = load_task();

    loop {
        println!("\nTo-do list");
        println!("1 - Add a task");
        println!("2 - Remove a task");
        println!("3 - View all tasks");
        println!("4 - Save and exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Faild to read line");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&mut tasks),
            2 => remove_task(&mut tasks),
            3 => view_task(&mut tasks),
            4 => {
                save_task(&mut tasks);
                println!("Task saved successfully, exiting...");
                break;
            }
            _ => println!("Invalid option, try again"),
        }
    }
}
