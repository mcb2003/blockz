use std::rc::Rc;

use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{Player, SolidBlock, Tile, TILE_SIZE};

pub struct TileSet {
    tiles: Vec<Option<Rc<dyn Tile>>>,
    width: i32,
    height: i32,
}

impl TileSet {
    pub fn load(level: &str) -> Self {
        let mut tiles: Vec<Option<Rc<dyn Tile>>> = Vec::with_capacity(level.len());
        for c in level.chars() {
            tiles.push(match c {
                '#' => Some(Rc::new(SolidBlock {})),
                'p' => Some(Rc::new(Player {})),
                _ => None,
            });
        }
        Self {
            tiles,
            width: olc::screen_width() / TILE_SIZE,
            height: olc::screen_height() / TILE_SIZE,
        }
    }

    pub fn draw(&self) {
        olc::clear(olc::BLACK);
        let mut pos = Vi2d { x: 0, y: 0 };
        while pos.y < self.height {
            while pos.x < self.width {
                let idx = (pos.y * self.width + pos.x) as usize;
                if let Some(tile) = &self.tiles[idx] {
                    tile.draw(pos);
                }
                pos.x += 1;
            }
            pos.x = 0;
            pos.y += 1;
        }
    }

    pub fn get_tile(&self, pos: Vi2d) -> Option<Rc<dyn Tile>> {
        self.tiles[(pos.y * self.width + pos.x) as usize].clone()
    }
}
