mod keycode;
mod play_sound;
mod args;
mod start;

use clap::Parser;
use crate::args::ArgParser;

const ASCII_ART: &str =
    r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;


fn main() {
    println!("{}", ASCII_ART);
    let a = ArgParser::parse();
    let soundpack = a.soundpack;
    let vol = a.volume.or(Some(100)).unwrap();

    start::rustyvibes::start_rustyvibes(soundpack, vol);
}
