use std::sync::mpsc::Sender;

use crate::helper;
use crate::music;
use crate::music::song::MetaData;
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

                let flag_as_second_arg_checker = commands[1].as_str();
                if helper::is_flag(flag_as_second_arg_checker) {
                    return ExecutedCommand::new(String::from("Can't have a flag as first arg for download!"), None);
                }

                if api_key.is_none() {
                    return ExecutedCommand::new(String::from("No api key for youtube provided"), None)
                }

                let mut name = String::new();
                let mut name_song = String::new();
                let mut artist = String::new();
                let mut album = String::new();
                let mut genere = String::new();
                let mut release_date = String::new();

                let mut flag_found = false;
                let mut flag = 0;
                // flag types
                // 0(not found), 
                // 1(rename), 
                // 2(artist name),
                // 3(album)

                for i in &commands[1..] {

                    let i = i.as_str();

                    if flag_found {
                        match flag {
                            1 => {
                                flag = 0;
                                flag_found = false;
                                name_song.push_str(i);
                                continue;
                            },
                            2 => {
                                flag = 0;
                                flag_found = false;
                                artist.push_str(i);
                                continue;
                            },
                            3 => {
                                flag = 0;
                                flag_found = false;
                                album.push_str(i);
                                continue;
                            },
                            4 => {
                                flag = 0;
                                flag_found = false;
                                genere.push_str(i);
                                continue;
                            },
                            _ => {}
                        }
                    }

                    match i {
                        "-" => {
                            name.clear();
                        }
                        "-rn" => {
                            flag_found = true;
                            flag = 1;
                            continue;
                        }
                        "-an" => {
                            flag_found = true;
                            flag = 2;
                            continue;
                        }
                        "-a" => {
                            flag_found = true;
                            flag = 3;
                            continue;
                        }
                        "-gr" => {
                            flag_found = true;
                            flag = 4;
                            continue;
                        }
                        _ => {
                            name.push_str(format!("{}", i).as_str());
                        }
                    }

                }

                match video::find(&name, api_key, 1) {

                    Ok(result) => {

                        if result.items.len() == 0 {
                            return ExecutedCommand::new(String::from("No video found"), None);
                        }

                        let s = format!("{}", result.items[0].snippet.title);
                        let audio_url = music::info::get_music_url(&result.items[0].id.video_id).unwrap();
                        let command = commands.join(" ");

                        let meta_data = MetaData::new(
                                assert_if_given(artist),
                                assert_if_given(album),
                                assert_if_given(genere),
                                None,
                                assert_if_given(release_date)
                        );

                        let task = format!("Downloading :::: {}", s);

                        if name_song.len() >= 1 {

                            music::downloader::download_audio(&audio_url, name_song.as_str(), tx.clone(), meta_data);
                            return ExecutedCommand::new(command, Some(task));

                        }
                        else {

                            music::downloader::download_audio(&audio_url, result.items[0].snippet.title.as_str(), tx.clone(), meta_data);
                            return ExecutedCommand::new(format!("download {}", name), Some(task));

                        }

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

fn assert_if_given(s: String) -> Option<String>  {

    Some(s).filter(|val| val.len() > 1 )

}
