use rodio::{Decoder, Source};
use core::f32;
use std::fs::File;
use std::io::BufReader;

pub fn simulate_audio_wave() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("./songs/ODETARI  - KEEP UP [Official Music Video].mp3")?;
    let source = Decoder::new(BufReader::new(file))?;

    let iteem = source.convert_samples::<f32>();

    let mut count = 0;
    for sample in iteem {
        if count == 44100 {
            println!("{}", count);
            break;
        }
        let amplitude = sample.abs();
        // let bar = (amplitude * 500.0) as usize;
        println!("{}", amplitude);
        count += 1;
    }

    Ok(())

}
