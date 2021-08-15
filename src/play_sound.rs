pub mod sound {
    use std::thread;
    use play;
    
    pub fn play_sound(name: String) {
        let file_name = format!("./sounds/{}.ogg", name.to_lowercase());
        println!("{}", &file_name[..]);
        
        thread::spawn(move|| {
            play::play("/Users/kunalbagaria/Desktop/rustyvibes/default/key6.mp3").unwrap();
        });
    }
}