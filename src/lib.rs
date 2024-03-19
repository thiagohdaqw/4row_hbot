mod utils;
mod solver;

use solver::{Board, HEIGHT, WIDTH};
use wasm_bindgen::prelude::*;

fn parse_board(board_input: &str) -> Board {
    let mut board: Board = [0; WIDTH*HEIGHT];
    for (index, value) in board_input.chars().map(|c| c as u8 - 48).enumerate() {
        board[index] = value;
    }
    board
}

#[wasm_bindgen]
pub fn solve(board_input: &str, depth: i32) -> usize {
    solver::solve(&parse_board(board_input), depth)
}

#[wasm_bindgen]
pub fn is_game_over(board_input: &str, player: bool) -> bool {
    solver::is_game_over(&parse_board(board_input), !player)
}