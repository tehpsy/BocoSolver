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

    // fn to_string(&self) -> String {
    //     match self {
    //         Orientation::Up => return "up".to_owned(),
    //         Orientation::Down => return "down".to_owned(),
    //         Orientation::Left => return "left".to_owned(),
    //         Orientation::Right => return "right".to_owned(),
    //     }
    // }
}

impl fmt::Display for Orientation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({})", self)
  }
}
