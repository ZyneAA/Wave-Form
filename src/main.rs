use std::error::Error;

mod youtube;
mod music;
mod ui;
mod helper;
mod wave;

fn main() -> Result<(), Box<dyn Error>> {

    wave::start()

}

