use std::{ fs, process::Command, error::Error };

use tui::{
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },

};
pub fn get_music_url(video_id: &str) -> Result<String, Box<dyn Error>> {

    let output = Command::new("yt-dlp")
        .args(&["-g", "-f", "bestaudio", video_id])
        .output()?;

    if output.status.success() {
        let url = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(url)
    }
    else {
        Err("Failed to download audio".into())
    }

}

pub fn get_local_songs<'a>(color_0: u8, color_1: u8, color_2: u8, selected_color_0: u8, selected_color_1: u8, selected_color_2: u8, target: usize) -> Result<Vec<Spans<'a>>, Box<dyn Error>> {

    let folder_path = "./songs";
    let mut songs = Vec::new();
    let mut a: usize = 0;

    for i in fs::read_dir(folder_path)? {

        let i = i?;
        let path = i.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "mp3" {
                    if let Some(path_str) = path.to_str() {
                        if target == a {
                            songs.push(Spans::from(Span::styled(String::from(&path_str[8..]), Style::default().
                                fg(Color::Rgb(color_0, color_1, color_2))
                                .bg(Color::Rgb(selected_color_0, selected_color_1, selected_color_2)).add_modifier(Modifier::BOLD))));
                        }
                        else {
                            songs.push(Spans::from(Span::styled(String::from(&path_str[8..]), Style::default().
                                fg(Color::Rgb(color_0, color_1, color_2)).add_modifier(Modifier::BOLD))));
                        }
                    }
                    a += 1;
                }
            }
        }

    }

    Ok(songs)

}
