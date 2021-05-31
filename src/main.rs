mod example_boards;
mod solver;
mod model;
mod utils;
mod hasher;
mod builder;

use std::time::{Duration, Instant};
use std::{collections::{HashMap, HashSet}};
use model::NetworkNode;

fn main() {
    let start1 = Instant::now();
    // let boards = builder::build(3, 3, 0, 0, 1, 1); // 3388 - 1.9s - 24
    // let boards = builder::build(1, 3, 0, 0, 1, 1); // 6 - tiny - 2
    // let boards = builder::build(2, 5, 1, 1, 1, 1); // 1010160 - ?s - 59
    let boards = builder::build(3, 3, 1, 1, 1, 1); // 529080(198405 condensed) - 374s - 53
    // let boards = builder::build(2, 4, 0, 1, 1, 1); // 21360 - 4.3s - 29
    // let boards = builder::build(2, 4, 0, 0, 1, 1); // 1968 - 0.5s - 25
    // let boards = builder::build(2, 3, 1, 0, 1, 1); // 3312 - 0.1s - 13
    println!("Creating {} boards took {:?}", boards.len(), start1.elapsed());

    let start2 = Instant::now();

    let mut stored_hardest: Option<(model::Board, i32, Vec<NetworkNode>, HashMap<u64, model::Board>)> = None;

    for board in boards {
        if let Some((cost, path, boards)) = solver::get_simplest_solution(&board) {
            if let Some(ref hardest) = stored_hardest {
                if cost > hardest.1 {
                    stored_hardest = Some((board.clone(), cost, path, boards));
                }
            } else {
                stored_hardest = Some((board.clone(), cost, path, boards));   
            }
        }
    }

    if let Some(hardest) = stored_hardest {
        // println!("Cost: {}", cost);
        println!("Cost: {}", hardest.1);
        utils::print(&hardest.0);
        // println!("Route: {:?}", hardest.2);


        if let Some(moves) = solver::moves_for(&hardest.3, &hardest.2) {
            println!("{:?}", moves);
        } else {
            println!("no moves found");
        }
    }

    println!("Analysing boards took {:?}", start2.elapsed());
    println!("Total: {:?}", start1.elapsed());

    // let board = example_boards::easiest_boco_level();
    // utils::print(&board);
    // match solver::get_simplest_solution(&board) {
    //     Some((cost, path)) => {
    //         println!("Cost: {}", cost);
    //         utils::print(&board);
    //         println!("Route: {:?}", path);
    //     },
    //     None => (),
    // }
}

#[cfg(test)]
mod test;