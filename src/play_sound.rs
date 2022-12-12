pub mod sound {
    use flume::{Receiver, Sender};
    use once_cell::sync::Lazy;
    use rodio_wav_fix::source::Buffered;
    use rodio_wav_fix::{source::Source, Decoder, OutputStream};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    type SoundSource = Buffered<Decoder<BufReader<File>>>;

    static GLOBAL_DATA: Lazy<Mutex<HashMap<String, SoundSource>>> = Lazy::new(|| {
        let m = HashMap::new();
        Mutex::new(m)
    });

    static WORKER_CHANNEL: Lazy<Mutex<Sender<String>>> = Lazy::new(|| Mutex::new(new_worker()));

    fn new_worker() -> Sender<String> {
        let (tx, rx) = flume::unbounded();
        thread::spawn(move || {
            worker(rx);
        });
        tx
    }

    pub fn play_sound(name: String, volume: u16) {
        let mut tx = WORKER_CHANNEL.lock().unwrap();
        if tx.is_disconnected() {
            *tx = new_worker()
        }
        tx.send(format!("{};{}", name, volume.to_string())).expect("Couldn't send name to threadpool");
        
    }

    pub fn worker(rx_channel: Receiver<String>) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        loop {
            if let Ok(raw) = rx_channel.recv_timeout(Duration::from_secs(20)) {
                // The data sent format is <file_name>;<volume>. 
                let data: Vec<&str> = raw.split(";").collect();
                let name = data[0].to_string();
                let volume = data[1].parse::<u16>().expect("Cannot parse volume.");
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
                let sink = rodio_wav_fix::Sink::try_new(&stream_handle).unwrap();
                // Since sink.set_volume accepts value from range (0 - 1.0), the pased volume
                // should be divided by 100.
                let vol = volume as f32 / 100.0;
                sink.set_volume(vol);
                sink.append(source);
                sink.detach();
            } else {
                // Timeout, time to put this thread to sleep to save CPU cycles (open audio OutputStreams use
                // around half a CPU millicore, and then CoreAudio uses another 7-10%)
                break;
            }
        }
    }
}
