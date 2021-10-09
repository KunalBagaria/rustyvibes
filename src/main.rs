use fltk::{ app, prelude::*, window::Window };
use std::thread;
use std::process::{ Command };

fn gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 600, "Rustyvibes");
    wind.end();
    wind.show();
    app.run().unwrap();
    Ok(())
}

fn main() {
    thread::spawn(|| {
        drop(Command::new("rustyvibes")
            .arg("/Users/kunalbagaria/Downloads/Soundpacks/nk-cream")
            .spawn()
            .expect("Failed to start echo process"));
    });
    drop(gui());
}