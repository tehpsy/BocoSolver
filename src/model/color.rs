use enum_iterator::IntoEnumIterator;
use std::fmt;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
pub enum Color {
    Black,
    Red,
}

// impl Color {
//     fn to_string(&self) -> String {
//         match self {
//             Color::Black => return "black".to_owned(),
//             Color::Red => return "red".to_owned(),
//         }
//     }
// }

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({})", self)
  }
}