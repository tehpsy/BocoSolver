use std::{collections::HashMap};
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

    let mut boards: HashMap<u64, Board> = hashmap!{};
    
    utils::build(&first_board, &mut boards, &mut c.borrow_mut());

    let goals = utils::goals(&boards, &c.borrow());

    if goals.len() > 0 {
        println!("{} solutions found for board:", goals.len());
        utils::print(&first_board);
        println!("---");

        // for goal in goals {
        //     let board = &boards[&goal.hash_id];
        //     utils::print(&board);
        //     println!("---");
        // }
        
        for goal in goals {
            let goal_hash = calculate_hash(&goal);
            let goal_node = NetworkNode{hash_id: goal_hash};
            // let goal_node_index = petgraph::graph::NodeIndex::new(goal_hash.try_into().unwrap());
            let foo = UnGraphMap::<NetworkNode, ()>::new();
            // let start_node = &c.borrow().nodes().iter().find(|node| => true);
            // let graph2 = Graph::<NetworkNode, ()>::new();
            let path = astar(
                &graph,
                // &c.borrow(),
                // &rc.borrow().into_graph(),
                NetworkNode{hash_id: first_board_hash},//petgraph::graph::NodeIndex::new(first_board_hash.try_into().unwrap()),
                |n| n == goal_node,
                |_| 1,
                |_| 0,
            );
        
            match path {
                Some((cost, path)) => {
                    println!("The total cost was {}: {:?}", cost, path);
                }
                None => println!("There was no path"),
            }
        };
    } else {
        println!("No solutions found");
    }
}

#[cfg(test)]
mod test;