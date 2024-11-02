use std::process::Command;
use std::error::Error;
use std::sync::mpsc::Sender;
use std::thread;

use crate::wave::WaveErr;

pub fn download_audio(audio_url: &str, file_name: &str, tx: Sender<String>) {


    let audio_url = audio_url.to_string();
    let file_name = file_name.to_string();

    let _ = thread::spawn(move || {


        let s = match download(&file_name, &audio_url) {

            Ok(k) => k,
            Err(e) => format!("{}", e)

        };

        tx.send(format!("Download complete: {}", s)).unwrap();

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
        let err = WaveErr::new(String::from("Failed to start yt-dlp, make sure yt-dlp is properly installed!"));
        Err(Box::new(err))
    }

}
