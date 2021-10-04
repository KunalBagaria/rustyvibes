pub mod sound {
    use std::thread;
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{ source::Source, Decoder, OutputStream };

    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let source = {
                let file = BufReader::new(File::open(&file_name[..]).unwrap());
                Decoder::new(file).unwrap().buffered()
            };
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}