use anyhow::Result;
use rodio::{source::Source, Decoder};
use std::io::BufReader;
use std::path::Path;
use std::{env, fs::File};
use wav::WAV_FORMAT_PCM;

struct SoundFile {
    data: Vec<i16>,
    sample_rate: usize,
    num_channels: usize,
}

impl SoundFile {
    fn from_ogg(path: &Path) -> Result<Self> {
        let ogg_file = BufReader::new(File::open(path).unwrap());
        let decoded_file = Decoder::new(ogg_file).unwrap();
        let sample_rate = decoded_file.sample_rate() as usize;
        let num_channels = decoded_file.channels() as usize;
        let data: Vec<i16> = decoded_file.collect();
        Ok(SoundFile {
            data,
            sample_rate,
            num_channels,
        })
    }

    fn split(self) -> (SoundFile, SoundFile) {
        let len = self.data.len();
        (
            SoundFile {
                data: self.data[..len / 2].to_vec(),
                sample_rate: self.sample_rate,
                num_channels: self.num_channels,
            },
            SoundFile {
                data: self.data[len / 2..].to_vec(),
                sample_rate: self.sample_rate,
                num_channels: self.num_channels,
            },
        )
    }

    fn trim(&mut self) {
        let mut intensities = self
            .data
            .chunks(20)
            .map(|x| x.iter().map(|y| y.abs() as isize).sum());
        let mut max_val = 0;
        for x in intensities.clone() {
            if x > max_val {
                max_val = x;
            }
        }
        let start_index = intensities
            .position(|x| x > max_val / 10)
            .expect("No start_index");
        // Back up the starting index chunk by one for a conservative start
        // and then map back to sample coordinates
        let start_index = (start_index.saturating_sub(5)) * 20;
        self.data.rotate_left(start_index);
        self.data.truncate(self.data.len() - start_index);
    }

    fn to_wav(&self, path: &Path) -> Result<()> {
        let data = wav::bit_depth::BitDepth::Sixteen(self.data.clone());
        let mut file = File::create(path)?;
        wav::write(
            wav::Header::new(WAV_FORMAT_PCM, 2, self.sample_rate as u32, 16),
            &data,
            &mut file,
        )
        .map_err(|x| x.into())
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(anyhow::anyhow!("Need a path to an ogg file"))
    } else {
        let path = Path::new(&args[1]);
        assert!(
            path.is_dir() && path.join("config.json").is_file(),
            "Need a path to a directory containing config.json"
        );
        let dir = std::fs::read_dir(path)?;
        for file in dir {
            let file = file.unwrap().path();
            if let Some(extension) = file.extension() {
                if extension == "ogg" {
                    let s = SoundFile::from_ogg(&file)?;
                    let (mut s1, mut s2) = s.split();
                    s1.trim();
                    s2.trim();
                    s1.to_wav(&file.with_extension("wav"))?;
                    s2.to_wav(&file.with_extension("up.wav"))?;
                }
            }
        }
        Ok(())
    }
}
