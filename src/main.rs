mod example_boards;
mod solver;
mod model;
mod utils;
mod hasher;
mod builder;

use std::time::{Duration, Instant};

fn main() {
    let start1 = Instant::now();
    // let boards = builder::build(3, 3, 0, 0, 1, 1);
    // let boards = builder::build(1, 3, 0, 0, 1, 1);
    // let boards = builder::build(2, 5, 1, 1, 1, 1);
    // let boards = builder::build(3, 3, 1, 1, 1, 1);
    // let boards = builder::build(2, 4, 0, 1, 1, 1); // 107520 - 138s - max 29 moves - 0.0013s per board
    // let boards = builder::build(2, 4, 0, 0, 1, 1); // 5376 - 10.4s - max 25 moves - 0.0019s per board
    let boards = builder::build(2, 3, 1, 0, 1, 1); // 23040 - 5.3s - max 13 moves - 0.0002s per board
    println!("Creating {} boards took {:?} seconds", boards.len(), start1.elapsed());

    // let n = 10000;
    // let a = boards[0..n].to_vec();
    // let b = boards[boards.len()-n .. boards.len()].to_vec();9

    let start2 = Instant::now();

    for board in boards {
        match solver::get_simplest_solution(&board) {
            Some((cost, path)) => {
                // println!("Cost: {}", cost);
                if cost >= 13 {
                    println!("Cost: {}", cost);
                    utils::print(&board);
                    println!("Route: {:?}", path);
                }
            },
            None => (),
        }
    }

    println!("Analysing boards took {:?} seconds", start2.elapsed());
}

#[cfg(test)]
mod test;