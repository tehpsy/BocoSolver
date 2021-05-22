use crate::model::*;
use crate::hasher;
use enum_iterator::IntoEnumIterator;
use maplit::hashmap;
use std::{collections::{HashMap, HashSet}};
use maplit::hashset;

fn boards_by_inserting(size: Size, color: Color, board: &Board) -> Vec<Board> {
  let player_block_id = board.player.block_id;
  let available_block_ids: Vec<u8> = board.blocks.iter()
    .filter(|&(k, v)| {
      return *k != player_block_id && (v.small == None) && (v.large == None); 
    })
    .map(|(k, _)| k.clone())
    .collect();

    let mut boards: Vec<Board> = vec![];

    for i in available_block_ids {
      Orientation::into_enum_iter().for_each(|orientation| {
        let opposite = orientation.opposite();
        let block_id_on_opening_side = board.blocks[&i].neighbour_ids.neighbour_towards(&orientation);
        if block_id_on_opening_side == None {
          return;
        }
        let block_on_opening_side = board.blocks[&block_id_on_opening_side.unwrap()];
        if block_on_opening_side.small != None && block_on_opening_side.small.unwrap().orientation == opposite {
          return;
        }
        if block_on_opening_side.large != None && block_on_opening_side.large.unwrap().orientation == opposite {
          return;
        }

        let mut new_board = board.clone();
        let mut block = new_board.blocks.get_mut(&i).unwrap();
        match size {
          Size::Small => block.small = Some(Unit{orientation: orientation, color: color}),
          Size::Large => block.large = Some(Unit{orientation: orientation, color: color}),
        }
        boards.push(new_board);
      });
    }

    return boards;
}

fn calc_block_id(curr_block_id: u8, num_columns: u8, num_rows: u8, orientation: Orientation) -> Option<u8> {
  let max = num_columns * num_rows;

  let val: Option<u8>;

  match orientation {
      Orientation::Up => 
          if curr_block_id < num_columns { val = None; } else { val = Some(curr_block_id - num_columns); },
      Orientation::Down =>
          if curr_block_id >= max - num_columns { val = None; } else { val = Some(curr_block_id + num_columns); },
      Orientation::Left =>
          if curr_block_id % num_columns == 0 { val = None; } else { val = Some(curr_block_id - 1); },
      Orientation::Right =>
          if curr_block_id % num_columns == num_columns - 1 { val = None; } else { val = Some(curr_block_id + 1); },
  };

  return val;
}

fn empty_board(
  num_rows: u8,
  num_columns: u8,
  player_id: u8
) -> Board {
  let mut blocks: HashMap<u8, Block> = hashmap!{};
  let num_blocks = num_columns * num_rows;

  for i in 0..num_blocks {
    blocks.insert(i, Block{
      small: None,
      large: None,
      id: i,
      neighbour_ids: NeighbourIds{
        up: calc_block_id(i, num_columns, num_rows, Orientation::Up),
        down: calc_block_id(i, num_columns, num_rows, Orientation::Down),
        left: calc_block_id(i, num_columns, num_rows, Orientation::Left),
        right: calc_block_id(i, num_columns, num_rows, Orientation::Right),
      }
    });
  }

  return Board{
    player: Player{block_id: player_id},
    blocks: blocks
  };
}

pub fn build(
  num_rows: u8,
  num_columns: u8,
  num_small_black: u8,
  num_large_black: u8,
  num_small_red: u8,
  num_large_red: u8
) -> Vec<Board> {
  let mut boards:Vec<Board> = vec![];
  let num_blocks = num_columns * num_rows;
  for player_id in 0..num_blocks {
    let board = empty_board(num_rows, num_columns, player_id);
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
  
  // boards
  condense(boards)
}

pub fn condense(boards: Vec<Board>) -> Vec<Board> {
  let mut used_hashes: HashSet<u64> = hashset!{};

  let mut result: Vec<Board> = vec![];

  for board in boards.iter() {
    let board1 = board;
    let board2 = &board1.rotate_cw_90_deg();
    let board3 = &board2.rotate_cw_90_deg();
    let board4 = &board3.rotate_cw_90_deg();

    let hashes: HashSet<u64> = vec![
      board1,
      board2,
      board3,
      board4,
      &board1.flip_horizontal(),
      &board2.flip_horizontal(),
      &board3.flip_horizontal(),
      &board4.flip_horizontal(),
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
    use super::*;

    #[test]
    fn doesnt_insert_units_onto_player_block() {
      let board = Board{
        player: Player{block_id: 0},
        blocks: hashmap!{
          0 => Block{
            small: None,
            large: None,
            id: 0,
            neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
          },
          1 => Block{
            small: None,
            large: None,
            id: 1,
            neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 4);
    }

    #[test]
    fn doesnt_insert_units_onto_blocks_containing_unit_matching_size() {
      let board = Board{
        player: Player{block_id: 0},
        blocks: hashmap!{
          0 => Block{
            small: None,
            large: None,
            id: 0,
            neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
          },
          1 => Block{
            small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
            large: None,
            id: 1,
            neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 0);
    }

    #[test]
    fn doesnt_insert_units_onto_blocks_containing_unit_not_matching_size() {
      let board = Board{
        player: Player{block_id: 0},
        blocks: hashmap!{
          0 => Block{
            small: None,
            large: None,
            id: 0,
            neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
          },
          1 => Block{
            small: None,
            large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
            id: 1,
            neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
          },
        }
      };

      let boards = boards_by_inserting(Size::Small, Color::Black, &board);
      assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_empty_board() {
      let board = empty_board(3, 4, 1);
      let expected = Board{
        player: Player{block_id: 1},
        blocks: hashmap!{
          0 => Block{
            small: None,
            large: None,
            id: 0,
            neighbour_ids: NeighbourIds::new(None, Some(4), None, Some(1))
          },
          1 => Block{
            small: None,
            large: None,
            id: 1,
            neighbour_ids: NeighbourIds::new(None, Some(5), Some(0), Some(2))
          },
          2 => Block{
            small: None,
            large: None,
            id: 2,
            neighbour_ids: NeighbourIds::new(None, Some(6), Some(1), Some(3))
          },
          3 => Block{
            small: None,
            large: None,
            id: 3,
            neighbour_ids: NeighbourIds::new(None, Some(7), Some(2), None)
          },
          4 => Block{
            small: None,
            large: None,
            id: 4,
            neighbour_ids: NeighbourIds::new(Some(0), Some(8), None, Some(5))
          },
          5 => Block{
            small: None,
            large: None,
            id: 5,
            neighbour_ids: NeighbourIds::new(Some(1), Some(9), Some(4), Some(6))
          },
          6 => Block{
            small: None,
            large: None,
            id: 6,
            neighbour_ids: NeighbourIds::new(Some(2), Some(10), Some(5), Some(7))
          },
          7 => Block{
            small: None,
            large: None,
            id: 7,
            neighbour_ids: NeighbourIds::new(Some(3), Some(11), Some(6), None)
          },
          8 => Block{
            small: None,
            large: None,
            id: 8,
            neighbour_ids: NeighbourIds::new(Some(4), None, None, Some(9))
          },
          9 => Block{
            small: None,
            large: None,
            id: 9,
            neighbour_ids: NeighbourIds::new(Some(5), None, Some(8), Some(10))
          },
          10 => Block{
            small: None,
            large: None,
            id: 10,
            neighbour_ids: NeighbourIds::new(Some(6), None, Some(9), Some(11))
          },
          11 => Block{
            small: None,
            large: None,
            id: 11,
            neighbour_ids: NeighbourIds::new(Some(7), None, Some(10), None)
          },
        }
      };

      assert_eq!(board, expected);
    }

    #[test]
    fn inserting_into_board() {
      let board = empty_board(2, 1, 0);
      let boards = boards_by_inserting(Size::Small, Color::Black, &board);

      assert_eq!(
        boards,
        [
          Board{
            player: Player{block_id: 0},
            blocks: hashmap!{
              0 => Block{
                small: None,
                large: None,
                id: 0,
                neighbour_ids: NeighbourIds::new(None, Some(1), None, None)
              },
              1 => Block{
                small: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
                large: None,
                id: 1,
                neighbour_ids: NeighbourIds::new(Some(0), None, None, None)
              },
            }
          },
          Board{
            player: Player{block_id: 0},
            blocks: hashmap!{
              0 => Block{
                small: None,
                large: None,
                id: 0,
                neighbour_ids: NeighbourIds::new(None, Some(1), None, None)
              },
              1 => Block{
                small: Some(Unit{orientation: Orientation::Down, color: Color::Black}),
                large: None,
                id: 1,
                neighbour_ids: NeighbourIds::new(Some(0), None, None, None)
              },
            }
          },
          Board{
            player: Player{block_id: 0},
            blocks: hashmap!{
              0 => Block{
                small: None,
                large: None,
                id: 0,
                neighbour_ids: NeighbourIds::new(None, Some(1), None, None)
              },
              1 => Block{
                small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
                large: None,
                id: 1,
                neighbour_ids: NeighbourIds::new(Some(0), None, None, None)
              },
            }
          },
          Board{
            player: Player{block_id: 0},
            blocks: hashmap!{
              0 => Block{
                small: None,
                large: None,
                id: 0,
                neighbour_ids: NeighbourIds::new(None, Some(1), None, None)
              },
              1 => Block{
                small: Some(Unit{orientation: Orientation::Right, color: Color::Black}),
                large: None,
                id: 1,
                neighbour_ids: NeighbourIds::new(Some(0), None, None, None)
              },
            }
          },
        ]
      )
    }

    #[test]
    fn test_build() {
      let boards = build(1, 3, 2, 0, 0, 0);
      let num_player_positions = 3;
      let num_available_block_slots = 2;
      let num_orientations = 4;
      assert_eq!(
        boards.len(),
        num_player_positions * (num_available_block_slots * num_orientations) * ((num_available_block_slots-1) * num_orientations)
      );
    }

    #[test]
    fn test_build2() {
      let boards = build(4, 1, 1, 2, 0, 0);
      assert_eq!(boards.len(), 1536);
    }

    #[test]
    fn test_build3() {
      let boards = build(3, 3, 1, 1, 1, 1);
      assert_eq!(boards.len(), 3870720);
    }

    #[test]
    fn condense_removes_flipped_and_rotated_boards() {
      let original_board = Board{
        player: Player{block_id: 0},
        blocks: hashmap!{
          0 => Block{
            small: None,
            large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
            id: 0,
            neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
          },
          1 => Block{
            small: Some(Unit{orientation: Orientation::Left, color: Color::Black}),
            large: None,
            id: 1,
            neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
          },
          2 => Block{
            small: None,
            large: Some(Unit{orientation: Orientation::Up, color: Color::Black}),
            id: 2,
            neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
          },
          3 => Block{
            small: Some(Unit{orientation: Orientation::Down, color: Color::Red}),
            large: None,
            id: 3,
            neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
          },
        }
      };
      let rotated_board = original_board.rotate_cw_90_deg();
      let flipped_board = original_board.flip_horizontal();

      assert_eq!(
        condense(
          vec![
            original_board,
            rotated_board,
            flipped_board,
          ]
        ).len(),
        1
      );
    }
}