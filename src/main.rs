mod builder;
mod solver;
mod model;
mod utils;
mod hasher;

fn main() {
    let board = builder::build_hardest_boco_level();
    solver::solve(board);
}

#[cfg(test)]
mod test;