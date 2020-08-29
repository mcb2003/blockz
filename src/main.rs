use olc_pixel_game_engine as olc;

use blockz::engine::{Engine, APP_NAME, APP_VERSION};

fn main() -> Result<(), olc::Error> {
    println!("{}, version {}.", APP_NAME, APP_VERSION);
    println!("Made with the One Lone Coder Pixel Game Engine: Copyright 2018, 2019, 2020 OneLoneCoder.com.");
    println!("https://github.com/OneLoneCoder/olcPixelGameEngine");
    let mut app = Engine::new();
    let title = format!("{} v{}", APP_NAME, APP_VERSION);
    match olc::start(title.as_str(), &mut app, 256, 240, 4, 4) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error starting application: {}", e);
            Err(e)
        }
    }
}
