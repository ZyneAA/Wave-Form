use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use rodio::{Decoder, OutputStream, source::Source};

pub fn play_audio_from_local(audio_url: &str, length: u64) -> Result<(), Box<dyn Error>> {

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let file = File::open(audio_url)?;
    let source = Decoder::new(BufReader::new(file))?;
    stream_handle.play_raw(source.convert_samples())?;
    std::thread::sleep(std::time::Duration::from_secs(length));

    Ok(())

}

