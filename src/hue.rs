use hueclient::Bridge;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn read_user_string() -> Option<String> {
    let filename = "hue_user";
    let mut file = File::open(filename).ok()?;
    let mut user = String::new();
    file.read_to_string(&mut user).ok()?;
    Some(user)
}

pub struct Message {
    pub light: usize,
    pub strength: f32
}

pub fn hue(rx: Receiver<Message>) -> Result<(), Box<dyn Error>> {
    let bridge = Bridge::discover().ok_or_else(|| "No bridge")?;
    let bridge = match read_user_string() {
        None => {
            println!("Press the bridge");
            let bridge = bridge.register_user("launchpad#svarta-sara")?;
            println!("User registered!");
            match File::create("hue_user") {
                Ok(mut file) => {
                    file.write(bridge.username.as_bytes()).unwrap();
                },
                Err(e) => {
                    eprintln!("Could not open \"user\" file to write username. Will continue, but you'll have to click the button again on next boot. {:?}", e);
                }
            };
            bridge
        }
        Some(username) => {
            bridge.with_user(username)
        }
    };
    println!("Light groups and their ids");
    for scene in &bridge.get_all_groups()? {
        println!("{:?}", scene);
    }
    let mut debounce = HashMap::<usize, f32>::new();
    loop {
        thread::sleep(Duration::new(1, 0));
        loop {
            // Empty out the que before continueing, to avoid flooding the bridge with messages
            match rx.try_recv() {
                Ok(new_message) => {
                    let entry = debounce.entry(new_message.light).or_insert(0f32);
                    *entry = new_message.strength;
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                _ => {
                    eprintln!("End of stream");
                    return Ok(());
                }
            }
        }
        for (light, strength) in debounce.iter() {
            let light = match light {
                // Remaps
                4 => 5,
                5 => 6,
                6 => 7,
                7 => 13,
                8 => 33,
                a => *a
            };
            println!("Light {} to {}", light, strength);
            let mut cmd = hueclient::CommandLight::default();
            if strength < &0.1f32 {
                cmd = cmd.off();
            } else {
                cmd = cmd.on().with_bri((strength * 255f32) as u8)
            }
            bridge.set_group_state(light, &cmd)?;
        }
        debounce.clear();
    }
}