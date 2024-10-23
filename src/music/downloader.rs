use std::process::Command;
use std::error::Error;
use std::thread;
use std::sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } };

use crate::wave::WaveErr;

pub fn download_audio(audio_url: &str, file_name: &str, worker_output_clone: Arc<Mutex<Option<String>>>, is_worker_finished_clone: Arc<AtomicBool>) {


    let audio_url = audio_url.to_string();
    let file_name = file_name.to_string();

    let _ = thread::spawn(move || {


        let s = match download(&file_name, &audio_url) {

            Ok(k) => k,
            Err(e) => format!("{}", e)

        };

        let mut output = worker_output_clone.lock().unwrap();
        *output = Some(s);
        is_worker_finished_clone.store(true, Ordering::SeqCst);

    });

}

fn download(file_name: &str, audio_url: &str) -> Result<String, Box<dyn Error>> {

    let command = Command::new("yt-dlp")
        .args(&["-x", "--audio-format", "mp3", "--output", &file_name, &audio_url])
        .output()?;

    if command.status.success() {
        Ok(String::from("Download success"))
    }
    else{
        let err = WaveErr::new(String::from("Failed to start yt-dlp, make sure yt-dlp is properly installed!"));
        Err(Box::new(err))
    }

}
