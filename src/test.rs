// use super::*;
// use maplit::hashset;
// use utils;
// use enum_iterator::IntoEnumIterator;
// use model::*;
// use maplit::hashmap;
// use super::*;
// use hasher::*;
// use std::collections::HashMap;
// use petgraph::graphmap::UnGraphMap;
// use std::rc::Rc;
// use std::cell::RefCell;

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_available_moves_when_no_units_on_board() {
//         let two_tiles = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                 1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//             }
//         };
//         assert_eq!(two_tiles.available_moves(), hashset![Orientation::Right]);
//         assert_eq!(Board{player: Player{block_id: 1}, ..two_tiles}.available_moves(), hashset![Orientation::Left]);

//         let single_tile = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, None)},
//             }
//         };
//         assert_eq!(single_tile.available_moves(), hashset![]);

//         fn create_complex_layout(block_id: u8) -> Board{
//             return Board{
//                 player: Player{block_id: block_id},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), Some(2))},
//                     2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(4), Some(1), None)},
//                     3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(1), None, None, Some(4))},
//                     4 => Block{small: None, large: None, id: 4, neighbour_ids: NeighbourIds::new(Some(2), None, Some(3), None)},
//                 }
//             }
//         }

//         assert_eq!(create_complex_layout(0).available_moves(), hashset![Orientation::Right]);
//         assert_eq!(create_complex_layout(1).available_moves(), hashset![Orientation::Right, Orientation::Left, Orientation::Down]);
//         assert_eq!(create_complex_layout(2).available_moves(), hashset![Orientation::Left, Orientation::Down]);
//         assert_eq!(create_complex_layout(3).available_moves(), hashset![Orientation::Right, Orientation::Up]);
//         assert_eq!(create_complex_layout(4).available_moves(), hashset![Orientation::Left, Orientation::Up]);
//     }

//     #[test]
//     fn test_available_moves_when_units_on_board() {
//         fn create_3_x_3_layout(block_id: u8, center_unit_orientation: Orientation) -> Board {
//             return Board{
//                 player: Player{block_id: block_id},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(4), Some(0), Some(2))},
//                     2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(5), Some(1), None)},
//                     3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(0), Some(6), None, Some(4))},
//                     4 => Block{small: Some(Unit{
//                         orientation: center_unit_orientation,
//                         color: Color::Red,
//                     }), large: None, id: 4, neighbour_ids: NeighbourIds::new(Some(1), Some(7), Some(3), Some(5))},
//                     5 => Block{small: None, large: None, id: 5, neighbour_ids: NeighbourIds::new(Some(2), Some(8), Some(4), None)},
//                     6 => Block{small: None, large: None, id: 6, neighbour_ids: NeighbourIds::new(Some(3), None, None, Some(7))},
//                     7 => Block{small: None, large: None, id: 7, neighbour_ids: NeighbourIds::new(Some(4), None, Some(6), Some(8))},
//                     8 => Block{small: None, large: None, id: 8, neighbour_ids: NeighbourIds::new(Some(5), None, Some(7), None)},
//                 }
//             }
//         }

//         assert_eq!(create_3_x_3_layout(1, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(1, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(1, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(1, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(3, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(3, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(3, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(3, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(5, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(5, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(5, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down]);
//         assert_eq!(create_3_x_3_layout(5, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Left]);
//         assert_eq!(create_3_x_3_layout(7, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(7, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Up]);
//         assert_eq!(create_3_x_3_layout(7, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//         assert_eq!(create_3_x_3_layout(7, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
//     }

//     #[test]
//     fn test_can_move_into_block_only_when_all_block_units_align() {
//         fn create_3_x_3_layout(block_id: u8, orientation_small: Orientation, orientation_large: Orientation) -> Board {
//             return Board{
//                 player: Player{block_id: block_id},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(4), Some(0), Some(2))},
//                     2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(5), Some(1), None)},
//                     3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(0), Some(6), None, Some(4))},
//                     4 => Block{small: Some(Unit{
//                         orientation: orientation_small,
//                         color: Color::Red,
//                     }), 
//                     large: Some(Unit{
//                         orientation: orientation_large,
//                         color: Color::Red,
//                     }), id: 4, neighbour_ids: NeighbourIds::new(Some(1), Some(7), Some(3), Some(5))},
//                     5 => Block{small: None, large: None, id: 5, neighbour_ids: NeighbourIds::new(Some(2), Some(8), Some(4), None)},
//                     6 => Block{small: None, large: None, id: 6, neighbour_ids: NeighbourIds::new(Some(3), None, None, Some(7))},
//                     7 => Block{small: None, large: None, id: 7, neighbour_ids: NeighbourIds::new(Some(4), None, Some(6), Some(8))},
//                     8 => Block{small: None, large: None, id: 8, neighbour_ids: NeighbourIds::new(Some(5), None, Some(7), None)},
//                 }
//             }
//         }

//         fn assert(block_id: u8, available_orientation: Orientation) {
//             Orientation::into_enum_iter().for_each(|orientation_small| {
//                 Orientation::into_enum_iter().for_each(|orientation_large| {
//                     let assert_val =
//                         available_orientation.opposite() == orientation_small &&
//                         available_orientation.opposite() == orientation_large;

//                     assert_eq!(
//                         create_3_x_3_layout(block_id, orientation_small, orientation_large).available_moves().contains(&available_orientation),
//                         assert_val
//                     );
//                 })
//             })
//         }

//         assert(1, Orientation::Down);
//         assert(3, Orientation::Right);
//         assert(5, Orientation::Left);
//         assert(7, Orientation::Up);
//     }

//     #[test]
//     fn test_only_one_unit_type_allowed_per_block() {
//         {
//             let board = Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                         large: None,
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             };
//             assert!(board.available_moves().is_empty());
//         }

//         {
//             let board = Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: None,
//                         large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: None,
//                         large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             };
//             assert!(board.available_moves().is_empty());
//         }
//     }

//     #[test]
//     fn test_moves_not_limited_by_unit_type_if_current_unit_does_not_move() {
//         {
//             let board = Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                         large: None,
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             };
//             assert!(board.available_moves().contains(&Orientation::Right));
//         }

//         {
//             let board = Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: None,
//                         large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: None,
//                         large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             };
//             assert!(board.available_moves().contains(&Orientation::Right));
//         }
//     }

//     #[test]
//     fn test_no_win_when_no_units() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }.is_win(),
//             false,
//         );
//     }

//     #[test]
//     fn test_no_win_when_units_match_location_but_only_one_is_red() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Black,
//                         }),
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: None,
//                         large: None,
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             }.is_win(),
//             false,
//         );
//     }

//     #[test]
//     fn test_no_win_when_red_units_at_different_locations() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: None,
//                         large: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             }.is_win(),
//             false,
//         );
//     }

//     #[test]
//     fn test_no_win_when_red_units_at_same_location_but_player_is_not() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: None,
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             }.is_win(),
//             false,
//         );
//     }

//     #[test]
//     fn test_win_when_red_units_at_same_location_as_player() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: None,
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                     },
//                     1 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Up,
//                             color: Color::Red,
//                         }),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                     },
//                 }
//             }.is_win(),
//             true,
//         );
//     }

//     #[test]
//     fn test_moving_moves_player() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }.moving(Orientation::Right),
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }
//         );
//     }

//     #[test]
//     fn test_moving_moves_units() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }), large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }.moving(Orientation::Right),
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }), large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }
//         );

//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Red,
//                     }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }.moving(Orientation::Right),
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Red,
//                     }), id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }
//         );
//     }

//     #[test]
//     fn test_units_do_not_move_if_orientation_matches_direction_of_movement() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Right,
//                             color: Color::Red,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Right,
//                             color: Color::Red,
//                         }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }.moving(Orientation::Right),
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Right,
//                             color: Color::Red,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Right,
//                             color: Color::Red,
//                         }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
//                     1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
//                 }
//             }
//         );
//     }

//     #[test]
//     fn test_board_building() {
//         let graph = UnGraphMap::<NetworkNode, ()>::new();
//         let rc = RefCell::new(graph); 
//         let c = Rc::new(rc);

//         let first_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
//                 },
//                 1 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Red,
//                     }),
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
//                 },
//             }
//         };
//         let mut boards: HashMap<u64, Board> = hashmap!{};
        
//         utils::build(&first_board, &mut boards, &mut c.borrow_mut());

//         assert_eq!(c.borrow().node_count(), 2);
//         assert_eq!(c.borrow().edge_count(), 1);
//         assert_eq!(boards.len(), c.borrow().node_count());
//         assert_eq!(utils::can_win(&boards, & c.borrow()), true);
//     }

//     #[test]
//     fn test_board_building_2() {
//         let graph = UnGraphMap::<NetworkNode, ()>::new();
//         let rc = RefCell::new(graph); 
//         let c = Rc::new(rc);

//         let first_board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
//                 },
//                 1 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Red,
//                     }),
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
//                 },
//                 2 => Block{
//                     small: None,
//                     large: None,
//                     id: 2,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
//                 },
//                 3 => Block{
//                     small: None,
//                     large: None,
//                     id: 3,
//                     neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
//                 },
//             }
//         };

//         let mut boards: HashMap<u64, Board> = hashmap!{};
        
//         utils::build(&first_board, &mut boards, &mut c.borrow_mut());

//         assert_eq!(c.borrow().node_count(), 7);
//         assert_eq!(c.borrow().edge_count(), 6);
//         assert_eq!(boards.len(), c.borrow().node_count());
//         assert_eq!(utils::can_win(&boards, & c.borrow()), true);
//     }

//     #[test]
//     fn test_board_building_3() {
//         let graph = UnGraphMap::<NetworkNode, ()>::new();
//         let rc = RefCell::new(graph); 
//         let c = Rc::new(rc);

//         let first_board = Board{
//             player: Player{block_id: 8},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: None,
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(5), None, Some(1))
//                 },
//                 1 => Block{
//                     small: None,
//                     large: None,
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(None, Some(6), Some(0), Some(2))
//                 },
//                 2 => Block{
//                     small: None,
//                     large: None,
//                     id: 2,
//                     neighbour_ids: NeighbourIds::new(None, Some(7), Some(1), Some(3))
//                 },
//                 3 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Down,
//                         color: Color::Black,
//                     }),
//                     id: 3,
//                     neighbour_ids: NeighbourIds::new(None, Some(8), Some(2), Some(4))
//                 },
//                 4 => Block{
//                     small: None,
//                     large: None,
//                     id: 4,
//                     neighbour_ids: NeighbourIds::new(None, Some(9), Some(3), None)
//                 },
//                 5 => Block{
//                     small: None,
//                     large: None,
//                     id: 5,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(6))
//                 },
//                 6 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     id: 6,
//                     neighbour_ids: NeighbourIds::new(Some(1), None, Some(5), Some(7))
//                 },
//                 7 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Black,
//                     }),
//                     id: 7,
//                     neighbour_ids: NeighbourIds::new(Some(2), None, Some(6), Some(8))
//                 },
//                 8 => Block{
//                     small: None,
//                     large: None,
//                     id: 8,
//                     neighbour_ids: NeighbourIds::new(Some(3), None, Some(7), Some(9))
//                 },
//                 9 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 9,
//                     neighbour_ids: NeighbourIds::new(Some(4), None, Some(8), None)
//                 },
//             }
//         };

//         let last_board = Board{
//             player: Player{block_id: 6},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Down,
//                         color: Color::Black,
//                     }),
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(5), None, Some(1))
//                 },
//                 1 => Block{
//                     small: None,
//                     large: None,
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(None, Some(6), Some(0), Some(2))
//                 },
//                 2 => Block{
//                     small: None,
//                     large: None,
//                     id: 2,
//                     neighbour_ids: NeighbourIds::new(None, Some(7), Some(1), Some(3))
//                 },
//                 3 => Block{
//                     small: None,
//                     large: None,
//                     id: 3,
//                     neighbour_ids: NeighbourIds::new(None, Some(8), Some(2), Some(4))
//                 },
//                 4 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Black,
//                     }),
//                     id: 4,
//                     neighbour_ids: NeighbourIds::new(None, Some(9), Some(3), None)
//                 },
//                 5 => Block{
//                     small: None,
//                     large: None,
//                     id: 5,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(6))
//                 },
//                 6 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     id: 6,
//                     neighbour_ids: NeighbourIds::new(Some(1), None, Some(5), Some(7))
//                 },
//                 7 => Block{
//                     small: None,
//                     large: None,
//                     id: 7,
//                     neighbour_ids: NeighbourIds::new(Some(2), None, Some(6), Some(8))
//                 },
//                 8 => Block{
//                     small: None,
//                     large: None,
//                     id: 8,
//                     neighbour_ids: NeighbourIds::new(Some(3), None, Some(7), Some(9))
//                 },
//                 9 => Block{
//                     small: None,
//                     large: None,
//                     id: 9,
//                     neighbour_ids: NeighbourIds::new(Some(4), None, Some(8), None)
//                 },
//             }
//         };

//         let intermediate_board = Board{
//             player: Player{block_id: 8},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: None,
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(5), None, Some(1))
//                 },
//                 1 => Block{
//                     small: None,
//                     large: None,
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(None, Some(6), Some(0), Some(2))
//                 },
//                 2 => Block{
//                     small: None,
//                     large: None,
//                     id: 2,
//                     neighbour_ids: NeighbourIds::new(None, Some(7), Some(1), Some(3))
//                 },
//                 3 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Down,
//                         color: Color::Black,
//                     }),
//                     id: 3,
//                     neighbour_ids: NeighbourIds::new(None, Some(8), Some(2), Some(4))
//                 },
//                 4 => Block{
//                     small: None,
//                     large: None,
//                     id: 4,
//                     neighbour_ids: NeighbourIds::new(None, Some(9), Some(3), None)
//                 },
//                 5 => Block{
//                     small: None,
//                     large: None,
//                     id: 5,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(6))
//                 },
//                 6 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     id: 6,
//                     neighbour_ids: NeighbourIds::new(Some(1), None, Some(5), Some(7))
//                 },
//                 7 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Black,
//                     }),
//                     id: 7,
//                     neighbour_ids: NeighbourIds::new(Some(2), None, Some(6), Some(8))
//                 },
//                 8 => Block{
//                     small: None,
//                     large: None,
//                     id: 8,
//                     neighbour_ids: NeighbourIds::new(Some(3), None, Some(7), Some(9))
//                 },
//                 9 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 9,
//                     neighbour_ids: NeighbourIds::new(Some(4), None, Some(8), None)
//                 },
//             }
//         };

//         let mut boards: HashMap<u64, Board> = hashmap!{};
//         let intermediate_board_hash = calculate_hash(&intermediate_board);

//         let first_hash_1 = calculate_hash(&first_board);
//         let first_hash_2 = calculate_hash(&first_board);
//         let last_hash_1 = calculate_hash(&last_board);
//         let last_hash_2 = calculate_hash(&last_board);

//         utils::build(&first_board, &mut boards, &mut c.borrow_mut());

//         // assert_eq!(first_hash_1, first_hash_2);
//         // assert_eq!(last_hash_1, last_hash_2);
//         // assert_eq!(first_hash_1, last_hash_2);


//         // let node_count = c.borrow().node_count();
//         // let edge_count = c.borrow().edge_count();
//         // println!("{}", node_count);
//         // println!("{}", boards.len());
//         // println!("{}", edge_count);
//         // println!("{}", first_board_hash);
//         // println!("{}", intermediate_board_hash);
        
//         // assert_eq!(boards.contains_key(&last_board_hash), true);
        
//         // assert_eq!(boards.contains_key(&first_board_hash), true);
        
//         // assert_eq!(false, true);

//         // assert_eq!(boards.contains_key(&intermediate_board_hash), true);
//         // assert_eq!(c.borrow().node_count(), 14);
//         // assert_eq!(c.borrow().edge_count(), 13);
//         assert_eq!(utils::can_win(&boards, & c.borrow()), true);
//     }

//     #[test]
//     fn test_large_unit_cannot_pass_through_small_unit() {
//         let board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Black,
//                     }),
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//                 1 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//             }
//         };

//         assert_eq!(board.available_moves(), hashset![]);
//     }

//     #[test]
//     fn test_available_moves_when_moving_from_large_unit_to_small_unit() {
//         let board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: None,
//                     large: Some(Unit{
//                         orientation: Orientation::Down,
//                         color: Color::Black,
//                     }),
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//                 1 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Up,
//                         color: Color::Red,
//                     }),
//                     large: None,
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//             }
//         };

//         assert_eq!(board.available_moves(), hashset![Orientation::Down]);
//     }

//     #[test]
//     fn test_available_moves_when_two_units_with_different_orientations() {
//         let board = Board{
//             player: Player{block_id: 0},
//             blocks: hashmap!{
//                 0 => Block{
//                     small: Some(Unit{
//                         orientation: Orientation::Down,
//                         color: Color::Black,
//                     }),
//                     large: Some(Unit{
//                         orientation: Orientation::Left,
//                         color: Color::Black,
//                     }),
//                     id: 0,
//                     neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//                 1 => Block{
//                     small: None,
//                     large: None,
//                     id: 1,
//                     neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//             }
//         };

//         assert_eq!(board.available_moves(), hashset![Orientation::Down]);
//     }

//     #[test]
//     fn test_moving_when_two_units_with_different_orientations() {
//         assert_eq!(
//             Board{
//                 player: Player{block_id: 0},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Down,
//                             color: Color::Black,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Left,
//                             color: Color::Black,
//                         }),
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//                     1 => Block{
//                         small: None,
//                         large: None,
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//                 }
//             }.moving(Orientation::Down),
//             Board{
//                 player: Player{block_id: 1},
//                 blocks: hashmap!{
//                     0 => Block{
//                         small: None,
//                         large: None,
//                         id: 0,
//                         neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//                     1 => Block{
//                         small: Some(Unit{
//                             orientation: Orientation::Down,
//                             color: Color::Black,
//                         }),
//                         large: Some(Unit{
//                             orientation: Orientation::Left,
//                             color: Color::Black,
//                         }),
//                         id: 1,
//                         neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//                 }
//             }
//         );

//         // assert_eq!(
//         //     Board{
//         //         player: Player{block_id: 0},
//         //         blocks: hashmap!{
//         //             0 => Block{
//         //                 small: Some(Unit{
//         //                     orientation: Orientation::Right,
//         //                     color: Color::Black,
//         //                 }),
//         //                 large: None,
//         //                 id: 0,
//         //                 neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//         //             1 => Block{
//         //                 small: None,
//         //                 large: Some(Unit{
//         //                     orientation: Orientation::Up,
//         //                     color: Color::Black,
//         //                 }),
//         //                 id: 1,
//         //                 neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//         //         }
//         //     }.moving(Orientation::Down),
//         //     Board{
//         //         player: Player{block_id: 1},
//         //         blocks: hashmap!{
//         //             0 => Block{
//         //                 small: None,
//         //                 large: None,
//         //                 id: 0,
//         //                 neighbour_ids: NeighbourIds::new(None, Some(1), None, None)},
//         //             1 => Block{
//         //                 small: Some(Unit{
//         //                     orientation: Orientation::Right,
//         //                     color: Color::Black,
//         //                 }),
//         //                 large: Some(Unit{
//         //                     orientation: Orientation::Up,
//         //                     color: Color::Black,
//         //                 }),
//         //                 id: 1,
//         //                 neighbour_ids: NeighbourIds::new(Some(0), None, None, None)},
//         //         }
//         //     }
//         // );
//     }

//     #[test]
//     fn test_block_id() {
//         assert_eq!(block_id( 0, 4, 3, Orientation::Up), None);
//         assert_eq!(block_id( 1, 4, 3, Orientation::Up), None);
//         assert_eq!(block_id( 2, 4, 3, Orientation::Up), None);
//         assert_eq!(block_id( 3, 4, 3, Orientation::Up), None);
//         assert_eq!(block_id( 4, 4, 3, Orientation::Up), Some(0));
//         assert_eq!(block_id( 5, 4, 3, Orientation::Up), Some(1));
//         assert_eq!(block_id( 6, 4, 3, Orientation::Up), Some(2));
//         assert_eq!(block_id( 7, 4, 3, Orientation::Up), Some(3));
//         assert_eq!(block_id( 8, 4, 3, Orientation::Up), Some(4));
//         assert_eq!(block_id( 9, 4, 3, Orientation::Up), Some(5));
//         assert_eq!(block_id(10, 4, 3, Orientation::Up), Some(6));
//         assert_eq!(block_id(11, 4, 3, Orientation::Up), Some(7));

//         assert_eq!(block_id( 0, 4, 3, Orientation::Down), Some( 4));
//         assert_eq!(block_id( 1, 4, 3, Orientation::Down), Some( 5));
//         assert_eq!(block_id( 2, 4, 3, Orientation::Down), Some( 6));
//         assert_eq!(block_id( 3, 4, 3, Orientation::Down), Some( 7));
//         assert_eq!(block_id( 4, 4, 3, Orientation::Down), Some( 8));
//         assert_eq!(block_id( 5, 4, 3, Orientation::Down), Some( 9));
//         assert_eq!(block_id( 6, 4, 3, Orientation::Down), Some(10));
//         assert_eq!(block_id( 7, 4, 3, Orientation::Down), Some(11));
//         assert_eq!(block_id( 8, 4, 3, Orientation::Down), None);
//         assert_eq!(block_id( 9, 4, 3, Orientation::Down), None);
//         assert_eq!(block_id(10, 4, 3, Orientation::Down), None);
//         assert_eq!(block_id(11, 4, 3, Orientation::Down), None);

//         assert_eq!(block_id( 0, 4, 3, Orientation::Left), None);
//         assert_eq!(block_id( 1, 4, 3, Orientation::Left), Some( 0));
//         assert_eq!(block_id( 2, 4, 3, Orientation::Left), Some( 1));
//         assert_eq!(block_id( 3, 4, 3, Orientation::Left), Some( 2));
//         assert_eq!(block_id( 4, 4, 3, Orientation::Left), None);
//         assert_eq!(block_id( 5, 4, 3, Orientation::Left), Some( 4));
//         assert_eq!(block_id( 6, 4, 3, Orientation::Left), Some( 5));
//         assert_eq!(block_id( 7, 4, 3, Orientation::Left), Some( 6));
//         assert_eq!(block_id( 8, 4, 3, Orientation::Left), None);
//         assert_eq!(block_id( 9, 4, 3, Orientation::Left), Some( 8));
//         assert_eq!(block_id(10, 4, 3, Orientation::Left), Some( 9));
//         assert_eq!(block_id(11, 4, 3, Orientation::Left), Some(10));

//         assert_eq!(block_id( 0, 4, 3, Orientation::Right), Some( 1));
//         assert_eq!(block_id( 1, 4, 3, Orientation::Right), Some( 2));
//         assert_eq!(block_id( 2, 4, 3, Orientation::Right), Some( 3));
//         assert_eq!(block_id( 3, 4, 3, Orientation::Right), None);
//         assert_eq!(block_id( 4, 4, 3, Orientation::Right), Some( 5));
//         assert_eq!(block_id( 5, 4, 3, Orientation::Right), Some( 6));
//         assert_eq!(block_id( 6, 4, 3, Orientation::Right), Some( 7));
//         assert_eq!(block_id( 7, 4, 3, Orientation::Right), None);
//         assert_eq!(block_id( 8, 4, 3, Orientation::Right), Some( 9));
//         assert_eq!(block_id( 9, 4, 3, Orientation::Right), Some(10));
//         assert_eq!(block_id(10, 4, 3, Orientation::Right), Some(11));
//         assert_eq!(block_id(11, 4, 3, Orientation::Right), None);
//     }

//     fn block_id(curr_block_id: u8, num_columns: u8, num_rows: u8, orientation: Orientation) -> Option<u8> {
//         let max = num_columns * num_rows;
    
//         let val: Option<u8>;
    
//         match orientation {
//             Orientation::Up => 
//                 if curr_block_id < num_columns { val = None; } else { val = Some(curr_block_id - num_columns); },
//             Orientation::Down =>
//                 if curr_block_id >= max - num_columns { val = None; } else { val = Some(curr_block_id + num_columns); },
//             Orientation::Left =>
//                 if curr_block_id % num_columns == 0 { val = None; } else { val = Some(curr_block_id - 1); },
//             Orientation::Right =>
//                 if curr_block_id % num_columns == num_columns - 1 { val = None; } else { val = Some(curr_block_id + 1); },
//         };
    
//         return val;
//     }
// }

