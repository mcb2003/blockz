use olc_pixel_game_engine as olc;

use blockz::engine::{self, Engine};

fn main() -> Result<(), olc::Error> {
    let mut app = Engine::new(0.0, 0.0);
    olc::start("Blockz", &mut app, 256, 240, 4, 4)?;
    Ok(())
}
