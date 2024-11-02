use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::sync::mpsc::Sender;

use rodio::{ source::SamplesConverter, Decoder, Sink, Source };

use super::song::Song;
use super::super::wave::WaveErr;

pub fn play_audio(sink: &Sink, mut song: Song) -> Result<(), Box<dyn Error>> {

    sink.clear();
    song.add_source();

    match song.source {
        Some(s) => {
            sink.append(s);
        },
        None => {
            let err = WaveErr::new(String::from("No Source found!"));
            Err(Box::new(err))?
        }
    };

    sink.play();

    Ok(())

}

