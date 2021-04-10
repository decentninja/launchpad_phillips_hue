use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::error::Error;

pub fn input_handling() -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let midi_in = MidiInput::new("midir reading input")?;
    let in_ports = midi_in.ports();

    let hard_coded = "MIDIIN2 (LPX MIDI)"; // TODO: Read this from file

    let in_port = in_ports.iter().find(|&x| midi_in.port_name(x).unwrap() == hard_coded);
    let in_port = match in_port {
        None => {
            println!("Could could not connect to midi input {}\nFound only:\n", hard_coded);
            for p in in_ports.iter() {
                let name = midi_in.port_name(p).unwrap();
                println!("{}", name);
            }
            Err("Fuck".to_string())?
        }
        Some(port) => port
    };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    Ok(midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        println!("{}: {:?} (len = {})", stamp, message, message.len());
        if message.len() != 3 {
            eprintln!("Got message of length {}. WTF it should be 3, maybe we need to filter messages better.", message.len());
        }
        let light = message[1];
        let value = message[2] as f32 / 127f32;
        println!("Set light {} to {}", light, value);
    }, ())?)
}

pub fn output_handling() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    
    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();

    let hard_coded = "MIDIOUT2 (LPX MIDI)"; // TODO: Read this from file

    let out_port = out_ports.iter().find(|&x| midi_out.port_name(x).unwrap() == hard_coded);
    let out_port = match out_port {
        None => {
            println!("Could could not connect to midi output {}\nFound only:\n", hard_coded);
            for p in out_port.iter() {
                let name = midi_out.port_name(p).unwrap();
                println!("{}", name);
            }
            Err("Fuck".to_string())?
        }
        Some(port) => port
    };
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    // TODO: Get things into programmer mode and do things...
    let _ = conn_out.send(&[127, 1, 1]);
    let _ = conn_out.send(&[127, 1, 1]);
    Ok(conn_out)
}