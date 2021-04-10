use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod launchpad;
use launchpad::*;
mod hue;


fn main() {
    loop {
        match run() {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err)
        }
        thread::sleep(Duration::new(5, 0));
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    // We need these two variables to stick around for the midi to run
    let _input_midi = input_handling(tx)?;
    let _output_midi = output_handling()?;
    hue::hue(rx)?;
    Ok(())
}