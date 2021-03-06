// Copyright 2020, Michael Connor Buchan and the Blockz contributors
// SPDX-License-Identifier: MIT

//! The main game engine module, containing everything necessary for the game to run except the
//! main function.

mod tile_set;
use tile_set::TileSet;
mod tile;
use tile::Tile;
mod block;
use block::{MovableBlock, SolidBlock};
mod player;
use player::Player;
mod direction;
use direction::Direction;

use std::fmt::Display;
use std::process;

use olc::Key;
use olc_pixel_game_engine as olc;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The size of a tile in pixels
const TILE_SIZE: i32 = 16;

/// Converts from a 2D tile cordinate to a 2D screen coordinate
fn world_to_screen(pos: &olc::Vi2d) -> olc::Vi2d {
    let mut new = pos.clone();
    new.x *= TILE_SIZE;
    new.y *= TILE_SIZE;
    new
}

/// The main Pixel Game Engine Application. Responsible for updating the display and controling
/// synthesised speech.
pub struct Engine {
    /// The game's speech synthesiser. Will be set to None if there is an error during construction.
    synth: Option<tts::TTS>,
    /// The game's play-field of Tiles
    tile_set: TileSet,
    /// The location of the cursor, which allows the player to inspect the play-field via speech
    cursor: olc::Vi2d,
    /// The width (in tiles) of the play-field
    width: i32,
    /// The height (in tiles) of the play-field
    height: i32,
}

impl Engine {
    /// Constructs an Engine struct, initialising all members correctly and outputting errors as
    /// appropriate
    pub fn new() -> Self {
        let synth = match tts::TTS::default() {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Warning: Failed to initialise tts engine: {}", e);
                eprintln!("Warning: Speech output will be sent to stdout");
                None
            }
        };
        let tile_set = TileSet::load(
            "################\
                #..............#\
                #..............#\
                #..............#\
                #..............#\
                #..............#\
                #......p.......#\
                #..............#\
                #......+.......#\
                #....+++++.....#\
                #......+.......#\
                #..............#\
                #..............#\
                #+............+#\
                ################",
        );
        let cursor = tile_set.player_pos.clone();

        Self {
            synth,
            tile_set,
            cursor,
            width: 0,
            height: 0,
        }
    }

    /// Speak the passed text. If interrupt is true, stops speaking the last message first. If
    /// there was an error during synth construction, output the message to stdout instead.
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
    /// Speak the tile under the cursor. If interrupt is true, stop speaking the last message
    /// first. If there was an error during synth construction, output the tile's description to
    /// stdout.
    fn speak_cursor_tile(&mut self, interrupt: bool) {
        if let Some(tile) = self.tile_set.get_tile(self.cursor) {
            self.speak(
                format!(
                    "{}, row {}, col {}",
                    tile.description(),
                    self.cursor.y + 1,
                    self.cursor.x + 1
                ),
                interrupt,
            );
        } else {
            self.speak(
                format!(
                    "Blank, row {}, col {}",
                    self.cursor.y + 1,
                    self.cursor.x + 1
                ),
                true,
            );
        }
    }
}

impl olc::Application for Engine {
    /// Called once by olc::Application when this struct is constructed. Calculates the width and height of
    /// the play-field, then speaks a welcome message and the description of the player Tile.
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.width = olc::screen_width() / TILE_SIZE;
        self.height = olc::screen_height() / TILE_SIZE;

        self.speak(format!("Welcome to {}!", APP_NAME), true);
        self.speak_cursor_tile(false);
        Ok(())
    }
    /// Called once per frame. Draws the play-field and handles user input.
    fn on_user_update(&mut self, _f_elapsed_time: f32) -> Result<(), olc::Error> {
        // USER INPUT //
        // Quit
        if olc::get_key(Key::Q).pressed {
            process::exit(0);
        }
        // Speak FPS
        if olc::get_key(Key::F).pressed {
            self.speak(format!("{} frames per second", olc::get_fps()), true);
        }
        // Speak game info
        if olc::get_key(Key::F1).pressed {
            self.speak(
                format!("{}, version {}. Created by Michael Connor Buchan, using the One Lone Coder Pixel Game Engine.", APP_NAME, APP_VERSION),
                true
                );
        }
        // Stop speech
        if olc::get_key(Key::S).pressed {
            if let Some(synth) = &mut self.synth {
                if let Err(e) = synth.stop() {
                    eprintln!("Warning: Failed to stop speech: {}", e);
                }
            }
        }

        // Move cursor
        if olc::get_key(Key::UP).pressed && self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::DOWN).pressed && self.cursor.y < self.height - 1 {
            self.cursor.y += 1;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::LEFT).pressed && self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::RIGHT).pressed && self.cursor.x < self.width - 1 {
            self.cursor.x += 1;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::HOME).pressed && self.cursor.x != 0 {
            self.cursor.x = 0;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::END).pressed && self.cursor.x != self.width - 1 {
            self.cursor.x = self.width - 1;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::PGUP).pressed && self.cursor.y != 0 {
            self.cursor.y = 0;
            self.speak_cursor_tile(true);
        }
        if olc::get_key(Key::PGDN).pressed && self.cursor.y != self.height - 1 {
            self.cursor.y = self.height - 1;
            self.speak_cursor_tile(true);
        }
        // Move cursor with the mouse
        if olc::get_mouse(0).pressed {
            let pos = olc::Vi2d::new(olc::get_mouse_x(), olc::get_mouse_y())
                / olc::Vi2d {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                };
            if self.cursor != pos {
                self.cursor = pos;
                self.speak_cursor_tile(true);
            }
        }

        // DRAWING THE SCREEN //
        self.tile_set.draw();
        let c = world_to_screen(&self.cursor);
        olc::draw_rect(c.x, c.y, TILE_SIZE, TILE_SIZE, olc::WHITE);
        Ok(())
    }
    /// Called once when the game is closed. Currently stops speech output (if any).
    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        if let Some(synth) = &mut self.synth {
            synth.stop().ok();
        }
        Ok(())
    }
}
