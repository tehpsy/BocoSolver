use enum_iterator::IntoEnumIterator;
use super::{NeighbourIds, block::Block, position};
use super::color::Color;
use super::unit::Unit;
use super::orientation::Orientation;
use super::position::Position;
use std::{borrow::Borrow, collections::HashMap, hash::Hash};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::{Hasher};
use maplit::hashmap;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub player_pos: Position,
    pub blocks: HashMap<Position, Block>,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
      let (translation, _) = self.get_size();

      self.player_pos.translate(-translation).hash(state);

        for key in self.blocks.keys().sorted() {
            key.translate(-translation).hash(state);
            self.blocks[key].hash(state);
        }
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

    pub fn get_size(&self) -> (Position, Position) {
      let min_x = (*(self.blocks.keys().min_by(|pos1, pos2| pos1.x.cmp(&pos2.x)).unwrap())).x;
      let max_x = (*(self.blocks.keys().max_by(|pos1, pos2| pos1.x.cmp(&pos2.x)).unwrap())).x;
      let min_y = (*(self.blocks.keys().min_by(|pos1, pos2| pos1.y.cmp(&pos2.y)).unwrap())).y;
      let max_y = (*(self.blocks.keys().max_by(|pos1, pos2| pos1.y.cmp(&pos2.y)).unwrap())).y;

      (Position{x: min_x, y: min_y}, Position{x: max_x, y: max_y})
    }

    pub fn flip_horizontal(&self) -> Board {
        let mut new_board = Board{
          player_pos: self.player_pos.reflect_horizontally(), 
          blocks: hashmap!{}
        };

        for (position, block) in self.blocks.iter() {
          new_board.blocks.insert(
            position.reflect_horizontally(),
            block.flip_horizontal()
          );
        }

        new_board
    }

    pub fn flip_vertical(&self) -> Board {
      let mut new_board = Board{
        player_pos: self.player_pos.reflect_vertically(), 
        blocks: hashmap!{}
      };

      for (position, block) in self.blocks.iter() {
        new_board.blocks.insert(
          position.reflect_vertically(),
          block.flip_vertical()
        );
      }

      new_board
  }

    pub fn rotate_cw_90_deg(&self) -> Board {
      let mut new_board = Board{
        player_pos: self.player_pos.rotate_cw(), 
        blocks: hashmap!{}
      };

      for (position, block) in self.blocks.iter() {
        new_board.blocks.insert(
          position.rotate_cw(),
          block.rotate_cw()
        );
      }

      new_board
    }

    pub fn translate(&self, translation: Position) -> Board {
      let mut new_board = Board{
        player_pos: self.player_pos.translate(translation),
        blocks: hashmap!{}
      };

      for (position, block) in self.blocks.iter() {
        new_board.blocks.insert(
          position.clone().translate(translation),
          block.clone()
        );
      }

      new_board
    }

    pub fn compare(&self, other: &Board) -> Option<Orientation> {
      if (self.player_pos.x == other.player_pos.x && self.player_pos.y == other.player_pos.y) {
        return None; 
      }

      let mut orientation: Option<Orientation> = None;

      if (self.player_pos.x == other.player_pos.x) {
        if (self.player_pos.y + 1 == other.player_pos.y) {
          orientation = Some(Orientation::Up);
        } 
        if (self.player_pos.y - 1 == other.player_pos.y) {
          orientation = Some(Orientation::Down);
        } 
      }

      if (self.player_pos.y == other.player_pos.y) {
        if (self.player_pos.x + 1 == other.player_pos.x) {
          orientation = Some(Orientation::Right);
        } 
        if (self.player_pos.x - 1 == other.player_pos.x) {
          orientation = Some(Orientation::Left);
        } 
      }

      if let Some(orient) = orientation {
        let available_moves = self.available_moves();
        if (available_moves.contains(&orient)) {
          return Some(orient);
        }
      }

      return None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::model::*;
    use crate::hasher::calculate_hash;
    use crate::utils;

    #[test]
    fn flips_board_horizontally() {
        let original_board = Board{
            player_pos: Position{x: 7, y: 2},
            blocks: hashmap!{
              Position{x: 5, y: 2} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
              },
              Position{x: 6, y: 2} => Block{
                small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
                large: None,
              },
              Position{x: 7, y: 2} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
              },
              Position{x: 7, y: 3} => Block{
                small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
                large: None,
              },
              Position{x: 7, y: 4} => Block{
                small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
                large: None,
              },
            }
        };
        let flipped_board = original_board.flip_horizontal();
        let expected_flipped_board = Board{
          player_pos: Position{x: -7, y: 2},
          blocks: hashmap!{
            Position{x: -5, y: 2} => Block{
              small: None,
              large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
            },
            Position{x: -6, y: 2} => Block{
              small: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
              large: None,
            },
            Position{x: -7, y: 2} => Block{
              small: None,
              large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
            },
            Position{x: -7, y: 3} => Block{
              small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
              large: None,
            },
            Position{x: -7, y: 4} => Block{
              small: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
              large: None,
            },
          }
        };

        assert_eq!(flipped_board, expected_flipped_board);
    }

    #[test]
    fn rotates_board_cw_90_deg() {
        let original_board = Board{
            player_pos: Position{x: 2, y: 1},
            blocks: hashmap!{
              Position{x: 2, y: 1} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
              },
              Position{x: 3, y: 1} => Block{
                small: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
                large: None,
              },
              Position{x: 4, y: 1} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Down, color: Color::Black}),
              },
              Position{x: 4, y: 2} => Block{
                small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
                large: None,
              },
            }
        };
        let expected_rotated_board = Board{
          player_pos: Position{x: 1, y: -2},
          blocks: hashmap!{
            Position{x: 1, y: -2} => Block{
              small: None,
              large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
            },
            Position{x: 1, y: -3} => Block{
              small: Some(Unit{orientation: Orientation::Down, color: Color::Black}),
              large: None,
            },
            Position{x: 1, y: -4} => Block{
              small: None,
              large: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
            },
            Position{x: 2, y: -4} => Block{
              small: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
              large: None,
            },
          }
      };

      assert_eq!(
        original_board.rotate_cw_90_deg(),
        expected_rotated_board
      );
    }

    #[test]
    fn multiple_rotations() {
        let board1 = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
              Position{x: 0, y: 0} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
              },
              Position{x: 1, y: 0} => Block{
                small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
                large: None,
              },
              Position{x: 0, y: 1} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
              },
              Position{x: 1, y: 1} => Block{
                small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
                large: None,
              },
            }
        };
        let board2 = board1.clone().rotate_cw_90_deg();
        let board3 = board2.clone().rotate_cw_90_deg();
        let board4 = board3.clone().rotate_cw_90_deg();
        let board5 = board4.clone().rotate_cw_90_deg();

        assert_eq!(board1, board5);
        utils::print(&board1);
        utils::print(&board5);
        assert_eq!(
            calculate_hash(&board1), 
            calculate_hash(&board5)
        );
    }

    #[test]
    fn multiple_flips() {
        let board1 = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
              Position{x: 0, y: 0} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
              },
              Position{x: 1, y: 0} => Block{
                small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
                large: None,
              },
              Position{x: 0, y: 1} => Block{
                small: None,
                large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
              },
              Position{x: 1, y: 1} => Block{
                small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
                large: None,
              },
            }
        };
        let board2 = board1.clone().flip_horizontal();
        let board3 = board2.clone().flip_horizontal();

        utils::print(&board1);
        utils::print(&board3);
        assert_eq!(board1, board3);
        assert_eq!(
            calculate_hash(&board1), 
            calculate_hash(&board3)
        );
    }

    #[test]
    fn compare_two_boards_with_non_adjacent_players() {
      let blocks = hashmap!{
        Position{x: 0, y: 0} => Block{ small: None, large: None },
        Position{x: -1, y: 0} => Block{ small: None, large: None },
        Position{x: -2, y: 0} => Block{ small: None, large: None },
        Position{x: 1, y: 0} => Block{ small: None, large: None },
        Position{x: 2, y: 0} => Block{ small: None, large: None },
        Position{x: 0, y: 1} => Block{ small: None, large: None },
        Position{x: 0, y: 2} => Block{ small: None, large: None },
        Position{x: 0, y: -1} => Block{ small: None, large: None },
        Position{x: 0, y: -2} => Block{ small: None, large: None },
      }; 

      let board1 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};
      let board2 = Board{player_pos: Position{x: 2, y: 0}, blocks: blocks.clone()};
      let board3 = Board{player_pos: Position{x: -2, y: 0}, blocks: blocks.clone()};
      let board4 = Board{player_pos: Position{x: 0, y: 2}, blocks: blocks.clone()};
      let board5 = Board{player_pos: Position{x: 0, y: -2}, blocks: blocks.clone()};

      assert_eq!(board1.compare(&board2), None);
      assert_eq!(board1.compare(&board3), None);
      assert_eq!(board1.compare(&board4), None);
      assert_eq!(board1.compare(&board5), None);
    }

    #[test]
    fn compare_two_boards_with_adjacent_players_and_no_blocking_units() {
      let blocks = hashmap!{
        Position{x: 0, y: 0} => Block{ small: None, large: None },
        Position{x: -1, y: 0} => Block{ small: None, large: None },
        Position{x: 1, y: 0} => Block{ small: None, large: None },
        Position{x: 0, y: 1} => Block{ small: None, large: None },
        Position{x: 0, y: -1} => Block{ small: None, large: None },
      }; 
      let board1 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};
      let board2 = Board{player_pos: Position{x: 1, y: 0}, blocks: blocks.clone()};
      let board3 = Board{player_pos: Position{x: -1, y: 0}, blocks: blocks.clone()};
      let board4 = Board{player_pos: Position{x: 0, y: 1}, blocks: blocks.clone()};
      let board5 = Board{player_pos: Position{x: 0, y: -1}, blocks: blocks.clone()};

      assert_eq!(board1.compare(&board2), Some(Orientation::Right));
      assert_eq!(board1.compare(&board3), Some(Orientation::Left));
      assert_eq!(board1.compare(&board4), Some(Orientation::Up));
      assert_eq!(board1.compare(&board5), Some(Orientation::Down));
    }

    #[test]
    fn compare_two_boards_with_players_at_same_location() {
      let blocks = hashmap!{
        Position{x: 0, y: 0} => Block{
          small: None,
          large: None,
        },
      };
      let board1 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};
      let board2 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};

      assert_eq!(board1.compare(&board2), None);
    }

    #[test]
    fn compare_two_boards_with_adjacent_players_and_blocking_units() {
      let blocks = hashmap!{
        Position{x: 0, y: 0} => Block{
          small: None,
          large: None,
        },
        Position{x: -1, y: 0} => Block{
          small: None,
          large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
        },
        Position{x: 1, y: 0} => Block{
          small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
          large: None,
        },
        Position{x: 0, y: 1} => Block{
          small: Some(Unit{orientation: Orientation::Down, color: Color::Black}),
          large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
        },
        Position{x: 0, y: -1} => Block{
          small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
          large: None,
        },
      }; 
      let board1 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};
      let board2 = Board{player_pos: Position{x: 1, y: 0}, blocks: blocks.clone()};
      let board3 = Board{player_pos: Position{x: -1, y: 0}, blocks: blocks.clone()};
      let board4 = Board{player_pos: Position{x: 0, y: 1}, blocks: blocks.clone()};
      let board5 = Board{player_pos: Position{x: 0, y: -1}, blocks: blocks.clone()};

      assert_eq!(board1.compare(&board2), Some(Orientation::Right));
      assert_eq!(board1.compare(&board3), None);
      assert_eq!(board1.compare(&board4), None);
      assert_eq!(board1.compare(&board5), Some(Orientation::Down));
    }
}