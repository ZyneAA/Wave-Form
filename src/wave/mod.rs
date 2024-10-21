use std::error::Error;
use std::fmt;

pub mod config;
pub mod command;

use crate::ui::components;

#[derive(Debug)]
pub struct WaveStyle {

    pub border_color_0: [u8; 3],
    pub border_color_1: [u8; 3],
    pub border_color_2: [u8; 3],
    pub border_color_3: [u8; 3],
    pub color_0: [u8; 3],
    pub color_1: [u8; 3],
    pub color_2: [u8; 3],
    pub command_history_length: u64,

}

impl WaveStyle {

    pub fn new() -> Self {

        WaveStyle {
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

    pub fn new(how: String) -> Self {

        WaveErr {
            log: how,
        }

    }

}

pub fn start() -> Result<(), Box<dyn Error>> {

    let look = config::config_wave();
    components::render_main_view(look).unwrap();

    Ok(())

}
