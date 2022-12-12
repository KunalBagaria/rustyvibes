pub mod rustyvibes {

    use rdev::{listen, Event};
    use serde_json;
    use serde_json::{Map, Value};
    use std::error::Error;
    use std::fs;

    pub use crate::keycode::key_code;
    pub use crate::play_sound::sound;

    fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }

    pub struct JSONFile {
        pub value: Option<serde_json::Map<std::string::String, serde_json::Value>>,
    }

    impl JSONFile {
        pub fn initialize(&mut self, directory: String) {
            let soundpack_config = &format!("{}/config.json", directory)[..];
            self.value = Some(initialize_json(soundpack_config).unwrap());
        }
        pub fn event_handler(self: &Self, event: Event, directory: String, vol: u16) {
            match &self.value {
                Some(value) => {
                    callback(event, value.clone(), directory, vol);
                }
                None => {
                    println!("JSON wasn't initialized");
                }
            }
        }
    }

    pub fn start_rustyvibes(args: String, vol: u16) {
        {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            unsafe {
                use libc::nice;
                nice(-20)
            };
        }

        {
            #[cfg(target_os = "windows")]
            {
                use thread_priority::*;
                assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
            }
        }

        let mut json_file = JSONFile { value: None };
        json_file.initialize(args.clone());

        println!("Soundpack configuration loaded");
        println!("Rustyvibes is running");

        let event_handler = move |event: Event| {
            json_file.event_handler(event, args.clone(), vol);
        };

        if let Err(error) = listen(event_handler) {
            println!("Error: {:?}", error)
        }
    }

    use once_cell::sync::Lazy;
    use std::collections::HashSet;
    use std::sync::Mutex;

    static KEY_DEPRESSED: Lazy<Mutex<HashSet<i32>>> = Lazy::new(|| Mutex::new(HashSet::new()));

    fn callback(event: Event, json_file: serde_json::Map<std::string::String, serde_json::Value>, directory: String, vol: u16) {
        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                let key_code = key_code::code_from_key(key);
                let key_down = KEY_DEPRESSED
                    .lock()
                    .expect("Can't open key_depressed set")
                    .insert(key_code.unwrap_or(0));
                if key_down {
                    let mut dest = match key_code {
                        Some(code) => json_file["defines"][&code.to_string()].to_string(),
                        None => {
                            println!("Unmapped key: {:?}", key); // for debugging
                            let default_key = 30; // keycode for 'a'
                            json_file["defines"][&default_key.to_string()].to_string()
                        }
                    };
                    dest.remove(0);
                    dest.remove(dest.len() - 1);
                    sound::play_sound(format!("{}/{}", directory, dest), vol);
                }
            }
            rdev::EventType::KeyRelease(key) => {
                let key_code = key_code::code_from_key(key);
                KEY_DEPRESSED
                    .lock()
                    .expect("Can't open key_depressed set for removal")
                    .remove(&key_code.unwrap_or(0));
                // println!("In the future, this'll trigger the keyup sound")
            }
            _ => (),
        }
    }
}