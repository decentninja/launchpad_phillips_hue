use std::error::Error;
use std::sync::mpsc;

mod launchpad;
use launchpad::*;
mod hue;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    // We need these two variables to stick around for the midi to run
    let _input_midi = input_handling(tx)?;
    let _output_midi = output_handling()?;
    hue::hue(rx)?;

    /*
    // Keep program alive until enter
    let mut input = String::new();
    input.clear();
    stdin().read_line(&mut input)?;
    println!("Bye!");
    */
    Ok(())
}