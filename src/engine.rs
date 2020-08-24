use std::process;

use olc::Key;
use olc_pixel_game_engine as olc;

pub struct Engine {
    synth: Option<tts::TTS>,
    x: f32,
    y: f32,
}

impl Engine {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            synth: tts::TTS::default().ok(),
            x,
            y,
        }
    }
}

impl olc::Application for Engine {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        if let Some(synth) = &mut self.synth {
            synth.speak("Welcome to Blockz!", true);
        }
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
