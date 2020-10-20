use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;
use std::rc::Rc;
use std::cell::RefCell;
// use petgraph::algo;
use enum_iterator::IntoEnumIterator;
use maplit::hashmap;
use maplit::hashset;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

impl Orientation {
    pub fn opposite(&self) -> Orientation {
        match self {
            Orientation::Up => return Orientation::Down,
            Orientation::Down => return Orientation::Up,
            Orientation::Left => return Orientation::Right,
            Orientation::Right => return Orientation::Left,
        }
    }
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Unit {
    orientation: Orientation,
    color: Color,
}

//use builder pattern to init the values -- default trait
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct NeighbourIds {
    up: Option<u8>,
    down: Option<u8>,
    left: Option<u8>,
    right: Option<u8>,
}

impl NeighbourIds {
    fn new(up: Option<u8>, down: Option<u8>, left: Option<u8>, right: Option<u8>) -> NeighbourIds {
        NeighbourIds{
            up,
            down,
            left,
            right,
        }
    }

    fn neighbour_towards(&self, orientation: &Orientation) -> Option<u8> {
        match orientation {
            Orientation::Up => return self.up,
            Orientation::Down => return self.down,
            Orientation::Left => return self.left,
            Orientation::Right => return self.right,
        }
    }
}

//use a vec of references to other 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Block {
    small: Option<Unit>,
    large: Option<Unit>,
    id: u8,
    neighbour_ids: NeighbourIds,
}

#[derive(Copy, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
struct NetworkNode {
    hash_id: u64,
}

//look for hashing a hashmap in rust

#[derive(Clone, Debug, Eq, PartialEq)]
struct Board {
    player: Player,
    blocks: HashMap<u8, Block>,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.blocks.iter().for_each(|(_, value)| {
            value.hash(state);
        });
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Board {
    fn available_moves(&self) -> HashSet<Orientation> {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let vector: Vec<Orientation> = Orientation::into_enum_iter()
        .filter(|orientation| {
            return neighbour_ids.neighbour_towards(orientation) != None;
        })
        .filter(|orientation| {
            let neighbour_id = neighbour_ids.neighbour_towards(orientation).unwrap();
            let neighbour = self.blocks[&neighbour_id];
            
            if (neighbour.small != None && neighbour.small.unwrap().orientation.opposite() != *orientation) ||
               (neighbour.large != None && neighbour.large.unwrap().orientation.opposite() != *orientation) { 
                return false
            } else {
                return match (neighbour.small, neighbour.large) {
                    (None, Some(_)) => 
                        (block.large == None || block.large.unwrap().orientation == *orientation),
                    (Some(_), None) => 
                        (block.small == None || block.small.unwrap().orientation == *orientation) &&
                        (block.large == None || block.large.unwrap().orientation == *orientation),
                    (Some(_), Some(_)) => block.small == None && block.large == None,
                    (None, None) => true,
                };
            }

            /*
            can't move if
            dest large or small is not in opposite orientation

            if dest has nothing, then we can move, dragging everything 
            if dest has only small, then we can only move if we won't drag anything
            if dest has only large, then we can only move if source has no large or wouldn't be dragged
            if dest has both, then we can only move if source has neither
            */

//             //there's no dest.large
//             //or dest.large orientation is opposite orientation
//             let neighbour_id = neighbour_ids.neighbour_towards(orientation).unwrap();
//             let neighbour = self.blocks[&neighbour_id];
//             let small_unit_ok = neighbour.small == None || 
//                 (neighbour.small.unwrap().orientation.opposite() == *orientation && (block.small == None || block.small.unwrap().orientation == *orientation));
//             let large_unit_ok = 
//                 neighbour.large == None ||
//                 (neighbour.large.unwrap().orientation.opposite() == *orientation && (block.large == None || block.large.unwrap().orientation == *orientation));
            
//             return small_unit_ok && large_unit_ok;
        })
        .collect();

        return HashSet::from_iter(vector);
    }

    //TODO combine available_moves and moving into a single move() -> Board? method

    fn moving(&self, orientation: Orientation) -> Board {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let neighbour_id = neighbour_ids.neighbour_towards(&orientation).unwrap();
        let mut blocks = self.blocks.clone();
        let mut neighbour_block = blocks.get_mut(&neighbour_id).unwrap().clone();
        let mut player_block = blocks.get_mut(&player_block_id).unwrap().clone();
        
        if player_block.small != None &&
            (player_block.small.unwrap().orientation != orientation || (player_block.large != None && (player_block.large.unwrap().orientation != orientation))) {
            neighbour_block.small = player_block.small;
            player_block.small = None;
        }

        if player_block.large != None && player_block.large.unwrap().orientation != orientation {
            neighbour_block.large = player_block.large;
            player_block.large = None;
        }
        
        blocks.insert(neighbour_id, neighbour_block);
        blocks.insert(player_block_id, player_block);
        
        let board = Board{
            player: Player{block_id: neighbour_id},
            blocks: blocks
        };
        let hash = calculate_hash(&board);
        return board;
    }

    fn next_boards(&self) -> Vec<Board> {
        return self.available_moves()
            .into_iter()
            .map(|orientation| { return self.moving(orientation); })
            .collect();
    }

    fn is_win(&self) -> bool {
        let player_block_id = self.player.block_id;
        let block = self.blocks[&player_block_id];
        return match (block.small, block.large) {
            (Some(small), Some(large)) => small.color == Color::Red && large.color == Color::Red,
            _ => false,
        };
    }
}

fn main() {
    let graph = UnGraphMap::<NetworkNode, ()>::new();
    let rc = RefCell::new(graph); 
    let c = Rc::new(rc);

    // let first_board = Board{
    //     player: Player{block_id: 0},
    //     blocks: hashmap!{
    //         0 => Block{
    //             small: Some(Unit{
    //                 orientation: Orientation::Left,
    //                 color: Color::Red,
    //             }),
    //             large: None,
    //             id: 0,
    //             neighbour_ids: NeighbourIds::new(None, Some(2), None, Some(1))
    //         },
    //         1 => Block{
    //             small: None,
    //             large: Some(Unit{
    //                 orientation: Orientation::Left,
    //                 color: Color::Red,
    //             }),
    //             id: 1,
    //             neighbour_ids: NeighbourIds::new(None, Some(3), Some(0), None)
    //         },
    //         2 => Block{
    //             small: None,
    //             large: None,
    //             id: 2,
    //             neighbour_ids: NeighbourIds::new(Some(0), None, None, Some(3))
    //         },
    //         3 => Block{
    //             small: None,
    //             large: None,
    //             id: 3,
    //             neighbour_ids: NeighbourIds::new(Some(1), None, Some(2), None)
    //         },
    //     }
    // };

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

    let mut boards: HashMap<u64, Board> = hashmap!{};
    
    build(first_board, &mut boards, &mut c.borrow_mut());

    // assert_eq!(c.borrow().node_count(), 14);
    // assert_eq!(c.borrow().edge_count(), 13);
    // assert_eq!(boards.len(), c.borrow().node_count());

    // assert_eq!(can_win(&boards, & c.borrow()), true);
    
    // assert_eq!(algo::has_path_connecting(c.borrow(), first_node, first_node, None), true);
    // println!(algo::dijkstra(c.borrow(), first_node, goal: Option<G::NodeId>, 1))
    // println!("{:?}", algo::dijkstra(c.borrow(), first_node, None, |_| 1));
}

// fn print(board: &Board) {
//     let player_block_id = board.player.block_id;
//     let mut keys: Vec<u8> = vec![];


//     let keys = board.blocks.keys().collect().sort();
// }

fn build(board: Board, boards: &mut HashMap<u64, Board>, network: &mut UnGraphMap::<NetworkNode, ()>) {
    let board_hash = calculate_hash(&board);
    let board_node = NetworkNode{hash_id: board_hash};

    // println!("{:?}", board);

    network.add_node(board_node);
    let board_clone = board.clone();
    let board_clone_hash = calculate_hash(&board_clone);
    assert_eq!(board_clone_hash, board_hash);
    boards.insert(board_hash, board.clone());
    // println!("{:?}", algo::dijkstra(c.borrow(), first_node, None, |_| 1));

    board.next_boards().iter().for_each(|next| {
        let next_node = NetworkNode{hash_id: calculate_hash(&next)};

        if !network.contains_node(next_node) {
            build(next.clone(), boards, network);
        }

        if !network.contains_edge(board_node, next_node) {
            network.add_edge(board_node, next_node, ());
        }
    });
}

fn can_win(boards: &HashMap<u64, Board>, network: & UnGraphMap::<NetworkNode, ()>) -> bool {
    match network.nodes().find(|node| { 
        let hash_id = node.hash_id;
        let board = &boards[&hash_id];
        return board.is_win();
    }) {
        Some(_) => return true,
        None => return false,
    }
}

#[cfg(test)]
mod test;