use app::init_file;
use std::env;
use task::{display_task, load_task};

fn main() {
    let mut target_file = init_file(env::args());

    let tasks = load_task(&mut target_file);

    display_task(&tasks);
}
