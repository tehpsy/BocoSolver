use enum_iterator::IntoEnumIterator;
use super::player::Player;
use super::block::Block;
use super::color::Color;
use super::orientation::Orientation;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub player: Player,
    pub blocks: HashMap<u8, Block>,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.blocks.iter().for_each(|(_, value)| {
            value.hash(state);
        });
    }
}

// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.player, self.blocks)
//     }
// }

impl Board {
    pub fn available_moves(&self) -> HashSet<Orientation> {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let vector: Vec<Orientation> = Orientation::into_enum_iter()
        .filter(|orientation| {
            return neighbour_ids.neighbour_towards(orientation) != None;
        })
        .filter(|orientation| {
            let neighbour_id = neighbour_ids.neighbour_towards(orientation).unwrap();
            let neighbour = self.blocks[&neighbour_id];
            
            if (neighbour.small != None && neighbour.small.unwrap().orientation.opposite() != *orientation) ||
               (neighbour.large != None && neighbour.large.unwrap().orientation.opposite() != *orientation) { 
                return false
            } else {
                return match (neighbour.small, neighbour.large) {
                    (None, Some(_)) => 
                        (block.large == None || block.large.unwrap().orientation == *orientation),
                    (Some(_), None) => 
                        (block.small == None || block.small.unwrap().orientation == *orientation) &&
                        (block.large == None || block.large.unwrap().orientation == *orientation),
                    (Some(_), Some(_)) => block.small == None && block.large == None,
                    (None, None) => true,
                };
            }

            /*
            can't move if
            dest large or small is not in opposite orientation

            if dest has nothing, then we can move, dragging everything 
            if dest has only small, then we can only move if we won't drag anything
            if dest has only large, then we can only move if source has no large or wouldn't be dragged
            if dest has both, then we can only move if source has neither
            */

//             //there's no dest.large
//             //or dest.large orientation is opposite orientation
//             let neighbour_id = neighbour_ids.neighbour_towards(orientation).unwrap();
//             let neighbour = self.blocks[&neighbour_id];
//             let small_unit_ok = neighbour.small == None || 
//                 (neighbour.small.unwrap().orientation.opposite() == *orientation && (block.small == None || block.small.unwrap().orientation == *orientation));
//             let large_unit_ok = 
//                 neighbour.large == None ||
//                 (neighbour.large.unwrap().orientation.opposite() == *orientation && (block.large == None || block.large.unwrap().orientation == *orientation));
            
//             return small_unit_ok && large_unit_ok;
        })
        .collect();

        return HashSet::from_iter(vector);
    }

    //TODO combine available_moves and moving into a single move() -> Board? method

    pub fn moving(&self, orientation: Orientation) -> Board {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let neighbour_id = neighbour_ids.neighbour_towards(&orientation).unwrap();
        let mut blocks = self.blocks.clone();
        let mut neighbour_block = blocks.get_mut(&neighbour_id).unwrap().clone();
        let mut player_block = blocks.get_mut(&player_block_id).unwrap().clone();
        
        if player_block.small != None &&
            (player_block.small.unwrap().orientation != orientation || (player_block.large != None && (player_block.large.unwrap().orientation != orientation))) {
            neighbour_block.small = player_block.small;
            player_block.small = None;
        }

        if player_block.large != None && player_block.large.unwrap().orientation != orientation {
            neighbour_block.large = player_block.large;
            player_block.large = None;
        }
        
        blocks.insert(neighbour_id, neighbour_block);
        blocks.insert(player_block_id, player_block);
        
        let board = Board{
            player: Player{block_id: neighbour_id},
            blocks: blocks
        };
        return board;
    }

    pub fn next_boards(&self) -> Vec<Board> {
        return self.available_moves()
            .into_iter()
            .map(|orientation| { return self.moving(orientation); })
            .collect();
    }

    pub fn is_win(&self) -> bool {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        return match (block.small, block.large) {
            (Some(small), Some(large)) => small.color == Color::Red && large.color == Color::Red,
            _ => false,
        };
    }
}