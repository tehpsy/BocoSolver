use std::fmt;
use super::orientation::Orientation;
use super::color::Color;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit {
    pub orientation: Orientation,
    pub color: Color,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.orientation, self.color)
    }
}

impl Unit {
    pub fn flip_horizontal(&self) -> Unit {
        Unit{
            orientation: self.orientation.flip_horizontal(),
            color: self.color
        }
    }

    pub fn rotate_cw(&self) -> Unit {
        Unit{
            orientation: self.orientation.rotate_cw_90_deg(),
            color: self.color
        }
    }
}