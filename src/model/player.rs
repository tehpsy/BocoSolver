#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Player {
    pub block_id: u8,
}

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({})", self.block_id)
//     }
// }

