pub mod sound {
    use std::thread;
    use play;
    // extern crate ears;
    // use ears::{ Sound, AudioController };
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};

    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            if file_name.ends_with(".mp3") {
                play::play(&file_name[..]).unwrap();
            } else {
                // let mut sound = Sound::new(&file_name[..]).unwrap();
                // sound.play();
                // while sound.is_playing() {};
                // Get a output stream handle to the default physical sound device
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                // Load a sound from a file, using a path relative to Cargo.toml
                let file = BufReader::new(File::open(&file_name[..]).unwrap());
                // Decode that sound file into a source
                let source = Decoder::new(file).unwrap();
                // Play the sound directly on the device
                drop(stream_handle.play_raw(source.convert_samples()));
    
                // The sound plays in a separate audio thread,
                // so we need to keep the main thread alive while it's playing.
                std::thread::sleep(std::time::Duration::from_millis(700));
            }
        });
    }
}