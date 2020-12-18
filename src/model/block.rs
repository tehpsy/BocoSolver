use super::unit::Unit;
use super::neighbour_ids::NeighbourIds;

//use a vec of references to other 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Block {
    pub small: Option<Unit>,
    pub large: Option<Unit>,
    pub id: u8,
    pub neighbour_ids: NeighbourIds,
}
