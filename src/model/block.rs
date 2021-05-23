use super::unit::Unit;

//use a vec of references to other 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Block {
    pub small: Option<Unit>,
    pub large: Option<Unit>,
}
