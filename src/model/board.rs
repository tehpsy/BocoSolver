use enum_iterator::IntoEnumIterator;
use super::{NeighbourIds, block::Block, position};
use super::color::Color;
use super::unit::Unit;
use super::orientation::Orientation;
use super::position::Position;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub player_pos: Position,
    pub blocks: HashMap<Position, Block>,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_pos.hash(state);
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
    pub fn neighbours(&self, position: Position) -> NeighbourIds {
      let up = position.shift(Orientation::Up);
      let down = position.shift(Orientation::Down);
      let left = position.shift(Orientation::Left);
      let right = position.shift(Orientation::Right);

      NeighbourIds{
        up: if self.blocks.contains_key(&up) { Some(up) } else { None },
        down: if self.blocks.contains_key(&down) { Some(down) } else { None },
        left: if self.blocks.contains_key(&left) { Some(left) } else { None },
        right: if self.blocks.contains_key(&right) { Some(right) } else { None },
      }
    }

    pub fn available_moves(&self) -> HashSet<Orientation> {
        let player_pos = self.player_pos;
        let block = self.blocks[&player_pos];
        let neighbours = self.neighbours(player_pos);
        let vector: Vec<Orientation> = Orientation::into_enum_iter()
        .filter(|orientation| {
            return neighbours.neighbour_towards(orientation) != None;
        })
        .filter(|orientation| {
            let neighbour_pos = neighbours.neighbour_towards(orientation).unwrap();
            let neighbour = self.blocks[&neighbour_pos];
            
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
        let player_pos = self.player_pos;
        let block = self.blocks[&player_pos];
        let neighbours = self.neighbours(player_pos);
        let neighbour_pos = neighbours.neighbour_towards(&orientation).unwrap();
        let mut blocks = self.blocks.clone();
        let mut neighbour_block = blocks.get_mut(&neighbour_pos).unwrap().clone();
        let mut player_block = blocks.get_mut(&player_pos).unwrap().clone();
        
        if player_block.small != None &&
            (player_block.small.unwrap().orientation != orientation || (player_block.large != None && (player_block.large.unwrap().orientation != orientation))) {
            neighbour_block.small = player_block.small;
            player_block.small = None;
        }

        if player_block.large != None && player_block.large.unwrap().orientation != orientation {
            neighbour_block.large = player_block.large;
            player_block.large = None;
        }
        
        blocks.insert(neighbour_pos, neighbour_block);
        blocks.insert(player_pos, player_block);
        
        let board = Board{
            player_pos: player_pos.shift(orientation),
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
        let block = self.blocks[&self.player_pos];
        return match (block.small, block.large) {
            (Some(small), Some(large)) => small.color == Color::Red && large.color == Color::Red,
            _ => false,
        };
    }

    // pub fn flip_horizontal(&self) -> Board {
    //     let mut new_board = self.clone();
    //     for (id, block) in new_board.blocks.iter_mut() {
    //         let left_neighbour = block.neighbour_ids.left;
    //         let right_neighbour = block.neighbour_ids.right;
    //         block.neighbour_ids.left = right_neighbour;
    //         block.neighbour_ids.right = left_neighbour;

    //         match block.small {
    //             Some(unit) => {
    //                 if unit.orientation.is_horizontal() {
    //                     let new_orientation = unit.orientation.opposite();
    //                     block.small = Some(Unit{orientation: new_orientation, color: block.small.unwrap().color});
    //                 }
    //             },
    //             _ => {},
    //         }

    //         match block.large {
    //             Some(unit) => {
    //                 if unit.orientation.is_horizontal() {
    //                     let new_orientation = unit.orientation.opposite();
    //                     block.large = Some(Unit{orientation: new_orientation, color: block.large.unwrap().color});
    //                 }
    //             },
    //             _ => {},
    //         }
    //     }

    //     new_board
    // }

    // pub fn rotate_cw_90_deg(&self) -> Board {
    //     let mut new_board = self.clone();
    //     for (id, block) in new_board.blocks.iter_mut() {
    //         let up_neighbour = block.neighbour_ids.up;
    //         let down_neighbour = block.neighbour_ids.down;
    //         let left_neighbour = block.neighbour_ids.left;
    //         let right_neighbour = block.neighbour_ids.right;
    //         block.neighbour_ids.up = left_neighbour;
    //         block.neighbour_ids.down = right_neighbour;
    //         block.neighbour_ids.left = down_neighbour;
    //         block.neighbour_ids.right = up_neighbour;

    //         match block.small {
    //             Some(unit) => {
    //                 let new_orientation = unit.orientation.rotate_cw_90_deg();
    //                 block.small = Some(Unit{orientation: new_orientation, color: block.small.unwrap().color});
    //             },
    //             _ => {},
    //         }

    //         match block.large {
    //             Some(unit) => {
    //                 let new_orientation = unit.orientation.rotate_cw_90_deg();
    //                 block.large = Some(Unit{orientation: new_orientation, color: block.large.unwrap().color});
    //             },
    //             _ => {},
    //         }
    //     }

    //     new_board
    // }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::model::*;
//     use crate::hasher::calculate_hash;
//     use maplit::hashmap;

//     #[test]
//     fn flips_board_horizontally() {
//         let original_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
//               },
//             }
//         };
//         let flipped_board = original_board.flip_horizontal();
//         let expected_flipped_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(2), Some(1), None)
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(0))
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, Some(3), None)
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(1), None, None, Some(2))
//               },
//             }
//         };

//         assert_eq!(flipped_board, expected_flipped_board);
//     }

//     #[test]
//     fn rotates_board_cw_90_deg() {
//         let original_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
//               },
//             }
//         };
//         let rotated_board = original_board.rotate_cw_90_deg();
//         let expected_rotated_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(1), Some(2), None)
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, Some(3), None)
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(0))
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(2), None, None, Some(1))
//               },
//             }
//         };

//         assert_eq!(rotated_board, expected_rotated_board);
//     }

//     #[test]
//     fn multiple_rotations() {
//         let original_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
//               },
//             }
//         };
//         let board1 = &original_board;
//         let board2 = &board1.rotate_cw_90_deg();
//         let board3 = &board2.rotate_cw_90_deg();
//         let board4 = &board3.rotate_cw_90_deg();
//         let board5 = &board4.rotate_cw_90_deg();

//         assert_eq!(original_board, *board5);
//         assert_eq!(
//             calculate_hash(&original_board), 
//             calculate_hash(board5)
//         );
//     }

//     #[test]
//     fn multiple_flips() {
//         let original_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//               0 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                 id: 0,
//                 neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
//               },
//               1 => Block{
//                 small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
//                 large: None,
//                 id: 1,
//                 neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
//               },
//               2 => Block{
//                 small: None,
//                 large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
//                 id: 2,
//                 neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
//               },
//               3 => Block{
//                 small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
//                 large: None,
//                 id: 3,
//                 neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
//               },
//             }
//         };
//         let board1 = &original_board;
//         let board2 = &board1.flip_horizontal();
//         let board3 = &board2.flip_horizontal();

//         assert_eq!(original_board, *board3);
//         assert_eq!(
//             calculate_hash(&original_board), 
//             calculate_hash(board3)
//         );
//     }
// }