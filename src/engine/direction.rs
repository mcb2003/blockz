//! Contains the Direction enum to represent North, South, East and West

use olc_pixel_game_engine::Key;

/// North, South, East and West
#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Converts olc_pixel_game_engine::Key values to directions. Both WASD and the arrow keys will
    /// be converted. Any other keys return None.
    fn from_key(key: Key) -> Option<Self> {
        match key {
            Key::W | Key::UP => Some(Self::North),
            Key::A | Key::LEFT => Some(Self::West),
            Key::S | Key::DOWN => Some(Self::South),
            Key::D | Key::RIGHT => Some(Self::East),
            _ => None,
        }
    }

    /// Returns true if the Direction is East or West
    fn is_horizontal(&self) -> bool {
        (self.clone() as u8) % 2 == 1
    }

    /// Returns true if the Direction is North or South
    fn is_verticle(&self) -> bool {
        (self.clone() as u8) % 2 == 0
    }
}
