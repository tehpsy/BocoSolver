use std::fmt;

use super::Orientation;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn shift(&self, orientation: Orientation) -> Position {
        match orientation {
            Orientation::Up => Position{x: self.x, y: self.y-1},
            Orientation::Down => Position{x: self.x, y: self.y+1},
            Orientation::Left => Position{x: self.x-1, y: self.y},
            Orientation::Right => Position{x: self.x+1, y: self.y},
        }
    }
}