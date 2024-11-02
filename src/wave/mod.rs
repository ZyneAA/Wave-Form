use std::error::Error;
use std::fmt;

use rodio::{ OutputStream, Sink };

pub mod config;
pub mod command;
pub mod test;

use crate::ui::components;

#[derive(Debug)]
pub struct WaveSettings {

    pub api_key: Option<String>,
    pub border_color_0: [u8; 3],
    pub border_color_1: [u8; 3],
    pub border_color_2: [u8; 3],
    pub border_color_3: [u8; 3],
    pub color_0: [u8; 3],
    pub color_1: [u8; 3],
    pub color_2: [u8; 3],
    pub command_history_length: u64,

}

impl WaveSettings {

    pub fn new() -> Self {

        WaveSettings {
            api_key: None,
            border_color_0: [255 as u8, 255 as u8, 255 as u8],
            border_color_1: [255 as u8, 255 as u8, 255 as u8],
            border_color_2: [255 as u8, 255 as u8, 255 as u8],
            border_color_3: [255 as u8, 255 as u8, 255 as u8],
            color_0: [255 as u8, 255 as u8, 255 as u8],
            color_1: [255 as u8, 255 as u8, 255 as u8],
            color_2: [255 as u8, 255 as u8, 255 as u8],
            command_history_length: 10,
        }

    }

}

#[derive(Debug)]
pub struct WaveErr {

    pub log: String,

}

impl fmt::Display for WaveErr {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wave Error: {}", self.log)
    }

}

impl Error for WaveErr {

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

}

impl WaveErr {

    pub fn new(log: String) -> Self {

        WaveErr {
            log
        }

    }

}

pub fn start() -> Result<(), Box<dyn Error>> {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let look = config::config_wave().unwrap();
    components::render_app(look, sink).unwrap();
    //test::simulate_audio_wave().unwrap();

    Ok(())
}

