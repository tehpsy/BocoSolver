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

pub fn get_simplest_solution(board: &Board) -> Option<(i32, Vec<NetworkNode>)> {
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
        return Some((shortest_cost.unwrap(), shortest_path.unwrap()))
    }
  }
