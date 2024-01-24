use std::env;
use crate::rubiks_cube_solver::rubiks_cube_solver;

mod rubiks_cube_solver;
mod Cube;
mod Face;
mod Movements;
mod constants;
mod Solver;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    rubiks_cube_solver(args)
}
