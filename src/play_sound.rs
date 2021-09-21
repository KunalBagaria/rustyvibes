pub mod sound {
    use std::thread;
    use play;
    extern crate ears;
    use ears::{ Sound, AudioController };
    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            if file_name.ends_with(".mp3") {
                play::play(&file_name[..]).unwrap();
            } else {
                let mut sound = Sound::new(&file_name[..]).unwrap();
                sound.play();
                while sound.is_playing() {};
            }
        });
    }
}