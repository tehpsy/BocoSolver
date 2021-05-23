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

    pub fn reflect_horizontally(&self, x: f64) -> Position {
        let x_float = 2_f64 * x - (self.x as f64);
        Position{x: x_float.round() as i8, y: self.y}
    }

    pub fn reflect_vertically(&self, y: f64) -> Position {
        let y_float = 2_f64 * y - (self.y as f64);
        Position{x: self.x, y: y_float.round() as i8}
    }

    pub fn rotate_cw(&self) -> Position {
        Position{x: -self.y, y: self.x}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn reflect_horizontally() {
        assert_eq!(Position{x: 0, y: 0}.reflect_horizontally(0_f64), Position{x: 0, y: 0});
        assert_eq!(Position{x: 1, y: 1}.reflect_horizontally(2_f64), Position{x: 3, y: 1});
        assert_eq!(Position{x: 2, y: -3}.reflect_horizontally(-1_f64), Position{x: -4, y: -3});
        assert_eq!(Position{x: 7, y: 4}.reflect_horizontally(3.5), Position{x: 0, y: 4});
    }

    #[test]
    fn reflect_vertically() {
        assert_eq!(Position{x: 0, y: 0}.reflect_vertically(0_f64), Position{x: 0, y: 0});
        assert_eq!(Position{x: 1, y: 1}.reflect_vertically(2_f64), Position{x: 1, y: 3});
        assert_eq!(Position{x: 2, y: -3}.reflect_vertically(-1_f64), Position{x: 2, y: 1});
        assert_eq!(Position{x: 7, y: 4}.reflect_vertically(3.5), Position{x: 7, y: 3});
    }

    #[test]
    fn rotate_cw() {
        assert_eq!(Position{x: 4, y: 1}.rotate_cw(), Position{x: -1, y: 4});
        assert_eq!(Position{x: -1, y: 4}.rotate_cw(), Position{x: -4, y: -1});
        assert_eq!(Position{x: -4, y: -1}.rotate_cw(), Position{x: 1, y: -4});
        assert_eq!(Position{x: 1, y: -4}.rotate_cw(), Position{x: 4, y: 1});
    }
}