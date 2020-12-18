use std::fmt;
use super::orientation::Orientation;
use super::color::Color;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit {
    pub orientation: Orientation,
    pub color: Color,
}

// impl Unit {
//     fn to_string(&self) -> String {
//         return self.orientation.to_string() + self.color.to_string();
//     }
// }

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.orientation, self.color)
    }
}