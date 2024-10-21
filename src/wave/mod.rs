use std::error::Error;

pub mod config;

use crate::ui::components;

pub fn start() -> Result<(), Box<dyn Error>> {

    let look = config::config_wave();
    components::render_main_view(look).unwrap();

    Ok(())

}
