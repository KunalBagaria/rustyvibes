use rdev::{ listen, Event };
use serde_json;
use std::error::Error;
use std::fs;
use serde_json::{Value, Map};
use libc;

mod play_sound;
mod macos;

pub use crate::play_sound::sound;
pub use crate::macos::macos_mod;


fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}

fn main() {
    unsafe {
        libc::setpriority(libc::PRIO_PROCESS, 0, -20);
    }
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    let json_file: serde_json::Map<std::string::String, serde_json::Value> = initialize_json("default/config.json").unwrap();
    match event.event_type {
        rdev::EventType::KeyPress(key) => {
            let key_code = macos_mod::code_from_key(key);
            match key_code {
                Some(code) => {
                    let mut dest: String = json_file["defines"][code.to_string().as_str()].to_string();
                    dest.remove(0);
                    dest.remove(dest.len() - 1);
                    sound::play_sound(dest)
                },
                None => println!("{}", "Unknown key")
            }
        },
        _ => ()
    }
}
