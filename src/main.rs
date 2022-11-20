mod keycode;
mod play_sound;
mod start;

use std::env;

const ASCII_ART: &str = r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;

fn help() {
    println!(
        "{}
Usage: rustyvibes <volume> <soundpack_path>",
        ASCII_ART
    );
    println!("volume [0-1.0] :Set the volume for rustyvibes, value between 0 and 1.0");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help();
    } else if args.len() == 3 {
        println!("{}", ASCII_ART);
        let vol = &args[1];
        let float: f32 = match vol.parse() {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error: not an integer!");
                help();
                return;
            }
        };
        start::rustyvibes::start_rustyvibes(float, args[2].clone());
    } else {
        match args[1].as_str() {
            "--help" => {
                help();
            }
            "--version" => {
                println!("{}", ASCII_ART);
                println!("You are on version {}", env!("CARGO_PKG_VERSION"));
            }
            _ => {
                help();
            }
        }
    }
}



