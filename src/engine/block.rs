//! Contains most of the block types

use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{Direction, Tile, TILE_SIZE};

/// A block that can't be pushed
pub struct SolidBlock {}
impl Tile for SolidBlock {
    fn description(&self) -> &str {
        "Solid block"
    }

    fn draw_at(&self, pos: Vi2d) {
        olc::fill_rect(pos.x, pos.y, TILE_SIZE, TILE_SIZE, olc::GREY);
    }

    fn is_pushable(&self, _dir: Direction) -> bool {
        false
    }
}

/// A block that can move in any direction
pub struct MovableBlock {}
impl Tile for MovableBlock {
    fn description(&self) -> &str {
        "Movable block"
    }

    fn draw_at(&self, pos: Vi2d) {
        olc::fill_rect(pos.x, pos.y, TILE_SIZE, TILE_SIZE, olc::DARK_BLUE);
        olc::fill_rect(pos.x + 6, pos.y, 4, TILE_SIZE, olc::YELLOW);
        olc::fill_rect(pos.x, pos.y + 6, TILE_SIZE, 4, olc::YELLOW);
    }

    fn is_pushable(&self, _dir: Direction) -> bool {
        true
    }
}
