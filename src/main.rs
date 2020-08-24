use olc_pixel_game_engine as olc;

use blockz::engine::{self, Engine};

fn main() -> Result<(), olc::Error> {
    let mut app = Engine::new();
    match olc::start("Blockz", &mut app, 256, 240, 4, 4) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error starting application: {}", e);
            Err(e)
        }
    }
}
