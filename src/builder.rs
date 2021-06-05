use crate::model::*;
use crate::hasher;
use enum_iterator::IntoEnumIterator;
use maplit::hashmap;
use std::{collections::{HashMap, HashSet}};
use maplit::hashset;

fn boards_by_inserting(size: Size, color: Color, board: &Board) -> Vec<Board> {
  let player_pos = board.player_pos;
  let available_block_positions: Vec<Position> = board.blocks.iter()
    .filter(|&(k, v)| {
      return *k != player_pos && (v.small == None) && (v.large == None); 
    })
    .map(|(k, _)| k.clone())
    .collect();

    let mut boards: Vec<Board> = vec![];

    for position in available_block_positions {
      let neighbours = board.neighbours(position);

      Orientation::into_enum_iter().for_each(|orientation| {
        let opposite = orientation.opposite();
        let block_pos_on_opening_side = neighbours.neighbour_towards(&orientation);
        if block_pos_on_opening_side == None {
          return;
        }
        let block_on_opening_side = board.blocks[&block_pos_on_opening_side.unwrap()];
        if block_on_opening_side.small != None && block_on_opening_side.small.unwrap().orientation == opposite {
          return;
        }
        if block_on_opening_side.large != None && block_on_opening_side.large.unwrap().orientation == opposite {
          return;
        }

        let mut new_board = board.clone();
        let mut block = new_board.blocks.get_mut(&position).unwrap();
        match size {
          Size::Small => block.small = Some(Unit{orientation: orientation, color: color}),
          Size::Large => block.large = Some(Unit{orientation: orientation, color: color}),
        }
        boards.push(new_board);
      });
    }

    return boards;
}

fn empty_board(
  num_rows: i8,
  num_columns: i8,
  player_pos: Position
) -> Board {
  let mut blocks: HashMap<Position, Block> = hashmap!{};
  
  for x in 0..num_columns {
    for y in 0..num_rows {
      let position = Position{x, y};
      blocks.insert(position, Block{
        small: None,
        large: None,
      });
    }
  }

  return Board{
    player_pos: player_pos,
    blocks: blocks
  };
}

pub fn build(
  num_rows: i8,
  num_columns: i8,
  num_small_black: i8,
  num_large_black: i8,
  num_small_red: i8,
  num_large_red: i8,
) -> Vec<Board> {
  let mut boards:Vec<Board> = vec![];
  for x in 0..num_columns {
    for y in 0..num_rows {
      let player_pos = Position{x, y};
      let board = empty_board(num_rows, num_columns, player_pos);
      let mut wip_boards = vec![board];
  
      for _ in 0..num_small_black {
        wip_boards = wip_boards
          .iter()
          .flat_map(|board| boards_by_inserting(Size::Small, Color::Black, &board))
          .collect();
      }
  
      for _ in 0..num_large_black {
        wip_boards = wip_boards
          .iter()
          .flat_map(|board| boards_by_inserting(Size::Large, Color::Black, &board))
          .collect();
      }
  
      for _ in 0..num_small_red {
        wip_boards = wip_boards
          .iter()
          .flat_map(|board| boards_by_inserting(Size::Small, Color::Red, &board))
          .collect();
      }
  
      for _ in 0..num_large_red {
        wip_boards = wip_boards
          .iter()
          .flat_map(|board| boards_by_inserting(Size::Large, Color::Red, &board))
          .collect();
      }
  
      boards.append(&mut wip_boards);
    }
  }

  condense(boards)
}

pub fn condense(boards: Vec<Board>) -> Vec<Board> {
  let mut used_hashes: HashSet<u64> = hashset!{};

  let mut result: Vec<Board> = Vec::with_capacity(boards.len());

  for board in boards.iter() {
    let board1 = board;
    let board2 = &board1.rotate_cw_90_deg();
    let board3 = &board2.rotate_cw_90_deg();
    let board4 = &board3.rotate_cw_90_deg();
    let board5 = &board1.flip_horizontal();
    let board6 = &board2.flip_horizontal();
    let board7 = &board3.flip_horizontal();
    let board8 = &board4.flip_horizontal();

    let hashes: HashSet<u64> = vec![
      board1,
      board2,
      board3,
      board4,
      board5,
      board6,
      board7,
      board8,
    ].iter().map(|board| hasher::calculate_hash(*board)).collect();
    
    if hashes.intersection(&used_hashes).collect::<Vec<&u64>>().len() == 0 {
      result.push(board.clone());
      used_hashes.insert(hasher::calculate_hash(board));
    }
  }

  println!("{}", boards.len());
  println!("{}", result.len());
  result
}

// pub fn build(
//   num_rows: u8,
//   num_columns: u8,
//   num_small_black: u8,
//   num_large_black: u8,
//   num_small_red: u8,
//   num_large_red: u8
// ) -> Vec<Board> {
//   //TODO some sanity checks on the args?

//   let num_blocks = num_columns * num_rows;
//   // let max_id = -1;
//   let mut block_ids: Vec<u8> = vec![];
//   for i in 0..num_blocks {
//     block_ids.push(i);
//   }

//   let mut counter = 0;

//   for (i, player_id) in ids.iter().enumerate() {
//       let mut ids2 = ids.clone();
//       ids2.remove(i);

//       for (i, red_small_id) in ids2.iter().enumerate() {
//         let mut ids3 = ids2.clone();
//         ids3.remove(i);

//         Orientation::into_enum_iter().for_each(|small_red_orientation| {
//             for (i, red_large_id) in ids3.iter().enumerate() {
//                 let mut ids4 = ids3.clone();
//                 ids4.remove(i);
        
//                 Orientation::into_enum_iter().for_each(|large_red_orientation| {
//                     let mut blocks: HashMap<u8, Block> = hashmap!{};
//                     for i in 0..num_blocks {
//                         blocks.insert(i, Block{
//                             small: None,
//                             large: None,
//                             id: i,
//                             neighbour_ids: NeighbourIds::new(
//                                 block_id(i, num_columns, num_rows, Orientation::Up),
//                                 block_id(i, num_columns, num_rows, Orientation::Down),
//                                 block_id(i, num_columns, num_rows, Orientation::Left),
//                                 block_id(i, num_columns, num_rows, Orientation::Right),
//                             )
//                         });
//                     }

//                     let mut red_small_block = blocks[&red_small_id];
//                     red_small_block.small = Some(Unit{orientation: small_red_orientation, color: Color::Red});

//                     let mut red_large_block = blocks[&red_large_id];
//                     red_large_block.large = Some(Unit{orientation: large_red_orientation, color: Color::Red});

//                     let first_board = Board{
//                         player: Player{block_id: *player_id},
//                         blocks: blocks
//                     };
//                     let first_board_hash = calculate_hash(&first_board);
                
//                     let mut boards: HashMap<u64, Board> = hashmap!{};
                    
//                     build(&first_board, &mut boards, &mut c.borrow_mut());
                
//                     print(&first_board);
//                     println!("{}", counter);
//                     counter += 1;

//                     let goals = goals(&boards, &c.borrow());
//                 });
//             }
//         });
//     }

//   return vec![];
// }

#[cfg(test)]
mod test {
    use crate::{hasher::calculate_hash, utils};

    use super::*;

    #[test]
    fn doesnt_insert_units_onto_player_block() {
      let board = Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 0} => Block{
            small: None,
            large: None,
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 1);
    }

    #[test]
    fn doesnt_insert_units_onto_blocks_containing_unit_matching_size() {
      let board = Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 0} => Block{
            small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
            large: None,
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 0);
    }

    #[test]
    fn doesnt_insert_units_onto_blocks_containing_unit_not_matching_size() {
      let board = Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 0} => Block{
            small: None,
            large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_empty_board() {
      let board = empty_board(3, 4, Position{x: 1, y: 0});
      let expected = Board{
        player_pos: Position{x: 1, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 2, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 3, y: 0} => Block{
            small: None,
            large: None,
          },
          Position{x: 0, y: 1} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 1} => Block{
            small: None,
            large: None,
          },
          Position{x: 2, y: 1} => Block{
            small: None,
            large: None,
          },
          Position{x: 3, y: 1} => Block{
            small: None,
            large: None,
          },
          Position{x: 0, y: 2} => Block{
            small: None,
            large: None,
          },
          Position{x: 1, y: 2} => Block{
            small: None,
            large: None,
          },
          Position{x: 2, y: 2} => Block{
            small: None,
            large: None,
          },
          Position{x: 3, y: 2} => Block{
            small: None,
            large: None,
          },
        }
      };

      assert_eq!(board, expected);
    }

    #[test]
    fn inserting_into_board() {
      let board = empty_board(2, 1, Position{x: 0, y: 0});
      let boards = boards_by_inserting(Size::Small, Color::Black, &board);

      assert_eq!(
        boards,
        [
          Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
              Position{x: 0, y: 0} => Block{
                small: None,
                large: None,
              },
              Position{x: 0, y: 1} => Block{
                small: Some(Unit{orientation: Orientation::Down, color: Color::Black}),
                large: None,
              },
            }
          },
        ]
      )
    }

    #[test]
    fn test_build1() {
      let boards = build(1, 3, 2, 0, 0, 0);
      assert_eq!(boards.len(), 2);
    }

    #[test]
    fn test_build2() {
      let boards = build(4, 1, 1, 2, 0, 0);
      assert_eq!(boards.len(), 6);
    }

    #[test]
    // fn test_build3() {
    //   let boards = build(3, 3, 1, 1, 1, 1);
    //   assert_eq!(boards.len(), 529080);
    // }

    #[test]
    fn test_build4() {
      let boards = build(2, 2, 1, 0, 0, 0);
      assert_eq!(boards.len(), 3);
    }

    #[test]
    fn condense_removes_flipped_and_rotated_boards() {
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
      let board2 = board1.rotate_cw_90_deg();
      let board3 = board2.rotate_cw_90_deg();
      let board4 = board3.rotate_cw_90_deg();
      let board5 = board1.flip_horizontal();
      let board6 = board2.flip_horizontal();
      let board7 = board3.flip_horizontal();
      let board8 = board4.flip_horizontal();

      assert_eq!(
        condense(
          vec![
            board1,
            board2,
            board3,
            board4,
            board5,
            board6,
            board7,
            board8,
          ]
        ).len(),
        1
      );
    }

    #[test]
    fn condense2() {
      assert_eq!(
        condense(
          vec![
            Board{
              player_pos: Position{x: 0, y: 0},
              blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None,},
                Position{x: 1, y: 0} => Block{small: None, large: None,},
                Position{x: 0, y: 1} => Block{small: Some(Unit{orientation: Orientation::Down, color: Color::Black}), large: None,},
                Position{x: 1, y: 1} => Block{small: None, large: None,},
              }
            },
            Board{
              player_pos: Position{x: 0, y: 1},
              blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: Some(Unit{orientation: Orientation::Up, color: Color::Black}), large: None,},
                Position{x: 1, y: 0} => Block{small: None, large: None,},
                Position{x: 0, y: 1} => Block{small: None, large: None,},
                Position{x: 1, y: 1} => Block{small: None, large: None,},
              }
            },
          ]
        ).len(),
        1
      );
    }

    #[test]
    fn condense3() {
      assert_eq!(
        condense(
          vec![
            Board{
              player_pos: Position{x: 0, y: 0},
              blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None,},
                Position{x: 1, y: 0} => Block{small: None, large: None,},
                Position{x: 0, y: 1} => Block{small: Some(Unit{orientation: Orientation::Down, color: Color::Black}), large: None,},
                Position{x: 1, y: 1} => Block{small: None, large: None,},
              }
            },
            Board{
              player_pos: Position{x: 0, y: 0},
              blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None,},
                Position{x: 1, y: 0} => Block{small: Some(Unit{orientation: Orientation::Left, color: Color::Black}), large: None,},
                Position{x: 0, y: 1} => Block{small: None, large: None,},
                Position{x: 1, y: 1} => Block{small: None, large: None,},
              }
            },
          ]
        ).len(),
        1
      );
    }

    #[test]
    fn rotate_cw() {
      let board = Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{small: None, large: None,},
          Position{x: 1, y: 0} => Block{small: None, large: None,},
          Position{x: 0, y: 1} => Block{small: Some(Unit{orientation: Orientation::Down, color: Color::Black}), large: None,},
          Position{x: 1, y: 1} => Block{small: None, large: None,},
        }
      };
      assert_eq!(
        board.rotate_cw_90_deg(),
        Board{
          player_pos: Position{x: 0, y: 0},
          blocks: hashmap!{
            Position{x: 0, y: 0} => Block{small: None, large: None,},
            Position{x: 1, y: 0} => Block{small: Some(Unit{orientation: Orientation::Left, color: Color::Black}), large: None,},
            Position{x: 0, y: -1} => Block{small: None, large: None,},
            Position{x: 1, y: -1} => Block{small: None, large: None,},
          }
        },
      );
    }

    #[test]
    fn hashing_automatically_translates_board_to_origin() {
      let board1 = Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
          Position{x: 0, y: 0} => Block{small: None, large: None,},
          Position{x: 1, y: 0} => Block{small: None, large: None,},
          Position{x: 0, y: 1} => Block{small: Some(Unit{orientation: Orientation::Down, color: Color::Black}), large: None,},
          Position{x: 1, y: 1} => Block{small: None, large: None,},
        }
      };
      let board2 = board1.translate(Position{x: 2, y: 3});
      let board3 = board1.translate(Position{x: -3, y: -2});
      assert_eq!(
        calculate_hash(&board1),
        calculate_hash(&board2),
      );
      assert_eq!(
        calculate_hash(&board1),
        calculate_hash(&board3),
      );
    }
}