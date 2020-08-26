//! Module containing the TileSet struct, which represents a game level, as well as the game's
//! state.

use olc_pixel_game_engine::Vi2d;

use super::TILE_SIZE;

/// Common interface for tiles within the tile-set
pub trait Tile {
    /// Returns a speakable description of the tile
    fn description(&self) -> &str;
    // Draws the tile at the specified position in screen space.
    fn draw_at(&self, pos: Vi2d);
    /// Draws the tile at the specified position in tile space. The default implementation converts
    /// the tile-space coordinate to screen-space and calls the draw_at() function.
    fn draw(&self, mut pos: Vi2d) {
        pos.x *= TILE_SIZE;
        pos.y *= TILE_SIZE;
        self.draw_at(pos);
    }
}
