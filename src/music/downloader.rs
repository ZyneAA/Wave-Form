use std::process::Command;
use std::error::Error;
use std::sync::mpsc::Sender;
use std::thread;

use crate::wave::WaveErr;
use super::song::{add_meta_data_to_mp3, MetaData};

pub fn download_audio(audio_url: &str, file_name: &str, tx: Sender<String>, meta_data: MetaData) {


    let audio_url = audio_url.to_string();
    let file_name = file_name.to_string();

    let _ = thread::spawn(move || {


        let s = match download(&file_name, &audio_url) {

            Ok(k) => {

                let path = format!("songs/{}.mp3", file_name);
                add_meta_data_to_mp3(&path, meta_data);

                k

            },
            Err(e) => format!("{}", e)

        };

        tx.send(format!("Download Completed: {}", s)).unwrap();

    });

}

fn download(file_name: &str, audio_url: &str) -> Result<String, Box<dyn Error>> {

    let file_name = file_name.replace('/', "-");

    let audio_path = format!("songs/{}", file_name);

    let command = Command::new("yt-dlp")
        .args(&["-x", "--audio-format", "mp3", "--output", &audio_path, audio_url])
        .output()?;

    if command.status.success() {
        let s = file_name;
        Ok(String::from(s))
    }
    else{
        let err = WaveErr::new(format!("{}", &audio_path));
        Err(Box::new(err))
    }

}
