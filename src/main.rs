mod keycode;
mod play_sound;
mod start;

use std::env;

const ASCII_ART: &str =
    r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;

fn help() {
    println!("{}
Usage: rustyvibes <soundpack_path>", ASCII_ART);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help();
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
                println!("{}", ASCII_ART);
                start::rustyvibes::start_rustyvibes(args[1].clone());
            }
        }
    }
}