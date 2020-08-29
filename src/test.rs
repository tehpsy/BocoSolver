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
    fn test_index_to_position() {
        fn make_node(width: u8, height: u8) -> Node2 {
            return Node2{
                width:width,
                height:height,
                player:Player{position:Position{x:0, y:0}},
                blocks:vec![]
            };
        }
        
        assert_eq!(make_node(2, 3).position_from(4), Position{x:0, y:2});
        assert_eq!(make_node(2, 3).position_from(0), Position{x:0, y:0});
        assert_eq!(make_node(2, 3).position_from(5), Position{x:1, y:2});
        assert_eq!(make_node(2, 3).index_from(&Position{x:0, y:2}), 4);
        assert_eq!(make_node(2, 3).index_from(&Position{x:0, y:0}), 0);
        assert_eq!(make_node(2, 3).index_from(&Position{x:1, y:2}), 5);
    }

    #[test]
    fn test_no_win_when_no_units() {
        assert_eq!(
            Node2{
                width:2,
                height:2,
                player:Player{position:Position{x:0, y:0}},
                blocks:vec![
                    Block{small:None, large:None, x:0, y:0},
                    Block{small:None, large:None, x:1, y:0},
                    Block{small:None, large:None, x:0, y:1},
                    Block{small:None, large:None, x:1, y:1},
                ]
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_units_match_but_only_one_is_red() {
        assert_eq!(
            Node2{
                width:2,
                height:2,
                player:Player{position:Position{x:0, y:0}},
                blocks:vec![
                    Block{small: None, large: None, x:0, y:0},
                    Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Black,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        x:1,
                        y:0,
                    },
                    Block{small: None, large: None, x:0, y:1},
                    Block{small: None, large: None, x:1, y:1},
                ]
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_different_locations() {
        assert_eq!(
            Node2{
                width:2,
                height:2,
                player:Player{position:Position{x:0, y:0}},
                blocks:vec![
                    Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: None,
                        x:0,
                        y:0,
                    },
                    Block{
                        small: None,
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        x:1,
                        y:0,
                    },
                    Block{small: None, large: None, x:0, y:1},
                    Block{small: None, large: None, x:1, y:1},
                ]
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_no_win_when_red_units_at_same_location_but_player_is_not() {
        assert_eq!(
            Node2{
                width:2,
                height:2,
                player:Player{position:Position{x:0, y:0}},
                blocks:vec![
                    Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: None,
                        x:1,
                        y:0,
                    },
                    Block{
                        small: None,
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        x:1,
                        y:0,
                    },
                    Block{small: None, large: None, x:0, y:1},
                    Block{small: None, large: None, x:1, y:1},
                ]
            }.is_win(),
            false,
        );
    }

    #[test]
    fn test_win_when_red_units_at_same_location_as_player() {
        assert_eq!(
            Node2{
                width:2,
                height:2,
                player:Player{position:Position{x:1, y:0}},
                blocks:vec![
                    Block{small: None, large: None, x:0, y:1},
                    Block{
                        small: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        large: Some(Unit{
                            orientation: Orientation::Up,
                            color: Color::Red,
                        }),
                        x:1,
                        y:0,
                    },
                    Block{small: None, large: None, x:0, y:1},
                    Block{small: None, large: None, x:1, y:1},
                ]
            }.is_win(),
            true,
        );
    }
}