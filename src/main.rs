extern crate midir;

use std::io::{stdin};
use std::error::Error;

mod launchpad;
use launchpad::*;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let _input_midi = input_handling();
    let _output_midi = output_handling();

    // Keep program alive until enter
    let mut input = String::new();
    input.clear();
    stdin().read_line(&mut input)?;
    println!("Bye!");
    Ok(())
}