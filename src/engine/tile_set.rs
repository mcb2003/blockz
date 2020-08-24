use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{SolidBlock, Tile, TILE_SIZE};

pub struct TileSet {
    tiles: Vec<Option<Box<dyn Tile>>>,
}

impl TileSet {
    pub fn new(num_tiles: usize) -> Self {
        let mut tiles: Vec<Option<Box<dyn Tile>>> = Vec::with_capacity(num_tiles);
        for i in 0..num_tiles {
            tiles.push(if i % 8 == 0 {
                Some(Box::new(SolidBlock {}))
            } else {
                None
            });
        }
        Self { tiles }
    }

    pub fn draw(&self) {
        let width = olc::screen_width() / TILE_SIZE;
        let height = olc::screen_height() / TILE_SIZE;
        olc::clear(olc::BLACK);
        let mut pos = Vi2d { x: 0, y: 0 };
        while pos.y < height {
            while pos.x < width {
                if let Some(tile) = &self.tiles[(pos.y * width + pos.x) as usize] {
                    tile.draw(pos);
                }
                pos.x += 1;
            }
            pos.x = 0;
            pos.y += 1;
        }
    }
}
