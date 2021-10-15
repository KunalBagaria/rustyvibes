use fltk::{ app, prelude::*, window::Window };
use std::thread;
use std::process::{ Command };
use std::env;
use std::fs;

fn gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 600, "Rustyvibes");
    wind.end();
    wind.show();
    app.run().unwrap();
    Ok(())
}

fn rustyvibes_path() -> Result<String, Box<dyn std::error::Error>> {
    let mut path = env::current_exe()?;
    path.pop();
    path.pop();
    path.push("Resources/resources/rustyvibes");
    Ok(path.to_str().unwrap().to_string())
}

fn soundpacks_path() -> Result<String, Box<dyn std::error::Error>> {
    let mut path = env::current_exe()?;
    path.pop();
    path.pop();
    path.push("Resources/resources/soundpacks");
    Ok(path.to_str().unwrap().to_string())
}

fn main() {
    let soundpacks_path = soundpacks_path().unwrap();
    let paths = fs::read_dir(soundpacks_path).unwrap();

    for (i, f) in paths.enumerate() {
        if i == 8 {
            thread::spawn(|| {
                let rustyvibes_path = rustyvibes_path().unwrap();
                let soundpack_path = f.unwrap().path().display().to_string();
                println!("{}", rustyvibes_path);
                drop(Command::new(rustyvibes_path)
                    .arg(soundpack_path)
                    .spawn()
                    .expect("Failed to start echo process"));
            });
            drop(gui());
        }
    }
}