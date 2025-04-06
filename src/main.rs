use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};
use reqwest;

use std::error::Error;

mod youtube;
mod music;
mod ui;
mod helper;
mod wave;

fn main() -> Result<(), Box<dyn Error>> {

    //let response  = youtube::video::find("Luther Kendrick Lamer", &api_key, 1).unwrap();
    //let url = music::info::get_music_url(&response.items[0].id.video_id).unwrap();

    //println!("{}", url);
    wave::start()


}

