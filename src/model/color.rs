use enum_iterator::IntoEnumIterator;
use std::fmt;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
pub enum Color {
    Black,
    Red,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({})", self)
  }
}