pub mod sound {
    use std::thread;
    use play;
    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            play::play(&file_name[..]).unwrap();
        });
    }
}