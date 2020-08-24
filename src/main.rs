use std::process;

use olc::Key;
use olc_pixel_game_engine as olc;

struct App {
    x: f32,
    y: f32,
}

impl olc::Application for App {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
    fn on_user_update(&mut self, f_elapsed_time: f32) -> Result<(), olc::Error> {
        olc::clear(olc::BLACK);

        if olc::get_key(Key::DOWN).held {
            self.y += 150.0 * f_elapsed_time;
            self.y = self.y.min(olc::screen_height() as f32 - 16.0);
        }
        if olc::get_key(Key::UP).held {
            self.y -= 150.0 * f_elapsed_time;
            self.y = self.y.max(0.0);
        }
        if olc::get_key(Key::RIGHT).held {
            self.x += 150.0 * f_elapsed_time;
            self.x = self.x.min(olc::screen_width() as f32 - 16.0);
        }
        if olc::get_key(Key::LEFT).held {
            self.x -= 150.0 * f_elapsed_time;
            self.x = self.x.max(0.0);
        }
        if olc::get_key(Key::Q).pressed {
            process::exit(0);
        }

        olc::fill_rect(self.x as i32, self.y as i32, 16, 16, olc::WHITE);
        Ok(())
    }
    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

fn main() -> Result<(), olc::Error> {
    let mut app = App { x: 0.0, y: 0.0 };
    olc::start("Blockz", &mut app, 256, 240, 4, 4)?;
    Ok(())
}
