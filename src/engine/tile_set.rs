use olc_pixel_game_engine as olc;

const TILE_SIZE: i32 = 16;

pub struct TileSet {
    tiles: Vec<bool>,
}

impl TileSet {
    pub fn new(num_tiles: usize) -> Self {
        let mut tiles = Vec::with_capacity(num_tiles);
        for i in 0..num_tiles {
            tiles.push(i % 8 == 0);
        }
        Self { tiles }
    }

    pub fn draw(&self) {
        let width = olc::screen_width() / TILE_SIZE;
        let height = olc::screen_height() / TILE_SIZE;
        olc::clear(olc::BLACK);
        for y in 0..height {
            for x in 0..width {
                if self.tiles[(y * width + x) as usize] {
                    olc::fill_rect(
                        x * TILE_SIZE,
                        y * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        olc::WHITE,
                    );
                }
            }
        }
    }
}
