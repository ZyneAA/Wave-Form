use std::process::Command;
use std::thread::{ self, JoinHandle };
use std::error::Error;

pub fn download_audio(audio_url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {

    let command = Command::new("yt-dlp")
        .args(&["-x", "--audio-format", "mp3", "--output", file_name, audio_url])
        .output()?;

    if command.status.success() {
        println!("Command executed successfully");
    }
    else {
        eprintln!("Command failed with error: {:?}", command);
    }

    Ok(())

}

pub fn threaded_download_audio(audio_url: &str, file_name: &str) -> Result<JoinHandle<Result<(), String>>, Box<dyn Error>> {


    let audio_url = audio_url.to_string();
    let file_name = file_name.to_string();


    let handle: JoinHandle<Result<(), String>> = thread::spawn(move || {

        println!("Preparing to download and write it to {}", file_name);

        let command = Command::new("yt-dlp")
            .args(&["-x", "--audio-format", "mp3", "--output", &file_name, &audio_url])
            .output();

        match command {

            Ok(output) => {
                if output.status.success() {
                    println!("Download completed");
                    // let _ = tx.send(Ok(()));
                }
                else {
                    eprintln!("Download failed with error: {:?}", output);
                    // let _ = tx.send(Err("Command failed".into()));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute download: {:?}", e);
                // let _ = tx.send(Err("Failed".into()));
            }

        };

        Ok(())

    });

    Ok(handle)

    /* match rx.recv()? {

        Ok(_) => {
            println!("donwload success");
            Ok(())
        },
        Err(e) => {
            println!("download failed");
            Err(e.into())
        }

    } */

}
