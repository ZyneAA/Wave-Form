use std::env;
use std::path::Path;

use dotenv::from_path;

use crate::helper;

pub struct WaveStyle {

    pub border_color_0: [u8; 3],
    pub border_color_1: [u8; 3],
    pub border_color_2: [u8; 3],
    pub border_color_3: [u8; 3],
    pub color_0: [u8; 3],
    pub color_1: [u8; 3],
    pub color_2: [u8; 3],

}

impl WaveStyle {

    fn new() -> Self {

        WaveStyle{
            border_color_0: [255 as u8, 255 as u8, 255 as u8],
            border_color_1: [255 as u8, 255 as u8, 255 as u8],
            border_color_2: [255 as u8, 255 as u8, 255 as u8],
            border_color_3: [255 as u8, 255 as u8, 255 as u8],
            color_0: [255 as u8, 255 as u8, 255 as u8],
            color_1: [255 as u8, 255 as u8, 255 as u8],
            color_2: [255 as u8, 255 as u8, 255 as u8],
        }

    }

}

pub fn config_wave() -> WaveStyle {

    from_path(Path::new("./.env")).ok();

    let mut found = false;

    let mut wave = WaveStyle::new();

    for (k, v) in env::vars() {

        if k == "WAVE_FORM" {
            found = true;
        }

        if found {

            match k.as_str() {

                "BORDER_COLOR_0" => wave.border_color_0 = helper::rgb_converter(&v),
                "BORDER_COLOR_1" => wave.border_color_1 = helper::rgb_converter(&v),
                "BORDER_COLOR_2" => wave.border_color_2 = helper::rgb_converter(&v),
                "BORDER_COLOR_3" => wave.border_color_3 = helper::rgb_converter(&v),
                "COLOR_0" => wave.border_color_3 = helper::rgb_converter(&v),
                "COLOR_1" => wave.border_color_3 = helper::rgb_converter(&v),
                "COLOR_2" => wave.border_color_3 = helper::rgb_converter(&v),
                _ => {}

            }

        }

    }

    wave

}
