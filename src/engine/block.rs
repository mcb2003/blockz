//! Contains most of the block types

use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{Tile, TILE_SIZE};

/// A block that can't be pushed
pub struct SolidBlock {}
impl Tile for SolidBlock {
    fn description(&self) -> &str {
        "Solid block"
    }

    fn draw_at(&self, pos: Vi2d) {
        olc::fill_rect(pos.x, pos.y, TILE_SIZE, TILE_SIZE, olc::WHITE);
    }
}
