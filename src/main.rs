use std::hash::Hash;
use std::collections::HashMap;
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
use petgraph::algo;
use enum_iterator::IntoEnumIterator;
use maplit::hashmap;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    id: char,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Player {
    block_id: u8,
}


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
enum Size {
    Small,
    Large,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Unit {
    orientation: Orientation,
    color: Color,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct NeighbourIds {
    up: Option<u8>,
    down: Option<u8>,
    left: Option<u8>,
    right: Option<u8>,
}

impl NeighbourIds {
    fn new(up: Option<u8>, down: Option<u8>, left: Option<u8>, right: Option<u8>) -> NeighbourIds {
        NeighbourIds{
            up: up,
            down: down,
            left: left,
            right: right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Block {
    small: Option<Unit>,
    large: Option<Unit>,
    id: u8,
    neighbour_ids: NeighbourIds,
}

// #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node2 {
    player: Player,
    blocks: HashMap<u8, Block>,
}

impl Node2 {
    fn available_moves(&self) -> Vec<Orientation> {
        return Orientation::into_enum_iter().filter(|orientation| {
            true
        }).collect();
    }

    // fn next_nodes(&self) -> Vec<Node2> {
    //     //where can player move?
    //     //for each move, do it
    //     //find what units have moved as a result
    // }

    fn is_win(&self) -> bool {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        return match (block.small, block.large) {
            (Some(small), Some(large)) => small.color == Color::Red && large.color == Color::Red,
            _ => false,
        };
    }
}

impl Node {
    fn next_nodes(&self) -> Vec<Node> {
        match self.id {
            'a' => return vec![Node{id: 'b'}],
            'b' => return vec![Node{id: 'a'}, Node{id: 'c'}, Node{id: 'd'}],
            'c' => return vec![Node{id: 'b'}, Node{id: 'f'}],
            'd' => return vec![Node{id: 'b'}, Node{id: 'e'}, Node{id: 'h'}],
            'e' => return vec![Node{id: 'd'}, Node{id: 'f'}],
            'f' => return vec![Node{id: 'c'}, Node{id: 'e'}, Node{id: 'g'}],
            'g' => return vec![Node{id: 'f'}],
            'h' => return vec![Node{id: 'd'}],
            _ => return vec![],
        };
    }

    fn is_win(&self) -> bool {
        return self.id == 'e' || self.id == 'h';
    }
}

// fn main() {
//     let mut graph = UnGraphMap::<_, ()>::new();
//     let first_node = Node{id:'a'};

//     graph.add_node(&first_node);
//     build(&first_node, &mut graph);

//     assert_eq!(graph.node_count(), 7);
// }

// fn build<'a>(node: &'a Node, network: &'a mut UnGraphMap::<&'a Node, ()>) {
//     node.next_nodes().iter().for_each(|next| {
//         if !network.contains_node(next) {
//             network.add_node(next);
//             build(next, network);
//         }

//         if !network.contains_edge(node, next) {
//             network.add_edge(node, next, ());
//         }
//     });
// }

fn main() {
    let graph = UnGraphMap::<Node, ()>::new();
    let rc = RefCell::new(graph); 
    let c = Rc::new(rc);

    let first_node = Node{id:'a'};

    c.borrow_mut().add_node(first_node);
    
    build(&first_node, &mut c.borrow_mut());

    assert_eq!(c.borrow().node_count(), 8);
    assert_eq!(c.borrow().edge_count(), 8);

    assert_eq!(can_win(& c.borrow()), true);
    // assert_eq!(algo::has_path_connecting(c.borrow(), first_node, first_node, None), true);
    // println!(algo::dijkstra(c.borrow(), first_node, goal: Option<G::NodeId>, 1))
    // println!("{:?}", algo::dijkstra(c.borrow(), first_node, None, |_| 1));
}

fn build(node: & Node, network: &mut UnGraphMap::<Node, ()>) {
    node.next_nodes().iter().for_each(|next| {
        
        if !network.contains_node(*next) {
            network.add_node(*next);
            build(next, network);
        }

        if !network.contains_edge(*node, *next) {
            network.add_edge(*node, *next, ());
        }
    });
}

fn can_win(network: & UnGraphMap::<Node, ()>) -> bool {
    match network.nodes().find(|node| { node.is_win() }) {
        Some(_) => return true,
        None => return false,
    }
}

// fn build<'a>(node: &'a Node, network: &'a mut UnGraphMap::<&'a Node, ()>) {
//     node.next_nodes().iter().for_each(|next| {
//         if !network.contains_node(next) {
//             network.add_node(next);
//             build(next, network);
//         }

//         if !network.contains_edge(node, next) {
//             network.add_edge(node, next, ());
//         }
//     });
// }



// use std::hash::Hash;
// use petgraph::graphmap::UnGraphMap;
// // use petgraph::graph::Graph;

// fn main() {
//     // let board = Board{};
//     // let mut history = Vec::new();
//     // history.push(board);
//     // let solutions = solve(history);

//     //we need to build the network first, then solve it
//     //otherwise different branches can cover the same nodes

//     let mut graph = UnGraphMap::<_, ()>::new();
//     let first_node = Board{foo:'a'};

//     graph.add_node(&first_node);
//     build(&first_node, &mut graph);
// }

// fn build<'a>(node: &'a Board, network: &'a mut UnGraphMap::<&'a Board, ()>) {
//     node.next_nodes().iter().for_each(|next| {
//         if !network.contains_node(next) {
//             network.add_node(next);
//             build(next, network);
//         }

//         if !network.contains_edge(node, next) {
//             network.add_edge(node, next, ());
//         }
//     });
// }

// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
// struct Board {
//     foo: char
// }

// impl Board {
//     fn next_nodes(&self) -> Vec<Board> {
//         match self.foo {
//             'a' => return vec![Board{foo: 'b'}],
//             'b' => return vec![Board{foo: 'a'}, Board{foo: 'c'}, Board{foo: 'd'}],
//             'c' => return vec![Board{foo: 'b'}, Board{foo: 'f'}],
//             'd' => return vec![Board{foo: 'b'}, Board{foo: 'e'}],
//             'e' => return vec![Board{foo: 'd'}, Board{foo: 'f'}],
//             'f' => return vec![Board{foo: 'c'}, Board{foo: 'e'}, Board{foo: 'g'}],
//             'g' => return vec![Board{foo: 'f'}],
//             _ => return vec![],
//         };
//     }
// }

// // type History = Vec<Board>;

// // fn solve(history: History) -> Vec<History> {
// //     let board = history.last().unwrap();
// //     let nextBoards = board.next(history.as_slice());
// //     let result = vec![];
// //     nextBoards.iter().for_each(|board| {
// //         let vec = vec![];
// //         vec.extend_from_slice(history.as_slice());
// //         vec.push(*board);
// //         result.append(&mut solve(vec));
// //     });

// //     return result;
// // }

// // impl Board {
// //     fn next(&self, history: &[Board]) -> Vec<Board> {
// //         /*
// //         let tiles = board.tiles
// //             .filter(hasBlocks)
// //             .filter(canAccess)
// //             .filter(playNotHere)
    
// //         return tiles
// //             .availableMoves
// //             .map(board(move: tile, direction: direction))
// //             .filter(notIn: history)        
// //         */
// //         return vec![];
// //     }
// // }

#[cfg(test)]
mod test;