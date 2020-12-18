use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
// use petgraph::algo;
// use petgraph::algo::dijkstra;
use petgraph::algo::astar;
use enum_iterator::IntoEnumIterator;
use maplit::hashmap;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use std::fmt;

mod model;
use model::Orientation;
use model::Player;
use model::Color;
use model::Size;
use model::Unit;
use model::NeighbourIds;
use model::Board;
use model::NetworkNode;
use model::Block;

//look for hashing a hashmap in rust

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn block_id(curr_block_id: u8, num_columns: u8, num_rows: u8, orientation: Orientation) -> Option<u8> {
    let max = num_columns * num_rows;

    let val: Option<u8>;

    match orientation {
        Orientation::Up => 
            if curr_block_id < num_columns { val = None; } else { val = Some(curr_block_id - num_columns); },
        Orientation::Down =>
            if curr_block_id >= max - num_columns { val = None; } else { val = Some(curr_block_id + num_columns); },
        Orientation::Left =>
            if curr_block_id % num_columns == 0 { val = None; } else { val = Some(curr_block_id - 1); },
        Orientation::Right =>
            if curr_block_id % num_columns == num_columns - 1 { val = None; } else { val = Some(curr_block_id + 1); },
    };

    return val;
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
    let first_board_hash = calculate_hash(&first_board);

    let mut boards: HashMap<u64, Board> = hashmap!{};
    
    build(&first_board, &mut boards, &mut c.borrow_mut());

    let goals = goals(&boards, &c.borrow());

    if goals.len() > 0 {
        // println!("Solutions found for board: {:?}", first_board);
        println!("{} solutions found for board:", goals.len());
        print(&first_board);
        println!("---");

        for goal in goals {
            let board = &boards[&goal.hash_id];
            print(&board);
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

fn print(board: &Board) {
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

fn build(board: &Board, boards: &mut HashMap<u64, Board>, network: &mut UnGraphMap::<NetworkNode, ()>) {
    let board_hash = calculate_hash(&board);
    let board_node = NetworkNode{hash_id: board_hash};

    network.add_node(board_node);
    let board_clone = board.clone();
    let board_clone_hash = calculate_hash(&board_clone);
    assert_eq!(board_clone_hash, board_hash);
    boards.insert(board_hash, board.clone());
    // println!("{:?}", algo::dijkstra(c.borrow(), first_node, None, |_| 1));

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

fn can_win(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> bool {
    return goals(boards, network).len() > 0;
}

fn goals(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> Vec<NetworkNode> {
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

#[cfg(test)]
mod test;