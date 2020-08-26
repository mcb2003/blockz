use olc_pixel_game_engine::Key;

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_key(key: Key) -> Option<Self> {
        match key {
            Key::W | Key::UP => Some(Self::North),
            Key::A | Key::LEFT => Some(Self::West),
            Key::S | Key::DOWN => Some(Self::South),
            Key::D | Key::RIGHT => Some(Self::East),
            _ => None,
        }
    }

    fn is_horizontal(&self) -> bool {
        (self.clone() as u8) % 2 == 1
    }

    fn is_verticle(&self) -> bool {
        (self.clone() as u8) % 2 == 0
    }
}
