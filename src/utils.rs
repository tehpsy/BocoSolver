use crate::model::*;
use crate::hasher::*;
use petgraph::graphmap::UnGraphMap;
use std::{collections::HashMap};

pub fn print(board: &Board) {
  let player_block_id = board.player.block_id;
  let mut keys = board.blocks.keys().cloned().collect::<Vec<u8>>();
  keys.sort();

  for key in keys {
      // let mut string = format!("{}: ", key);
      println!("{}", key);
      let block = board.blocks[&key];
      if key == player_block_id {
          // string += "player, ";
          println!("player ");
      }
      
      if block.small != None {
          match (block.small.unwrap().orientation, block.small.unwrap().color) {
              (Orientation::Up, Color::Black) => println!("small up black"),
              (Orientation::Down, Color::Black) => println!("small down black"),
              (Orientation::Left, Color::Black) => println!("small left black"),
              (Orientation::Right, Color::Black) => println!("small right black"),
              (Orientation::Up, Color::Red) => println!("small up red"),
              (Orientation::Down, Color::Red) => println!("small down red"),
              (Orientation::Left, Color::Red) => println!("small left red"),
              (Orientation::Right, Color::Red) => println!("small right red"),
          };
          // println!("{}", block.small.unwrap());
          // string += format!("{}, ", &block.small.unwrap().to_string().to_owned()[..]);
      }

      if block.large != None {
          match (block.large.unwrap().orientation, block.large.unwrap().color) {
              (Orientation::Up, Color::Black) => println!("large up black"),
              (Orientation::Down, Color::Black) => println!("large down black"),
              (Orientation::Left, Color::Black) => println!("large left black"),
              (Orientation::Right, Color::Black) => println!("large right black"),
              (Orientation::Up, Color::Red) => println!("large up red"),
              (Orientation::Down, Color::Red) => println!("large down red"),
              (Orientation::Left, Color::Red) => println!("large left red"),
              (Orientation::Right, Color::Red) => println!("large right red"),
          };
          // println!("{}", block.large.unwrap());
          // string += format!("large {}, ", block.large.unwrap());
      }

      // println!("{}", string);
  }
}

pub fn build(board: &Board, boards: &mut HashMap<u64, Board>, network: &mut UnGraphMap::<NetworkNode, ()>) {
  let board_hash = calculate_hash(&board);
  let board_node = NetworkNode{hash_id: board_hash};

  network.add_node(board_node);
  boards.insert(board_hash, board.clone());

  if board.is_win() { return; }

  board.next_boards().iter().for_each(|next| {
      let next_node = NetworkNode{hash_id: calculate_hash(&next)};

      if !network.contains_node(next_node) {
          build(&next.clone(), boards, network);
      }

      if !network.contains_edge(board_node, next_node) {
          network.add_edge(board_node, next_node, ());
      }
  });
}

pub fn can_win(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> bool {
  return goals(boards, network).len() > 0;
}

pub fn goals(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> Vec<NetworkNode> {
  return 
      network.nodes()
      .filter(|node| { 
          let hash_id = node.hash_id;
          let board = &boards[&hash_id];
          return board.is_win();
      })
      .into_iter()
      // .map(|network_node| { return network_node.hash_id; })
      .collect();
}
