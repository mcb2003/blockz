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

fn world_to_screen(pos: &olc::Vi2d) -> olc::Vi2d {
    let mut new = pos.clone();
    new.x *= TILE_SIZE;
    new.y *= TILE_SIZE;
    new
}

pub struct Engine {
    synth: Option<tts::TTS>,
    tile_set: TileSet,
    cursor: olc::Vi2d,
    width: i32,
    height: i32,
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
            tile_set: TileSet::load("#################..............##..............##..............##..............##..............##..............##..............##..............##..............##..............##..............##..............##..............#################"),
            cursor: olc::Vi2d { x: 0, y: 0 },
            width: 0,
            height: 0,
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
fn speak_cursor_tile(&mut self) {
    if let Some(tile) = self.tile_set.get_tile(self.cursor) {
        self.speak(format!("{}, row {}, col {}", tile.description(), self.cursor.y+1, self.cursor.x+1), true);
    } else {
        self.speak(format!("Blank, row {}, col {}", self.cursor.y+1, self.cursor.x+1), true);
    }
    }
}


impl olc::Application for Engine {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.width = olc::screen_width() / TILE_SIZE;
        self.height = olc::screen_height() / TILE_SIZE;

        self.speak("Welcome to Blockz!", true);
        Ok(())
    }
    fn on_user_update(&mut self, _f_elapsed_time: f32) -> Result<(), olc::Error> {
        if olc::get_key(Key::Q).pressed {
            process::exit(0);
        }
        // Speak FPS
        if olc::get_key(Key::F).pressed {
            self.speak(format!("{} frames per second", olc::get_fps()), true);
        }
        // Move cursor
        if olc::get_key(Key::UP).pressed && self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::DOWN).pressed && self.cursor.y < self.height - 1 {
            self.cursor.y += 1;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::LEFT).pressed && self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::RIGHT).pressed && self.cursor.x < self.width - 1 {
            self.cursor.x += 1;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::HOME).pressed && self.cursor.x != 0 {
            self.cursor.x = 0;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::END).pressed && self.cursor.x != self.width - 1 {
            self.cursor.x = self.width - 1;
            self.speak_cursor_tile();
        }
        if olc::get_key(Key::PGUP).pressed && self.cursor.y != 0 {
            self.cursor.y = 0;
            self.speak_cursor_tile();
        }
            if olc::get_key(Key::PGDN).pressed && self.cursor.y != self.height - 1 {
            self.cursor.y = self.height - 1;
            self.speak_cursor_tile();
        }

        self.tile_set.draw();
        let c = world_to_screen(&self.cursor);
        olc::draw_rect(c.x, c.y, TILE_SIZE, TILE_SIZE, olc::CYAN);
        Ok(())
    }
    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}
