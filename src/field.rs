use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    Empty,
    Box,
    Robot,
    Wall,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Field::Empty => f.write_char('.'),
            Field::Box => f.write_char('O'),
            Field::Robot => f.write_char('@'),
            Field::Wall => f.write_char('#'),
        }
    }
}

impl TryFrom<char> for Field {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            i => Err(format!("invalid char {i}")),
        }
    }
}
