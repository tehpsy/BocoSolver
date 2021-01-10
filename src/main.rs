use std::{collections::HashMap, convert::TryInto};
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
use maplit::hashmap;
use std::hash::{Hash, Hasher};
use petgraph::algo::astar;

mod model;
use model::*;

mod utils;

mod builder;

//look for hashing a hashmap in rust

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let graph = UnGraphMap::<NetworkNode, ()>::new();
    let rc = RefCell::new(graph); 
    let c = Rc::new(rc);

    let first_board = builder::build_hardest_boco_level();
    let first_board_hash = calculate_hash(&first_board);
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

#[cfg(test)]
mod test;