use olc_pixel_game_engine as olc;
use olc_pixel_game_engine::Vi2d;

use super::TILE_SIZE;

pub trait Tile {
    fn draw(&self, pos: Vi2d);
}
