use olc_pixel_game_engine as olc;

use super::{Tile, TILE_SIZE};

pub struct Player {}

impl Tile for Player {
    fn description(&self) -> &str {
        "Player"
    }
    fn draw_at(&self, pos: olc::Vi2d) {
        olc::fill_rect(pos.x, pos.y, TILE_SIZE, TILE_SIZE, olc::DARK_CYAN);
        olc::draw_string(pos.x + 4, pos.y + 4, "P", olc::WHITE);
    }
}
