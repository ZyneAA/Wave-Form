use reqwest::blocking::{ Client, get };
use serde::Deserialize;

use crate::wave::WaveErr;

#[derive(Deserialize, Debug)]
pub struct YoutubeItem {

    pub id: Id,
    pub snippet: Snippet,

}

#[derive(Deserialize, Debug)]
pub struct Id {

    #[serde(rename = "videoId")]
    pub video_id: String,

}

#[derive(Deserialize, Debug)]
pub struct Snippet {

    pub title: String,

    #[serde(rename = "channelTitle")]
    pub channel: String,

    #[serde(rename = "publishTime")]
    pub publish_time: String,


}

#[derive(Deserialize, Debug)]
pub struct YoutubeResponse<T> {

    pub items: Vec<T>

}

#[derive(Deserialize)]
pub struct DetailItem {

    #[serde(rename = "contentDetails")]
    pub content_details: ContentDetails,

}

#[derive(Deserialize)]
pub struct ContentDetails {

    pub duration: String,

}

pub fn find(querry: &str, api_key: &Option<String>, max_results: u64) -> Result<YoutubeResponse<YoutubeItem>, Box<dyn std::error::Error>> {

    match api_key {

        Some(key) => {

            let url = "https://www.googleapis.com/youtube/v3/search";
            let max_rows: String = max_results.to_string();

            let client = Client::new();
            let response = client
                .get(url)
                .query(&[
                    ("part", "snippet"),
                    ("q", querry),
                    ("key", key.as_str()),
                    ("type", "video"),
                    ("maxResults", &max_rows),
                ])
            .send()?;

            let bodie = response.text()?;
            let response: YoutubeResponse<YoutubeItem> = serde_json::from_str(&bodie)?;

            Ok(response)

        }
        None => {

            let err = WaveErr::new(String::from("No api key provided!"));
            Err(Box::new(err))

        }
    }

}

pub fn get_video_details(video_id: &str, api_key: &str) -> Result<YoutubeResponse<DetailItem>, Box<dyn std::error::Error>>{

    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=contentDetails&id={}&key={}",
        video_id, api_key
    );

    let response = get(&url)?.text()?;
    let response: YoutubeResponse<DetailItem> = serde_json::from_str(&response)?;

    Ok(response)
}

