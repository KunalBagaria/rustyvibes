use rdev::{ listen, Event };

mod play_sound;
pub use crate::play_sound::sound;

fn main() {
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