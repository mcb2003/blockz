use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{Tile, TILE_SIZE};

pub struct SolidBlock {}
impl Tile for SolidBlock {
    fn draw_at(&self, pos: Vi2d) {
        olc::fill_rect(pos.x, pos.y, TILE_SIZE, TILE_SIZE, olc::WHITE);
    }
}
