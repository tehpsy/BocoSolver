use crate::*;

use std::{collections::HashMap, convert::TryInto};
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
use maplit::hashmap;
use petgraph::algo::astar;

use model::*;

use utils;
use hasher;

pub fn solve(board: Board) {
  let graph = UnGraphMap::<NetworkNode, ()>::new();
  let rc = RefCell::new(graph); 
  let c = Rc::new(rc);

  let first_board = board;
  let first_board_hash = hasher::calculate_hash(&first_board);
  let start = NetworkNode{hash_id: first_board_hash.try_into().unwrap()};

  let mut boards: HashMap<u64, Board> = hashmap!{};
  
  utils::build(&first_board, &mut boards, &mut c.borrow_mut());

  let goals = utils::goals(&boards, &c.borrow());
  
  println!("{} solutions found for board:", goals.len());
  utils::print(&first_board);
  println!("---");

  for goal in &goals {
      let board = &boards[&goal.hash_id];
      utils::print(&board);
      println!("---");

      let path = astar(
          &*c.borrow(),
          start,
          |n| n == *goal,
          |_| 1,
          |_| 1,
      );
  
      match path {
          Some((cost, path)) => {
              println!("The total cost was {}: {:?}", cost, path);
          }
          None => println!("There was no path"),
      }
  };
}

pub fn get_simplest_solution(board: &Board) -> Option<(i32, Vec<NetworkNode>, HashMap<u64, Board>)> {
    let graph = UnGraphMap::<NetworkNode, ()>::new();
    let rc = RefCell::new(graph); 
    let c = Rc::new(rc);
  
    let first_board = board;
    let first_board_hash = hasher::calculate_hash(&first_board);
    let start = NetworkNode{hash_id: first_board_hash.try_into().unwrap()};
  
    let mut boards: HashMap<u64, Board> = hashmap!{};
    
    utils::build(&first_board, &mut boards, &mut c.borrow_mut());
    // println!("{}", c.borrow().node_count());
    // println!("{}", boards.len());
    // let hash1: u64 = (&boards.keys())[0];
    // utils::print((&boards.keys().collect::<u64>())[0]);
    // utils::print(&boards[&1]);

    let goals = utils::goals(&boards, &c.borrow());
    let mut shortest_cost: Option<i32> = None;
    let mut shortest_path: Option<Vec<NetworkNode>> = None;
    // println!("{}", goals.len());

    for goal in &goals {
        let path = astar(
            &*c.borrow(),
            start,
            |n| n == *goal,
            |_| 1,
            |_| 1,
        );
    
        match path {
            Some((cost, path)) => {
                if shortest_cost == None || shortest_cost.unwrap() > cost {
                    shortest_cost = Some(cost);
                    shortest_path = Some(path);
                } 
            },
            None => (),
        }
    };

    if shortest_cost == None {
        return None
    } else {
        let nodes = shortest_path.unwrap();
        return Some((shortest_cost.unwrap(), nodes, boards))
    }
}

pub fn moves_for(boards: &HashMap<u64, Board>, nodes: &Vec<NetworkNode>) -> Option<Vec<Orientation>> {
    let mut moves: Vec<Orientation> = Vec::with_capacity(nodes.len());

    for index in 0..nodes.len()-1 {
        let this_node = nodes[index];
        let next_node = nodes[index+1];
        let this_board = boards.get(&this_node.hash_id);
        let next_board = boards.get(&next_node.hash_id);

        if let (Some(board1), Some(board2)) = (this_board, next_board) {
            if let Some(orientation) = board1.compare(board2) {
                moves.push(orientation);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    return Some(moves);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hasher::calculate_hash;
    
    #[test]
    fn no_moves_when_nodes_not_found() {
        let blocks = hashmap!{
            Position{x: 0, y: 0} => Block{ small: None, large: None, },
            Position{x: 1, y: 0} => Block{ small: None, large: None, },
        }; 
        let board1 = Board{player_pos: Position{x: 0, y: 0}, blocks: blocks.clone()};
        let board2 = Board{player_pos: Position{x: 1, y: 0}, blocks: blocks.clone()};
        let board1_hash = calculate_hash(&board1);
        let board2_hash = calculate_hash(&board2);
        let boards = hashmap!{
            board1_hash => board1,
            board2_hash => board2,
        };
        let nodes: Vec<NetworkNode> = vec![
            NetworkNode{hash_id:board1_hash},
            NetworkNode{hash_id:123}
        ];
        let moves = moves_for(&boards, &nodes);
        assert_eq!(moves, None);
    }

    #[test]
    fn calculates_moves_correctly() {
        let unit_left = Some(Unit{orientation: Orientation::Left, color: Color::Red});
        let unit_up = Some(Unit{orientation: Orientation::Up, color: Color::Red});
        let board1 = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: unit_left.clone(), },
                Position{x: 1, y: 0} => Block{ small: None, large: None, },
                Position{x: 2, y: 0} => Block{ small: None, large: None, },
                Position{x: 0, y: 1} => Block{ small: unit_up.clone(), large: None, },
                Position{x: 1, y: 1} => Block{ small: None, large: None, },
                Position{x: 2, y: 1} => Block{ small: None, large: None, },
            },
        };
        let board2 = Board{
            player_pos: Position{x: 1, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: None, },
                Position{x: 1, y: 0} => Block{ small: None, large: unit_left.clone(), },
                Position{x: 2, y: 0} => Block{ small: None, large: None, },
                Position{x: 0, y: 1} => Block{ small: unit_up.clone(), large: None, },
                Position{x: 1, y: 1} => Block{ small: None, large: None, },
                Position{x: 2, y: 1} => Block{ small: None, large: None, },
            },
        };
        let board3 = Board{
            player_pos: Position{x: 2, y: 0},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: None, },
                Position{x: 1, y: 0} => Block{ small: None, large: None, },
                Position{x: 2, y: 0} => Block{ small: None, large: unit_left.clone(), },
                Position{x: 0, y: 1} => Block{ small: unit_up.clone(), large: None, },
                Position{x: 1, y: 1} => Block{ small: None, large: None, },
                Position{x: 2, y: 1} => Block{ small: None, large: None, },
            },
        };
        let board4 = Board{
            player_pos: Position{x: 2, y: 1},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: None, },
                Position{x: 1, y: 0} => Block{ small: None, large: None, },
                Position{x: 2, y: 0} => Block{ small: None, large: None, },
                Position{x: 0, y: 1} => Block{ small: unit_up.clone(), large: None, },
                Position{x: 1, y: 1} => Block{ small: None, large: None, },
                Position{x: 2, y: 1} => Block{ small: None, large: unit_left.clone(), },
            },
        };
        let board5 = Board{
            player_pos: Position{x: 1, y: 1},
            blocks: board4.blocks.clone(),
        };
        let board6 = Board{
            player_pos: Position{x: 1, y: 0},
            blocks: board4.blocks.clone(),
        };
        let board7 = Board{
            player_pos: Position{x: 0, y: 0},
            blocks: board4.blocks.clone(),
        };
        let board8 = Board{
            player_pos: Position{x: 0, y: 1},
            blocks: board4.blocks.clone(),
        };
        let board9 = Board{
            player_pos: Position{x: 1, y: 1},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: None, },
                Position{x: 1, y: 0} => Block{ small: None, large: None, },
                Position{x: 2, y: 0} => Block{ small: None, large: None, },
                Position{x: 0, y: 1} => Block{ small: None, large: None, },
                Position{x: 1, y: 1} => Block{ small: unit_up.clone(), large: None, },
                Position{x: 2, y: 1} => Block{ small: None, large: unit_left.clone(), },
            },
        };
        let board10 = Board{
            player_pos: Position{x: 2, y: 1},
            blocks: hashmap!{
                Position{x: 0, y: 0} => Block{ small: None, large: None, },
                Position{x: 1, y: 0} => Block{ small: None, large: None, },
                Position{x: 2, y: 0} => Block{ small: None, large: None, },
                Position{x: 0, y: 1} => Block{ small: None, large: None, },
                Position{x: 1, y: 1} => Block{ small: None, large: None, },
                Position{x: 2, y: 1} => Block{ small: unit_up.clone(), large: unit_left.clone(), },
            },
        };
        let board1_hash = calculate_hash(&board1);
        let board2_hash = calculate_hash(&board2);
        let board3_hash = calculate_hash(&board3);
        let board4_hash = calculate_hash(&board4);
        let board5_hash = calculate_hash(&board5);
        let board6_hash = calculate_hash(&board6);
        let board7_hash = calculate_hash(&board7);
        let board8_hash = calculate_hash(&board8);
        let board9_hash = calculate_hash(&board9);
        let board10_hash = calculate_hash(&board10);
        let boards = hashmap!{
            board1_hash => board1,
            board2_hash => board2,
            board3_hash => board3,
            board4_hash => board4,
            board5_hash => board5,
            board6_hash => board6,
            board7_hash => board7,
            board8_hash => board8,
            board9_hash => board9,
            board10_hash => board10,
        };
        let nodes: Vec<NetworkNode> = vec![
            NetworkNode{hash_id:board1_hash},
            NetworkNode{hash_id:board2_hash},
            NetworkNode{hash_id:board3_hash},
            NetworkNode{hash_id:board4_hash},
            NetworkNode{hash_id:board5_hash},
            NetworkNode{hash_id:board6_hash},
            NetworkNode{hash_id:board7_hash},
            NetworkNode{hash_id:board8_hash},
            NetworkNode{hash_id:board9_hash},
            NetworkNode{hash_id:board10_hash},
        ];
        let moves = moves_for(&boards, &nodes);
        assert_eq!(moves, Some(vec![
            Orientation::Right,
            Orientation::Right,
            Orientation::Down,
            Orientation::Left,
            Orientation::Up,
            Orientation::Left,
            Orientation::Down,
            Orientation::Right,
            Orientation::Right,
        ]));
    }
}