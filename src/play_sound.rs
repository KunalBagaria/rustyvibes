pub mod sound {
    use once_cell::sync::Lazy;
    use rodio::source::Buffered;
    use rodio::{source::Source, Decoder, OutputStream};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::Mutex;
    use std::thread;

    type SoundSource = Buffered<Decoder<BufReader<File>>>;

    static GLOBAL_DATA: Lazy<Mutex<HashMap<String, SoundSource>>> = Lazy::new(|| {
        let m = HashMap::new();
        Mutex::new(m)
    });

    static THREAD_POOL_SENDER: Lazy<flume::Sender<String>> = Lazy::new(|| {
        let (tx, rx) = flume::unbounded();
        for _ in 0..10 {
            let rxc = rx.clone();
            thread::spawn(move || {
                worker(rxc);
            });
        }
        tx
    });

    pub fn play_sound(name: String) {
        THREAD_POOL_SENDER
            .send(name)
            .expect("Couldn't send name to threadpool");
    }

    pub fn worker(rx_channel: flume::Receiver<String>) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        loop {
            let name = rx_channel
                .recv()
                .expect("Couldn't receive file name in worker thread");
            let file_name = format!("{}", name);
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
        }
    }
}
