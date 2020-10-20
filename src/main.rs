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

struct Blocks {
    blocks: HashMap<u8, Block>,
}

impl Hash for Blocks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocks.iter().for_each(|block| {
            block.hash(state);
        });
    }
}

impl Copy for Blocks {}

impl Clone for Blocks {
    fn clone(&self) -> Blocks {
        // return Blocks{self.blocks.clone()}
        *self
    }
}

// #[derive(Clone)]
// #[derive(Ord)]
#[derive(Copy, Ord, Clone)]
struct Node {
    player: Player,
    blocks: Blocks,
}

//look for hashing a hashmap in rust

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.blocks.hash(state);
    }
}

// impl<K, V> HashMap<K, V> {
//     fn persist(&self, prefix: &str, data: &HashMap<K, HashSet<V>>) {
//         self.dirty.set(true);
//         self.data.set(Some(data.clone()));
//     }
// }

impl Node {
    fn available_moves(&self) -> HashSet<Orientation> {
        let player_block_id = self.player.block_id;
        let block = self.blocks.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let vector: Vec<Orientation> = Orientation::into_enum_iter()
        .filter(|orientation| {
            return neighbour_ids.neighbour_towards(orientation) != None;
        })
        .filter(|orientation| {
            let neighbour_id = neighbour_ids.neighbour_towards(orientation).unwrap();
            let neighbour = self.blocks.blocks[&neighbour_id];
            let small_unit_ok = neighbour.small == None || 
                (neighbour.small.unwrap().orientation.opposite() == *orientation && (block.small == None || block.small.unwrap().orientation == *orientation));
            let large_unit_ok = neighbour.large == None ||
                (neighbour.large.unwrap().orientation.opposite() == *orientation && (block.large == None || block.large.unwrap().orientation == *orientation));
            return small_unit_ok && large_unit_ok;
        })
        .collect();

        return HashSet::from_iter(vector);
    }

    fn moving(&self, orientation: Orientation) -> Node {
        let player_block_id = self.player.block_id;
        let block = self.blocks.blocks[&player_block_id];
        let neighbour_ids = block.neighbour_ids;
        let neighbour_id = neighbour_ids.neighbour_towards(&orientation).unwrap();
        let mut blocks = self.blocks.blocks.clone();
        let mut neighbour_block = blocks.get_mut(&neighbour_id).unwrap().clone();
        let mut player_block = blocks.get_mut(&player_block_id).unwrap().clone();
        
        if player_block.small != None && player_block.small.unwrap().orientation != orientation {
            neighbour_block.small = player_block.small;
            player_block.small = None;
        }

        if player_block.large != None && player_block.large.unwrap().orientation != orientation {
            neighbour_block.large = player_block.large;
            player_block.large = None;
        }
        
        blocks.insert(neighbour_id, neighbour_block);
        blocks.insert(player_block_id, player_block);
        
        return Node{
            player: Player{block_id: neighbour_id},
            blocks: Blocks{blocks}
        }
    }

    fn next_nodes(&self) -> Vec<Node> {
        return self.available_moves()
            .into_iter()
            .map(|orientation| { return self.moving(orientation); })
            .collect();
    }

    fn is_win(&self) -> bool {
        let player_block_id = self.player.block_id;
        let block = self.blocks.blocks[&player_block_id];
        return match (block.small, block.large) {
            (Some(small), Some(large)) => small.color == Color::Red && large.color == Color::Red,
            _ => false,
        };
    }
}

fn main() {
    let graph = UnGraphMap::<Node, ()>::new();
    let rc = RefCell::new(graph); 
    let c = Rc::new(rc);

    let first_node = Node{
        player: Player{block_id: 0},
        blocks: hashmap!{
            0 => Block{
                small: Some(Unit{
                    orientation: Orientation::Up,
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

#[cfg(test)]
mod test;