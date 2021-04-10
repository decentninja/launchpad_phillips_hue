extern crate midir;

use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, Ignore};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();

    let hard_coded = "MIDIIN2 (LPX MIDI)"; // TODO: Read this from file

    let in_port = in_ports.iter().find(|&x| midi_in.port_name(x).unwrap() == hard_coded);
    let in_port = match in_port {
        None => {
            println!("Could could not connect to {}\nFound only:\n", hard_coded);
            for p in in_ports.iter() {
                let name = midi_in.port_name(p).unwrap();
                println!("{}", name);
            }
            Err("Fuck".to_string())?
        }
        Some(port) => port
    };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        if message.len() != 3 {
            eprintln!("Got message of length {}. WTF it should be 3.", message.len());
            return;
        }
        let light = message[1];
        let value = message[2] as f32 / 127f32;
        println!("Set light {} to {}", light, value);
    }, ())?;
    
    let mut input = String::new();
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}