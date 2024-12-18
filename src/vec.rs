use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2i {
    pub fn up() -> Self {
        Self { x: 0, y: -1 }
    }

    pub fn down() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn right() -> Self {
        Self { x: 1, y: 0 }
    }

    pub fn left() -> Self {
        Self { x: -1, y: 0 }
    }
}
