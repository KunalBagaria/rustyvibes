pub mod sound {
    use once_cell::sync::Lazy;
    use std::thread;
    use rodio::source::Buffered;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::Mutex;
    use rodio::{ source::Source, Decoder, OutputStream };

    type SoundSource = Buffered<Decoder<BufReader<File>>>;

    static GLOBAL_DATA: Lazy<Mutex<HashMap<String, SoundSource>>> = Lazy::new(|| {
        let m = HashMap::new();
        Mutex::new(m)
    });

    pub fn play_sound(name: String) {
        let file_name = format!("{}", name);
        thread::spawn(move|| {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let source = {
                let mut sound_map = GLOBAL_DATA.lock().unwrap();
                sound_map
                    .entry(name.clone())
                    .or_insert_with(|| {
                        let file = BufReader::new(File::open(&file_name[..]).unwrap());
                        Decoder::new(file).unwrap().buffered()
                    })
                    .clone()
            };
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}