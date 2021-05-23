use super::unit::Unit;

//use a vec of references to other 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Block {
    pub small: Option<Unit>,
    pub large: Option<Unit>,
}

impl Block {
    pub fn flip_horizontal(&self) -> Block {
        let small: Option<Unit>;
        let large: Option<Unit>;
        if self.small != None { small = Some(self.small.unwrap().flip_horizontal()); } else { small = None; }
        if self.large != None { large = Some(self.large.unwrap().flip_horizontal()); } else { large = None; }
        Block{
            small: small,
            large: large,
        }
    }

    pub fn flip_vertical(&self) -> Block {
        let small: Option<Unit>;
        let large: Option<Unit>;
        if self.small != None { small = Some(self.small.unwrap().flip_vertical()); } else { small = None; }
        if self.large != None { large = Some(self.large.unwrap().flip_vertical()); } else { large = None; }
        Block{
            small: small,
            large: large,
        }
    }

    pub fn rotate_cw(&self) -> Block {
        let small: Option<Unit>;
        let large: Option<Unit>;
        if self.small != None { small = Some(self.small.unwrap().rotate_cw()); } else { small = None; }
        if self.large != None { large = Some(self.large.unwrap().rotate_cw()); } else { large = None; }
        Block{
            small: small,
            large: large,
        }
    }
}