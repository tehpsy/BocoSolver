use std::collections::HashMap;
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
use maplit::hashmap;
use std::hash::{Hash, Hasher};
// use petgraph::algo;
// use petgraph::algo::dijkstra;
// use petgraph::algo::astar;

mod model;
use model::*;

mod utils;

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

    // for num_columns in 0..3 {
    //     for num_rows in 0..3 {
        
    //     }   
    // }

    let first_board = Board{
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
    // let first_board_hash = calculate_hash(&first_board);

    let mut boards: HashMap<u64, Board> = hashmap!{};
    
    utils::build(&first_board, &mut boards, &mut c.borrow_mut());

    let goals = utils::goals(&boards, &c.borrow());

    if goals.len() > 0 {
        // println!("Solutions found for board: {:?}", first_board);
        println!("{} solutions found for board:", goals.len());
        utils::print(&first_board);
        println!("---");

        for goal in goals {
            let board = &boards[&goal.hash_id];
            utils::print(&board);
            println!("---");
        }
        

        // for goal in goals {
            // let goal_node = NetworkNode{hash_id: calculate_hash(&goal)};
            // 
            // let foo = UnGraphMap::<NetworkNode, ()>::new();
            // let res = dijkstra(
            //     &c.borrow().into_graph(),
            //     NetworkNode{hash_id: first_board_hash},
            //     Some(goal),
            //     |_| 1
            // );
            // let start_node = &c.borrow().nodes().iter().find(|node| => true);
            // let path = astar(
            //     UnGraphMap::<NetworkNode, ()>::new(),
            //     // &c.borrow().into_graph(),
            //     // petgraph::graph::NodeIndex(),
            //     NetworkNode{hash_id: first_board_hash},               // start
            //     |n| n == goal_node,      // is_goal
            //     |_| 1, // edge_cost
            //     |_| 0,           // estimate_cost
            // );
        
            // match path {
            //     Some((cost, path)) => {
            //         println!("The total cost was {}: {:?}", cost, path);
            //     }
            //     None => println!("There was no path"),
            // }
        // };

        // for goal in goals {
        //     let res = dijkstra(
        //         &c.borrow().into_graph(),
        //         NetworkNode{hash_id: first_board_hash},
        //         Some(goal),
        //         |_| 1
        //     );
    } else {
        println!("No solutions found");
    }
    

    // assert_eq!(can_win(&boards, & c.borrow()), true);
    
    // assert_eq!(algo::has_path_connecting(c.borrow(), first_node, first_node, None), true);
    // println!(algo::dijkstra(c.borrow(), first_node, goal: Option<G::NodeId>, 1))
    // println!("{:?}", algo::dijkstra(c.borrow(), first_node, None, |_| 1));
}

#[cfg(test)]
mod test;