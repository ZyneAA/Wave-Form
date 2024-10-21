use std::process::Command;
use std::thread::{ self, JoinHandle };
use std::error::Error;

pub fn download_audio(audio_url: &str, file_name: &str) -> Result<JoinHandle<Result<(), String>>, Box<dyn Error>> {


    let audio_url = audio_url.to_string();
    let file_name = file_name.to_string();


    let handle: JoinHandle<Result<(), String>> = thread::spawn(move || {

        let _ = Command::new("yt-dlp")
            .args(&["-x", "--audio-format", "mp3", "--output", &file_name, &audio_url])
            .output();

        Ok(())

    });

    Ok(handle)

}
