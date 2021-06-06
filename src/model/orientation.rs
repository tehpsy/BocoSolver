use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    pub fn opposite(&self) -> Orientation {
        match self {
            Orientation::Up => return Orientation::Down,
            Orientation::Down => return Orientation::Up,
            Orientation::Left => return Orientation::Right,
            Orientation::Right => return Orientation::Left,
        }
    }

    pub fn flip_horizontal(&self) -> Orientation {
        match self {
            Orientation::Up => return Orientation::Up,
            Orientation::Down => return Orientation::Down,
            Orientation::Left => return Orientation::Right,
            Orientation::Right => return Orientation::Left,
        }
    }

    pub fn rotate_cw_90_deg(&self) -> Orientation {
        match self {
            Orientation::Up => return Orientation::Right,
            Orientation::Down => return Orientation::Left,
            Orientation::Left => return Orientation::Up,
            Orientation::Right => return Orientation::Down,
        }
    }
}

impl fmt::Display for Orientation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({})", self)
  }
}
