use super::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_available_moves_when_no_units_on_board() {
        let two_tiles = Node{
            player: Player{block_id: 0},
            blocks: hashmap!{
                0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
            }
        };
        assert_eq!(two_tiles.available_moves(), hashset![Orientation::Right]);
        assert_eq!(Node{player: Player{block_id: 1}, ..two_tiles}.available_moves(), hashset![Orientation::Left]);

        let single_tile = Node{
            player: Player{block_id: 0},
            blocks: hashmap!{
                0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, None)},
            }
        };
        assert_eq!(single_tile.available_moves(), hashset![]);

        fn create_complex_layout(block_id: u8) -> Node {
            return Node{
                player: Player{block_id: block_id},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), Some(2))},
                    2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(4), Some(1), None)},
                    3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(1), None, None, Some(4))},
                    4 => Block{small: None, large: None, id: 4, neighbour_ids: NeighbourIds::new(Some(2), None, Some(3), None)},
                }
            }
        }

        assert_eq!(create_complex_layout(0).available_moves(), hashset![Orientation::Right]);
        assert_eq!(create_complex_layout(1).available_moves(), hashset![Orientation::Right, Orientation::Left, Orientation::Down]);
        assert_eq!(create_complex_layout(2).available_moves(), hashset![Orientation::Left, Orientation::Down]);
        assert_eq!(create_complex_layout(3).available_moves(), hashset![Orientation::Right, Orientation::Up]);
        assert_eq!(create_complex_layout(4).available_moves(), hashset![Orientation::Left, Orientation::Up]);
    }

    #[test]
    fn test_available_moves_when_units_on_board() {
        fn create_3_x_3_layout(block_id: u8, center_unit_orientation: Orientation) -> Node {
            return Node{
                player: Player{block_id: block_id},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(4), Some(0), Some(2))},
                    2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(5), Some(1), None)},
                    3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(0), Some(6), None, Some(4))},
                    4 => Block{small: Some(Unit{
                        orientation: center_unit_orientation,
                        color: Color::Red,
                    }), large: None, id: 4, neighbour_ids: NeighbourIds::new(Some(1), Some(7), Some(3), Some(5))},
                    5 => Block{small: None, large: None, id: 5, neighbour_ids: NeighbourIds::new(Some(2), Some(8), Some(4), None)},
                    6 => Block{small: None, large: None, id: 6, neighbour_ids: NeighbourIds::new(Some(3), None, None, Some(7))},
                    7 => Block{small: None, large: None, id: 7, neighbour_ids: NeighbourIds::new(Some(4), None, Some(6), Some(8))},
                    8 => Block{small: None, large: None, id: 8, neighbour_ids: NeighbourIds::new(Some(5), None, Some(7), None)},
                }
            }
        }

        assert_eq!(create_3_x_3_layout(1, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(1, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(1, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(1, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(3, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(3, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(3, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(3, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(5, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(5, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(5, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(5, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Left]);
        assert_eq!(create_3_x_3_layout(7, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(7, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Up]);
        assert_eq!(create_3_x_3_layout(7, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(7, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
    }

    #[test]
    fn test_can_move_into_block_only_when_all_block_units_align() {
        fn create_3_x_3_layout(block_id: u8, orientation_small: Orientation, orientation_large: Orientation) -> Node {
            return Node{
                player: Player{block_id: block_id},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, Some(3), None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, Some(4), Some(0), Some(2))},
                    2 => Block{small: None, large: None, id: 2, neighbour_ids: NeighbourIds::new(None, Some(5), Some(1), None)},
                    3 => Block{small: None, large: None, id: 3, neighbour_ids: NeighbourIds::new(Some(0), Some(6), None, Some(4))},
                    4 => Block{small: Some(Unit{
                        orientation: orientation_small,
                        color: Color::Red,
                    }), 
                    large: Some(Unit{
                        orientation: orientation_large,
                        color: Color::Red,
                    }), id: 4, neighbour_ids: NeighbourIds::new(Some(1), Some(7), Some(3), Some(5))},
                    5 => Block{small: None, large: None, id: 5, neighbour_ids: NeighbourIds::new(Some(2), Some(8), Some(4), None)},
                    6 => Block{small: None, large: None, id: 6, neighbour_ids: NeighbourIds::new(Some(3), None, None, Some(7))},
                    7 => Block{small: None, large: None, id: 7, neighbour_ids: NeighbourIds::new(Some(4), None, Some(6), Some(8))},
                    8 => Block{small: None, large: None, id: 8, neighbour_ids: NeighbourIds::new(Some(5), None, Some(7), None)},
                }
            }
        }

        fn assert(block_id: u8, available_orientation: Orientation) {
            Orientation::into_enum_iter().for_each(|orientation_small| {
                Orientation::into_enum_iter().for_each(|orientation_large| {
                    let assert_val =
                        available_orientation.opposite() == orientation_small &&
                        available_orientation.opposite() == orientation_large;

                    assert_eq!(
                        create_3_x_3_layout(block_id, orientation_small, orientation_large).available_moves().contains(&available_orientation),
                        assert_val
                    );
                })
            })
        }

        assert(1, Orientation::Down);
        assert(3, Orientation::Right);
        assert(5, Orientation::Left);
        assert(7, Orientation::Up);
    }

    #[test]
    fn test_only_one_unit_type_allowed_per_block() {
        {
            let node = Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
                        large: None,
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        large: None,
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            };
            assert!(node.available_moves().is_empty());
        }

        {
            let node = Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            };
            assert!(node.available_moves().is_empty());
        }
    }

    #[test]
    fn test_moves_not_limited_by_unit_type_if_current_unit_does_not_move() {
        {
            let node = Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
                        large: None,
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        large: None,
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            };
            assert!(node.available_moves().contains(&Orientation::Right));
        }

        {
            let node = Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            };
            assert!(node.available_moves().contains(&Orientation::Right));
        }
    }

    #[test]
    fn test_no_win_when_no_units() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_units_match_location_but_only_one_is_red() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Black,
                        }),
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
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_different_locations() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: None,
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: None,
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_same_location_but_player_is_not() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: None,
                        large: None,
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_win_when_red_units_at_same_location_as_player() {
        assert_eq!(
            Node{
                player: Player{block_id: 1},
                blocks: hashmap!{
                    0 => Block{
                        small: None,
                        large: None,
                        id: 0,
                        neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
                    },
                    1 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        id: 1,
                        neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
                    },
                }
            }.is_win(),
            true,
        );
    }

    #[test]
    fn test_moving_moves_player() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }.moving(Orientation::Right),
            Node{
                player: Player{block_id: 1},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }
        );
    }

    #[test]
    fn test_moving_moves_units() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }), large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }.moving(Orientation::Right),
            Node{
                player: Player{block_id: 1},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }), large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }
        );

        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{small: None, large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }.moving(Orientation::Right),
            Node{
                player: Player{block_id: 1},
                blocks: hashmap!{
                    0 => Block{small: None, large: None, id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    }), id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }
        );
    }

    #[test]
    fn test_units_do_not_move_if_orientation_matches_direction_of_movement() {
        assert_eq!(
            Node{
                player: Player{block_id: 0},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }.moving(Orientation::Right),
            Node{
                player: Player{block_id: 1},
                blocks: hashmap!{
                    0 => Block{
                        small: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }), id: 0, neighbour_ids: NeighbourIds::new(None, None, None, Some(1))},
                    1 => Block{small: None, large: None, id: 1, neighbour_ids: NeighbourIds::new(None, None, Some(0), None)},
                }
            }
        );
    }
}