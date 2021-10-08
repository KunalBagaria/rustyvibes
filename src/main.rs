mod listen;
mod keycode;
mod play_sound;

use listen::rustyvibes;
use fltk::{ app, prelude::*, window::Window };
use std::thread;

fn gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 600, "Rustyvibes");
    wind.end();
    wind.show();
    app.run().unwrap();
    Ok(())
}

fn main() {
    thread::spawn(move || {
        // rustyvibes::start_listening("/Users/kunalbagaria/Downloads/Soundpacks/nk-cream".to_string());
    });
    drop(gui());
}