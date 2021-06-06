use super::orientation::Orientation;
use super::position::Position;

//use builder pattern to init the values -- default trait
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NeighbourIds {
    pub up: Option<Position>,
    pub down: Option<Position>,
    pub left: Option<Position>,
    pub right: Option<Position>,
}

impl NeighbourIds {
    pub fn neighbour_towards(&self, orientation: &Orientation) -> Option<Position> {
        match orientation {
            Orientation::Up => return self.up,
            Orientation::Down => return self.down,
            Orientation::Left => return self.left,
            Orientation::Right => return self.right,
        }
    }
}