use rdev::{ listen, Event };
use serde_json;
use std::error::Error;
use std::fs;
use serde_json::{Value, Map};

mod play_sound;
mod json;

pub use crate::play_sound::sound;
// pub use crate::json::json_data::JSONData;


fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}

fn main() {
    let json_file = initialize_json("default/config.json").unwrap();
    println!("{:#?}", json_file["defines"]);
    // println!("{:#?}", json_file.data);
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