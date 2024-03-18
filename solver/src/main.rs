mod solver;

use solver::Board;

use crate::solver::{solve, PLAYER_A};


fn main() {
    let board: Board = [
        0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];

    println!("{}", solve(board, 8, PLAYER_A));
}
