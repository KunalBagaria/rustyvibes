pub mod sound {
    use std::thread;
    use play::play;
    
    pub fn play_sound(name: String) {
        
        let file_name = format!("./sounds/{}.ogg", name.to_lowercase());
        println!("{}", &file_name[..]);
        
        thread::spawn(move|| {
            play::play("/Users/kunalbagaria/Desktop/rustyvibes/default/key1.mp3").unwrap();
        });

        // drop();

        // let echo_child = Command::new("afplay")
        //     .arg("default/keystroke.wav")
        //     // .stdout(Stdio::piped())
        //     .spawn()
        //     .expect("Failed to start echo process");
        // drop(echo_child);
    }
}