use std::fmt::Display;

use crate::vec::Vec2i;

#[derive(Debug)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Movement::Up => f.write_str("Up"),
            Movement::Down => f.write_str("Down"),
            Movement::Left => f.write_str("Left"),
            Movement::Right => f.write_str("Right"),
        }
    }
}

impl Into<Vec2i> for &Movement {
    fn into(self) -> Vec2i {
        match self {
            Movement::Up => Vec2i::up(),
            Movement::Down => Vec2i::down(),
            Movement::Left => Vec2i::left(),
            Movement::Right => Vec2i::right(),
        }
    }
}

impl TryFrom<char> for Movement {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            i => Err(format!("invalid movement direction {i}")),
        }
    }
}
