use std::sync::mpsc::Sender;

use crate::music;
use crate::youtube::video;

pub struct ExecutedCommand<T> {

    pub info: String,
    pub execution_process: Option<T>,

}

impl<T> ExecutedCommand<T> {

    pub fn new(info: String, execution_process: Option<T>) -> Self {

        ExecutedCommand {
            info,
            execution_process
        }

    }

}

pub fn execute_commands(commands: &Vec<String>, api_key: &Option<String>, tx: &Sender<String>) -> ExecutedCommand<String> {

    if commands.len() == 0 {
        return ExecutedCommand::new(String::from("No command to execute"), None)
    }
    else {

        let main_cmd = commands[0].as_str();

        match main_cmd {

            "download" => {

                let mut name = String::new();

                for i in &commands[1..] {

                    let i = i.as_str();

                    if i == "-" {
                        name.clear();
                    }
                    else {
                        name.push_str(format!("{} ", i).as_str());
                    }
                }

                if api_key.is_none() {
                    return ExecutedCommand::new(String::from("No api key for youtube provided"), None)
                }

                match video::find(&name, api_key, 1) {
                    Ok(result) => {

                        if result.items.len() == 0 {
                            return ExecutedCommand::new(String::from("No video found"), None);
                        }

                        let s = format!("{} {}", result.items[0].snippet.title, result.items[0].snippet.channel);
                        let audio_url = music::info::get_music_url(&result.items[0].id.video_id).unwrap();
                        music::downloader::download_audio(&audio_url, result.items[0].snippet.title.as_str(), tx.clone());

                        let task = format!("Download Task   ===>   downloading :::: {}", s);
                        return ExecutedCommand::new(format!("download {}", name), Some(task));
                    }
                    Err(e) => {
                        let s = format!("{}", e);
                        return ExecutedCommand::new(String::from(s), None)
                    }
                }

            }
            _ => return ExecutedCommand::new(String::from("Invalid command"), None)

        };

    }

}
