use olc_pixel_game_engine::Vi2d;

use super::TILE_SIZE;

pub trait Tile {
    fn description(&self) -> &str;
    fn draw_at(&self, pos: Vi2d);
    fn draw(&self, mut pos: Vi2d) {
        pos.x *= TILE_SIZE;
        pos.y *= TILE_SIZE;
        self.draw_at(pos);
    }
}
