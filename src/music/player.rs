use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Sender;

use rodio::{Decoder, Sink};

use super::song::Song;
use super::super::wave::WaveErr;

pub fn play_audio(sink: &Sink, mut song: Song, wave_tx: &Sender<Decoder<BufReader<File>>>) -> Result<(), Box<dyn Error>> {

    sink.clear();
    song.add_source();

    let source = song.get_source();

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

    wave_tx.send(source);

    Ok(())

}

