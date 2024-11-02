use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

pub fn simulate_audio_wave() -> Result<(), Box<dyn std::error::Error>> {

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let file = File::open("./songs/Joji - 777.mp3")?;
    let source = Decoder::new(BufReader::new(file))?;

    //for sample in source.convert_samples::<f32>() {
      //  let amplitude = sample.abs();

        //let bar = (amplitude * 100.0) as usize;
        //println!("{}", bar);

        //thread::sleep(Duration::from_millis(16));
    //}

    let s = source.convert_samples::<f32>();

    Ok(())

}
