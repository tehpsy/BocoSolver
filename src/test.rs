use super::*;
use maplit::hashset;
use utils;
use enum_iterator::IntoEnumIterator;
use model::*;
use maplit::hashmap;
use super::*;
use std::collections::HashMap;
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_moves_when_no_units_on_board() {
        let two_tiles = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None},
                Position{x: 1, y: 0} => Block{small: None, large: None},
            }
        };
        assert_eq!(two_tiles.available_moves(), hashset![Orientation::Right]);
        assert_eq!(Board{player_pos: Position{x: 1, y: 0}, ..two_tiles}.available_moves(), hashset![Orientation::Left]);

        let single_tile = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None},
            }
        };
        assert_eq!(single_tile.available_moves(), hashset![]);

        let complex_layout = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{small: None, large: None},
                Position{x: 1, y: 0} => Block{small: None, large: None},
                Position{x: 2, y: 0} => Block{small: None, large: None},
                Position{x: 1, y: 1} => Block{small: None, large: None},
                Position{x: 2, y: 1} => Block{small: None, large: None},
            }
        };

        assert_eq!(Board{player_pos: Position{x: 0, y: 0}, ..complex_layout.clone()}.available_moves(), hashset![Orientation::Right]);
        assert_eq!(Board{player_pos: Position{x: 1, y: 0}, ..complex_layout.clone()}.available_moves(), hashset![Orientation::Right, Orientation::Left, Orientation::Up]);
        assert_eq!(Board{player_pos: Position{x: 2, y: 0}, ..complex_layout.clone()}.available_moves(), hashset![Orientation::Left, Orientation::Up]);
        assert_eq!(Board{player_pos: Position{x: 1, y: 1}, ..complex_layout.clone()}.available_moves(), hashset![Orientation::Right, Orientation::Down]);
        assert_eq!(Board{player_pos: Position{x: 2, y: 1}, ..complex_layout.clone()}.available_moves(), hashset![Orientation::Left, Orientation::Down]);
    }

    #[test]
    fn test_available_moves_when_units_on_board() {
        fn create_3_x_3_layout(player_pos: Position, center_unit_orientation: Orientation) -> Board {
            return Board{
                player_pos,
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                    Position{x: 2, y: 0} => Block{small: None, large: None},
                    Position{x: 0, y: 1} => Block{small: None, large: None},
                    Position{x: 1, y: 1} => Block{small: Some(Unit{
                        orientation: center_unit_orientation,
                        color: Color::Red,
                    }), large: None},
                    Position{x: 2, y: 1} => Block{small: None, large: None},
                    Position{x: 0, y: 2} => Block{small: None, large: None},
                    Position{x: 1, y: 2} => Block{small: None, large: None},
                    Position{x: 2, y: 2} => Block{small: None, large: None},
                }
            }
        }

        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 0}, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 0}, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Up]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 0}, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 0}, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 0, y: 1}, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 0, y: 1}, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 0, y: 1}, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 0, y: 1}, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 2, y: 1}, Orientation::Up).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 2, y: 1}, Orientation::Down).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 2, y: 1}, Orientation::Left).available_moves(), hashset![Orientation::Up, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 2, y: 1}, Orientation::Right).available_moves(), hashset![Orientation::Up, Orientation::Down, Orientation::Left]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 2}, Orientation::Up).available_moves(), hashset![Orientation::Left, Orientation::Right, Orientation::Down]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 2}, Orientation::Down).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 2}, Orientation::Left).available_moves(), hashset![Orientation::Left, Orientation::Right]);
        assert_eq!(create_3_x_3_layout(Position{x: 1, y: 2}, Orientation::Right).available_moves(), hashset![Orientation::Left, Orientation::Right]);
    }

    #[test]
    fn test_can_move_into_block_only_when_all_block_units_align() {
        fn create_3_x_3_layout(player_pos: Position, orientation_small: Orientation, orientation_large: Orientation) -> Board {
            return Board{
                player_pos,
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                    Position{x: 2, y: 0} => Block{small: None, large: None},
                    Position{x: 0, y: 1} => Block{small: None, large: None},
                    Position{x: 1, y: 1} => Block{small: Some(Unit{
                        orientation: orientation_small,
                        color: Color::Red,
                    }), 
                    large: Some(Unit{
                        orientation: orientation_large,
                        color: Color::Red,
                    })},
                    Position{x: 2, y: 1} => Block{small: None, large: None},
                    Position{x: 0, y: 2} => Block{small: None, large: None},
                    Position{x: 1, y: 2} => Block{small: None, large: None},
                    Position{x: 2, y: 2} => Block{small: None, large: None},
                }
            }
        }

        fn assert(player_pos: Position, available_orientation: Orientation) {
            Orientation::into_enum_iter().for_each(|orientation_small| {
                Orientation::into_enum_iter().for_each(|orientation_large| {
                    let assert_val =
                        available_orientation.opposite() == orientation_small &&
                        available_orientation.opposite() == orientation_large;

                    assert_eq!(
                        create_3_x_3_layout(player_pos, orientation_small, orientation_large).available_moves().contains(&available_orientation),
                        assert_val
                    );
                })
            })
        }

        assert(Position{x: 1, y: 0}, Orientation::Up);
        assert(Position{x: 0, y: 1}, Orientation::Right);
        assert(Position{x: 2, y: 1}, Orientation::Left);
        assert(Position{x: 1, y: 2}, Orientation::Down);
    }

    #[test]
    fn test_only_one_unit_type_allowed_per_block() {
        {
            let board = Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
                        large: None,
                    },
                    Position{x: 1, y: 0} => Block{
                        small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        large: None,
                    },
                }
            };
            assert!(board.available_moves().is_empty());
        }

        {
            let board = Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Up, color: Color::Red}),
                    },
                    Position{x: 1, y: 0} => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                    },
                }
            };
            assert!(board.available_moves().is_empty());
        }
    }

    #[test]
    fn test_moves_not_limited_by_unit_type_if_current_unit_does_not_move() {
        {
            let board = Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
                        large: None,
                    },
                    Position{x: 1, y: 0} => Block{
                        small: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                        large: None,
                    },
                }
            };
            assert!(board.available_moves().contains(&Orientation::Right));
        }

        {
            let board = Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Right, color: Color::Red}),
                    },
                    Position{x: 1, y: 0} => Block{
                        small: None,
                        large: Some(Unit{orientation: Orientation::Left, color: Color::Red}),
                    },
                }
            };
            assert!(board.available_moves().contains(&Orientation::Right));
        }
    }

    #[test]
    fn test_no_win_when_no_units() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_units_match_location_but_only_one_is_red() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Black,
                        }),
                    },
                    Position{x: 1, y: 0} => Block{
                        small: None,
                        large: None,
                    },
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_different_locations() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: None,
                    },
                    Position{x: 1, y: 0} => Block{
                        small: None,
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                    },
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_same_location_but_player_is_not() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: None,
                        large: None,
                    },
                    Position{x: 1, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                    },
                }
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_win_when_red_units_at_same_location_as_player() {
        assert_eq!(
            Board{
                player_pos: Position{x: 1, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: None,
                        large: None,
                    },
                    Position{x: 1, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                    },
                }
            }.is_win(),
            true,
        );
    }

    #[test]
    fn test_moving_moves_player() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }.moving(Orientation::Right),
            Board{
                player_pos: Position{x: 1, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }
        );
    }

    #[test]
    fn test_moving_moves_units() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }), large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }.moving(Orientation::Right),
            Board{
                player_pos: Position{x: 1, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }), large: None},
                }
            }
        );

        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    })},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }.moving(Orientation::Right),
            Board{
                player_pos: Position{x: 1, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{small: None, large: None},
                    Position{x: 1, y: 0} => Block{small: None, large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    })},
                }
            }
        );
    }

    #[test]
    fn test_units_do_not_move_if_orientation_matches_direction_of_movement() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        })},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }.moving(Orientation::Right),
            Board{
                player_pos: Position{x: 1, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Right,
                            color: Color::Red,
                        })},
                    Position{x: 1, y: 0} => Block{small: None, large: None},
                }
            }
        );
    }

    #[test]
    fn test_board_building() {
        let graph = UnGraphMap::<NetworkNode, ()>::new();
        let rc = RefCell::new(graph); 
        let c = Rc::new(rc);

        let first_board = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }),
                    large: None,
                },
                Position{x: 1, y: 0} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    }),
                },
            }
        };
        let mut boards: HashMap<u64, Board> = hashmap!{};
        
        utils::build(&first_board, &mut boards, &mut c.borrow_mut());

        assert_eq!(c.borrow().node_count(), 2);
        assert_eq!(c.borrow().edge_count(), 1);
        assert_eq!(boards.len(), c.borrow().node_count());
        assert_eq!(can_win(&boards, & c.borrow()), true);
    }

    fn can_win(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> bool {
        return utils::goals(boards, network).len() > 0;
    }

    #[test]
    fn test_board_building_2() {
        let graph = UnGraphMap::<NetworkNode, ()>::new();
        let rc = RefCell::new(graph); 
        let c = Rc::new(rc);

        let first_board = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Down,
                        color: Color::Red,
                    }),
                    large: None,
                },
                Position{x: 1, y: 0} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Red,
                    }),
                },
                Position{x: 0, y: 1} => Block{
                    small: None,
                    large: None,
                },
                Position{x: 1, y: 1} => Block{
                    small: None,
                    large: None,
                },
            }
        };

        let mut boards: HashMap<u64, Board> = hashmap!{};
        
        utils::build(&first_board, &mut boards, &mut c.borrow_mut());

        assert_eq!(c.borrow().node_count(), 7);
        assert_eq!(c.borrow().edge_count(), 6);
        assert_eq!(boards.len(), c.borrow().node_count());
        assert_eq!(can_win(&boards, & c.borrow()), true);
    }

    #[test]
    fn test_board_building_3() {
        let graph = UnGraphMap::<NetworkNode, ()>::new();
        let rc = RefCell::new(graph); 
        let c = Rc::new(rc);

        let first_board = Board{
            player_pos: Position{x: 3, y: 1},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: None,
                    large: None
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
                    large: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Black,
                    }),
                },
                Position{x: 4, y: 0} => Block{
                    small: None,
                    large: None,
                },
                Position{x: 0, y: 1} => Block{
                    small: None,
                    large: None,
                },
                Position{x: 1, y: 1} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Down,
                        color: Color::Red,
                    }),
                },
                Position{x: 2, y: 1} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Black,
                    }),
                },
                Position{x: 3, y: 1} => Block{
                    small: None,
                    large: None,
                },
                Position{x: 4, y: 1} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Down,
                        color: Color::Red,
                    }),
                    large: None,
                },
            }
        };

        // let last_board = Board{
        //     player_pos: Position{x: 1, y: 1},
        //     blocks: hashmap!{
        //         Position{x: 0, y: 0} => Block{
        //             small: None,
        //             large: Some(Unit{
        //                 orientation: Orientation::Down,
        //                 color: Color::Black,
        //             }),
        //         },
        //         Position{x: 1, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 2, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 3, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 4, y: 0} => Block{
        //             small: None,
        //             large: Some(Unit{
        //                 orientation: Orientation::Left,
        //                 color: Color::Black,
        //             }),
        //         },
        //         Position{x: 0, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 1, y: 1} => Block{
        //             small: Some(Unit{
        //                 orientation: Orientation::Up,
        //                 color: Color::Red,
        //             }),
        //             large: Some(Unit{
        //                 orientation: Orientation::Up,
        //                 color: Color::Red,
        //             }),
        //         },
        //         Position{x: 2, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 3, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 4, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //     }
        // };

        // let intermediate_board = Board{
        //     player_pos: Position{x: 3, y: 1},
        //     blocks: hashmap!{
        //         Position{x: 0, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 1, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 2, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 3, y: 0} => Block{
        //             small: None,
        //             large: Some(Unit{
        //                 orientation: Orientation::Down,
        //                 color: Color::Black,
        //             }),
        //         },
        //         Position{x: 4, y: 0} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 0, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 1, y: 1} => Block{
        //             small: None,
        //             large: Some(Unit{
        //                 orientation: Orientation::Up,
        //                 color: Color::Red,
        //             }),
        //         },
        //         Position{x: 2, y: 1} => Block{
        //             small: None,
        //             large: Some(Unit{
        //                 orientation: Orientation::Left,
        //                 color: Color::Black,
        //             }),
        //         },
        //         Position{x: 3, y: 1} => Block{
        //             small: None,
        //             large: None,
        //         },
        //         Position{x: 4, y: 1} => Block{
        //             small: Some(Unit{
        //                 orientation: Orientation::Up,
        //                 color: Color::Red,
        //             }),
        //             large: None,
        //         },
        //     }
        // };

        let mut boards: HashMap<u64, Board> = hashmap!{};
        // let intermediate_board_hash = calculate_hash(&intermediate_board);

        // let first_hash_1 = calculate_hash(&first_board);
        // let first_hash_2 = calculate_hash(&first_board);
        // let last_hash_1 = calculate_hash(&last_board);
        // let last_hash_2 = calculate_hash(&last_board);

        utils::build(&first_board, &mut boards, &mut c.borrow_mut());

        // assert_eq!(first_hash_1, first_hash_2);
        // assert_eq!(last_hash_1, last_hash_2);
        // assert_eq!(first_hash_1, last_hash_2);


        // let node_count = c.borrow().node_count();
        // let edge_count = c.borrow().edge_count();
        // println!("{}", node_count);
        // println!("{}", boards.len());
        // println!("{}", edge_count);
        // println!("{}", first_board_hash);
        // println!("{}", intermediate_board_hash);
        
        // assert_eq!(boards.contains_key(&last_board_hash), true);
        
        // assert_eq!(boards.contains_key(&first_board_hash), true);
        
        // assert_eq!(false, true);

        // assert_eq!(boards.contains_key(&intermediate_board_hash), true);
        // assert_eq!(c.borrow().node_count(), 14);
        // assert_eq!(c.borrow().edge_count(), 13);
        assert_eq!(can_win(&boards, & c.borrow()), true);
    }

    #[test]
    fn test_large_unit_cannot_pass_through_small_unit() {
        let board = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Black,
                    }),
                },
                Position{x: 0, y: 1} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Red,
                    }),
                    large: None,
                },
            }
        };

        assert_eq!(board.available_moves(), hashset![]);
    }

    #[test]
    fn test_available_moves_when_moving_from_large_unit_to_small_unit() {
        let board = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: None,
                    large: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Black,
                    }),
                },
                Position{x: 0, y: 1} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Down,
                        color: Color::Red,
                    }),
                    large: None,
                },
            }
        };

        assert_eq!(board.available_moves(), hashset![Orientation::Up]);
    }

    #[test]
    fn test_available_moves_when_two_units_with_different_orientations() {
        let board = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{
                    small: Some(Unit{
                        orientation: Orientation::Up,
                        color: Color::Black,
                    }),
                    large: Some(Unit{
                        orientation: Orientation::Left,
                        color: Color::Black,
                    }),
                },
                Position{x: 0, y: 1} => Block{
                    small: None,
                    large: None,
                },
            }
        };

        assert_eq!(board.available_moves(), hashset![Orientation::Up]);
    }

    #[test]
    fn test_moving_when_two_units_with_different_orientations() {
        assert_eq!(
            Board{
                player_pos: Position{x: 0, y: 0},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Down,
                            color: Color::Black,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Left,
                            color: Color::Black,
                        }),
                    },
                    Position{x: 0, y: 1} => Block{
                        small: None,
                        large: None,
                    },
                }
            }.moving(Orientation::Up),
            Board{
                player_pos: Position{x: 0, y: 1},
                blocks: hashmap!{
                    Position{x: 0, y: 0} => Block{
                        small: None,
                        large: None,
                    },
                    Position{x: 0, y: 1} => Block{
                        small: Some(Unit{
                            orientation: Orientation::Down,
                            color: Color::Black,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Left,
                            color: Color::Black,
                        }),
                    },
                }
            }
        );
    }
}

