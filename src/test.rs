use super::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_available_moves() {
    //     let node2 = Node2{
    //         height:2,
    //         width:2,
    //         player:Player{position:Position{x:0, y:0}},
    //         reds:Reds{
    //             small:Unit{position:Position{x:0, y:1}, orientation:Orientation::Up, color:Color::Red, size:Size::Small},
    //             large:Unit{position:Position{x:1, y:1}, orientation:Orientation::Left, color:Color::Red, size:Size::Large},
    //         }
    //     };
    
    //     let available_moves = node2.available_moves();

    //     assert_eq!(available_moves, [Orientation::Right, Orientation::Down]);
    // }

    #[test]
    fn test_no_win_when_no_units() {
        assert_eq!(
            Node2{
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
            Node2{
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
            Node2{
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
            Node2{
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
            Node2{
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
}