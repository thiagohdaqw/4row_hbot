mod solver;

use solver::Board;

use crate::solver::solve;




fn main() {
    let board: Board = [
        0, 0, 0, 2, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];

    println!("{}", solve(&board, 8));
}
