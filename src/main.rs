mod solver;
mod model;
mod utils;
mod hasher;
mod builder;

use std::time::{Instant};
use std::{collections::{HashMap}};
use model::NetworkNode;
extern crate rayon;
use rayon::prelude::*;

use crate::solver::Solution;

fn main() {
    let start1 = Instant::now();
    // let boards = builder::build(3, 3, 0, 0, 1, 1); // 3388 - 1.9s - 24
    // let boards = builder::build(1, 3, 0, 0, 1, 1); // 6 - tiny - 2
    // let boards = builder::build(2, 5, 1, 1, 1, 1); // 1010160 - ?s - 59
    // let boards = builder::build(3, 3, 1, 1, 1, 1); // 529080(198405 condensed) - 374s - 53
    let boards = builder::build(2, 4, 0, 1, 1, 1); // 21360 - 4.3s - 29
    // let boards = builder::build(2, 5, 2, 1, 1, 1);
    // let boards = builder::build(2, 4, 0, 0, 1, 1); // 1968 - 0.5s - 25
    // let boards = builder::build(2, 3, 1, 0, 1, 1); // 3312 - 0.1s - 13
    println!("Creating {} boards took {:?}", boards.len(), start1.elapsed());

    let start2 = Instant::now();

    let solutions: Vec<Solution> = boards
        // .iter()
        .par_iter()
        .map(|board| { solver::get_simplest_solution(&board) })
        .filter_map(|x| x)
        .collect();

    if let Some(max) = solutions.par_iter().max_by_key(|result| result.cost) {
        let board = max.boards.get(&max.start_node.hash_id).unwrap();
        println!("Cost: {}", max.cost);
        utils::print(&board);

        if let Some(moves) = solver::moves_for(&max.boards, &max.nodes) {
            println!("{:?}", moves);
        } else {
            println!("no moves found");
        }
    }

    println!("Analysing boards took {:?}", start2.elapsed());
    println!("Total: {:?}", start1.elapsed());
}

#[cfg(test)]
mod test;