mod tile_set;
pub use tile_set::TileSet;
mod tile;
pub use tile::Tile;
mod block;
pub use block::SolidBlock;

use std::fmt::Display;
use std::process;

use olc::Key;
use olc_pixel_game_engine as olc;

const TILE_SIZE: i32 = 16;

pub struct Engine {
    synth: Option<tts::TTS>,
    tile_set: TileSet,
}

impl Engine {
    pub fn new() -> Self {
        let synth = match tts::TTS::default() {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Warning: Failed to initialise tts engine: {}", e);
                eprintln!("Warning: Speech output will be sent to stdout");
                None
            }
        };
        Self {
            synth,
            tile_set: TileSet::new((olc::screen_width() * olc::screen_height()) as usize),
        }
    }

    fn speak<S>(&mut self, text: S, interrupt: bool)
    where
        S: Display + Into<String>,
    {
        if let Some(synth) = &mut self.synth {
            match synth.speak(text, interrupt) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Warning: Failed to synthesise text: {}", e);
                }
            };
        } else {
            println!("{}", text);
        }
    }
}

impl olc::Application for Engine {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.speak("Welcome to Blockz!", true);
        Ok(())
    }
    fn on_user_update(&mut self, _f_elapsed_time: f32) -> Result<(), olc::Error> {
        if olc::get_key(Key::Q).pressed {
            process::exit(0);
        }

        self.tile_set.draw();
        Ok(())
    }
    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}
