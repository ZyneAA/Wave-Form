use std::process::Command;

pub fn get_music_url(video_id: &str) -> Result<String, Box<dyn std::error::Error>> {

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
