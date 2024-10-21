use std::env;
use std::path::Path;

use dotenv::from_path;

use crate::helper;
use crate::wave::WaveStyle;

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
                "COLOR_0" => wave.color_0 = helper::rgb_converter(&v),
                "COLOR_1" => wave.color_1 = helper::rgb_converter(&v),
                "COLOR_2" => wave.color_2 = helper::rgb_converter(&v),
                "COMMAND_HISTORY_LENGTH" => wave.command_history_length = v.parse().unwrap(),
                _ => {}

            }

        }

    }

    println!("{:?}", &wave);
    wave

}
