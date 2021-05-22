mod example_boards;
mod solver;
mod model;
mod utils;
mod hasher;
mod builder;

use std::time::{Duration, Instant};

fn main() {
    let start1 = Instant::now();
    // let boards = builder::build(3, 3, 0, 0, 1, 1); // 3388 - 1.9s - 24
    // let boards = builder::build(1, 3, 0, 0, 1, 1); // 6 - tiny - 2
    // let boards = builder::build(2, 5, 1, 1, 1, 1); // 1010160 - ?s - 59
    // let boards = builder::build(3, 3, 1, 1, 1, 1); // 529080 - 374s - 52
    let boards = builder::build(2, 4, 0, 1, 1, 1); // 21360 - 4.3s - 29
    // let boards = builder::build(2, 4, 0, 0, 1, 1); // 1968 - 0.5s - 25
    // let boards = builder::build(2, 3, 1, 0, 1, 1); // 3312 - 0.1s - 13
    println!("Creating {} boards took {:?} seconds", boards.len(), start1.elapsed());

    // let n = 10000;
    // let a = boards[0..n].to_vec();
    // let b = boards[boards.len()-n .. boards.len()].to_vec();9

    let start2 = Instant::now();

    for board in boards {
        match solver::get_simplest_solution(&board) {
            Some((cost, path)) => {
                // println!("Cost: {}", cost);
                if cost >= 24 {
                    println!("Cost: {}", cost);
                    utils::print(&board);
                    println!("Route: {:?}", path);
                }
            },
            None => (),
        }
    }

    println!("Analysing boards took {:?} seconds", start2.elapsed());
    println!("Total: {:?} seconds", start1.elapsed());
}

#[cfg(test)]
mod test;