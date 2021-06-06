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

impl std::ops::Neg for Position {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Position{x: -self.x, y: -self.y}
    }
}

impl Position {
    pub fn shift(&self, orientation: Orientation) -> Position {
        match orientation {
            Orientation::Up => Position{x: self.x, y: self.y+1},
            Orientation::Down => Position{x: self.x, y: self.y-1},
            Orientation::Left => Position{x: self.x-1, y: self.y},
            Orientation::Right => Position{x: self.x+1, y: self.y},
        }
    }

    pub fn reflect_horizontally(&self) -> Position {
        Position{x: -self.x, y: self.y}
    }

    pub fn rotate_cw(&self) -> Position {
        Position{x: self.y, y: -self.x}
    }

    pub fn translate(&self, translation: Position) -> Position {
        Position{x: self.x + translation.x, y: self.y + translation.y}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn reflect_horizontally() {
        assert_eq!(Position{x: 0, y: 0}.reflect_horizontally(), Position{x: 0, y: 0});
        assert_eq!(Position{x: 1, y: 1}.reflect_horizontally(), Position{x: -1, y: 1});
        assert_eq!(Position{x: 2, y: -3}.reflect_horizontally(), Position{x: -2, y: -3});
        assert_eq!(Position{x: 7, y: 4}.reflect_horizontally(), Position{x: -7, y: 4});
    }

    #[test]
    fn rotate_cw() {
        assert_eq!(Position{x: 4, y: 1}.rotate_cw(), Position{x: 1, y: -4});
        assert_eq!(Position{x: -1, y: 4}.rotate_cw(), Position{x: 4, y: 1});
        assert_eq!(Position{x: -4, y: -1}.rotate_cw(), Position{x: -1, y: 4});
        assert_eq!(Position{x: 1, y: -4}.rotate_cw(), Position{x: -4, y: -1});
    }
}