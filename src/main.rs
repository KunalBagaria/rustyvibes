use rdev::{ listen, Event };
use std::fs::File;
use std::io::copy;
use std::io::stdout;
use serde_json;

mod play_sound;
mod json;

pub use crate::play_sound::sound;
pub use crate::json::json_data;

fn initialize_json() -> serde_json::Value {
    let mut file = File::open("default/config.json").unwrap();
    let mut stdout = stdout();
    let json_str = &copy(&mut file, &mut stdout).unwrap().to_string();
    let value = serde_json::from_str(json_str).unwrap();
    value
}

fn main() {
    let json_file = json_data::JSONData { data: initialize_json() };
    println!("{}", json_file.data);
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    match event.event_type {
        rdev::EventType::KeyPress(key) => {
            let key_name: String = format!("{:?}", key).to_string();
            sound::play_sound(key_name)
            // println!("{:?}.wav", key)
        },
        _ => ()
    }
}