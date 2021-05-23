use maplit::hashmap;
use crate::model::*;

pub fn hardest_boco_level() -> Board {
    return Board{
        player_pos: Position{x: 3, y: 1},
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
                large: Some(Unit{
                    orientation: Orientation::Down,
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
                    orientation: Orientation::Up,
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
                    orientation: Orientation::Up,
                    color: Color::Red,
                }),
                large: None,
            },
        }
    };
}

pub fn easiest_boco_level() -> Board {
    return Board{
        player_pos: Position{x: 0, y: 0},
        blocks: hashmap!{
            Position{x: 0, y: 0} => Block{
                small: Some(Unit{
                    orientation: Orientation::Left,
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
}