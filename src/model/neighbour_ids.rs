use super::orientation::Orientation;

//use builder pattern to init the values -- default trait
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NeighbourIds {
    pub up: Option<u8>,
    pub down: Option<u8>,
    pub left: Option<u8>,
    pub right: Option<u8>,
}

impl NeighbourIds {
    pub fn new(up: Option<u8>, down: Option<u8>, left: Option<u8>, right: Option<u8>) -> NeighbourIds {
        NeighbourIds{
            up,
            down,
            left,
            right,
        }
    }

    pub fn neighbour_towards(&self, orientation: &Orientation) -> Option<u8> {
        match orientation {
            Orientation::Up => return self.up,
            Orientation::Down => return self.down,
            Orientation::Left => return self.left,
            Orientation::Right => return self.right,
        }
    }
}