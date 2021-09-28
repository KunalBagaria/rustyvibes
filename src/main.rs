use rdev::{ listen, Event };
use serde_json;
use std::env;
use std::error::Error;
use std::fs;
use serde_json::{ Value, Map };
use libc::{ nice };

mod play_sound;
mod keycode;

pub use crate::play_sound::sound;
pub use crate::keycode::key_code;

fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}

pub struct JSONFile {
    pub value: Option<serde_json::Map<std::string::String, serde_json::Value>>
}

impl JSONFile {
    pub fn initialize(&mut self) {
        let args: Vec<String> = env::args().collect();
        let directory = args[1].clone();
        let soundpack_config = &format!("{}/config.json", directory)[..];
        self.value = Some(initialize_json(soundpack_config).unwrap());
    }
    pub fn event_handler(self: &Self, event: Event) {
        match &self.value {
            Some(value) => {
                callback(event, value.clone());
            },
            None => { println!("JSON wasn't initialized"); }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████

Usage: rustyvibes <soundpack_path>
"#);

    } else {
        unsafe { nice(-20) };
        
        let mut json_file = JSONFile { value: None };
        json_file.initialize();

        let event_handler = move |event: Event| {
            json_file.event_handler(event);
        };

        if let Err(error) = listen(event_handler) {
            println!("Error: {:?}", error)
        }
    }
}

fn callback(event: Event, json_file: serde_json::Map<std::string::String, serde_json::Value>) {
    match event.event_type {
        rdev::EventType::KeyPress(key) => {
            let args: Vec<String> = env::args().collect();
            let directory = args[1].clone();
            // let json_file: serde_json::Map<std::string::String, serde_json::Value> = initialize_json(soundpack_config).unwrap();
            let key_code = key_code::code_from_key(key);
            match key_code {
                Some(code) => {
                    let mut dest: String = json_file["defines"][code.to_string().as_str()].to_string();
                    dest.remove(0);
                    dest.remove(dest.len() - 1);
                    sound::play_sound(format!("{}/{}", directory, dest));
                },
                None => {
                    let mut dest: String = json_file["sound"].to_string();
                    dest.remove(0);
                    dest.remove(dest.len() - 1);
                    sound::play_sound(format!("{}/{}", directory, dest));
                }
            }
        },
        _ => ()
    }
}