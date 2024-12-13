use app::{init_file, run_app};
use std::{env, fs::File};

/// Main function of the todo list app
fn main() -> Result<(), String> {
    let target_file: File = init_file(env::args());

    run_app(target_file)?;

    Ok(())
}
