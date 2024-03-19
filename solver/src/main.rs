mod solver;

use solver::Board;

use crate::solver::{solve, PLAYER_A};


fn main() {
    let board: Board = [
        0, 0, 0, 2, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];

    println!("{}", solve(board, 8));

    // let test = [
    //     'A','B','C','D','E','F','G',
    //     'G','H','I','J','K','L','M',
    //     'M','N','O','P','Q','R','S',
    //     'S','T','U','V','W','X','Y',
    //     'Y','Z','a','b','c','d','e',
    //     'f','g','h','i','j','k','l',
    //     ];

    // const WIDTH: usize = 7;
    // const HEIGHT: usize = 6;
    // for k in 3..(WIDTH+HEIGHT-4) {
    //     for col in 0..(k+1) {
    //         let row: i32 = HEIGHT as i32 - k as i32 + col as i32;
    //         if row >= 0 && row < HEIGHT as i32 && col < WIDTH  {
    //             print!("{} ", test[WIDTH*row as usize + col]);
    //         }
    //     }
    //     println!();
    // }
}
