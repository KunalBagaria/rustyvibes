pub mod sound {
    use std::thread;
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};

    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open(&file_name[..]).unwrap());
            let source = Decoder::new(file).unwrap();
            drop(stream_handle.play_raw(source.convert_samples()));
            std::thread::sleep(std::time::Duration::from_millis(700));
        });
    }
}