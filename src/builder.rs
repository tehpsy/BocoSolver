use crate::*;

pub fn build_hardest_boco_level() -> Board {
  return Board{
    player: Player{block_id: 8},
    blocks: hashmap!{
        0 => Block{
            small: None,
            large: None,
            id: 0,
            neighbour_ids: NeighbourIds::new(None, Some(5), None, Some(1))
        },
        1 => Block{
            small: None,
            large: None,
            id: 1,
            neighbour_ids: NeighbourIds::new(None, Some(6), Some(0), Some(2))
        },
        2 => Block{
            small: None,
            large: None,
            id: 2,
            neighbour_ids: NeighbourIds::new(None, Some(7), Some(1), Some(3))
        },
        3 => Block{
            small: None,
            large: Some(Unit{
                orientation: Orientation::Down,
                color: Color::Black,
            }),
            id: 3,
            neighbour_ids: NeighbourIds::new(None, Some(8), Some(2), Some(4))
        },
        4 => Block{
            small: None,
            large: None,
            id: 4,
            neighbour_ids: NeighbourIds::new(None, Some(9), Some(3), None)
        },
        5 => Block{
            small: None,
            large: None,
            id: 5,
            neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(6))
        },
        6 => Block{
            small: None,
            large: Some(Unit{
                orientation: Orientation::Up,
                color: Color::Red,
            }),
            id: 6,
            neighbour_ids: NeighbourIds::new(Some(1), None, Some(5), Some(7))
        },
        7 => Block{
            small: None,
            large: Some(Unit{
                orientation: Orientation::Left,
                color: Color::Black,
            }),
            id: 7,
            neighbour_ids: NeighbourIds::new(Some(2), None, Some(6), Some(8))
        },
        8 => Block{
            small: None,
            large: None,
            id: 8,
            neighbour_ids: NeighbourIds::new(Some(3), None, Some(7), Some(9))
        },
        9 => Block{
            small: Some(Unit{
                orientation: Orientation::Up,
                color: Color::Red,
            }),
            large: None,
            id: 9,
            neighbour_ids: NeighbourIds::new(Some(4), None, Some(8), None)
        },
    }
  };
}

pub fn build_easiest_boco_level() -> Board {
    return Board{
      player: Player{block_id: 0},
      blocks: hashmap!{
          0 => Block{
              small: Some(Unit{
                orientation: Orientation::Left,
                color: Color::Red,
              }),
              large: None,
              id: 0,
              neighbour_ids: NeighbourIds::new(None, None, None, Some(1))
          },
          1 => Block{
              small: None,
              large: Some(Unit{
                orientation: Orientation::Left,
                color: Color::Red,
              }),
              id: 1,
              neighbour_ids: NeighbourIds::new(None, None, Some(0), None)
          },
      }
    };
  }