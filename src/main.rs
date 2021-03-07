mod example_boards;
mod solver;
mod model;
mod utils;
mod hasher;

fn main() {
    solver::solve(example_boards::hardest_boco_level());
}

#[cfg(test)]
mod test;