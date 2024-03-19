use std::convert::TryInto;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

macro_rules! boardValue {
    ($board: expr, $row: expr, $col: expr) => { $board[$row * WIDTH + $col] }
}
macro_rules! boardLastRowValue {
    ($board: expr, $col: expr) => { boardValue!($board, HEIGHT-1, $col) }
}

pub const EMPTY: u8 = 0;
pub const PLAYER_A: u8 = 1;
pub const PLAYER_B: u8 = 2;

pub type Board = [u8; WIDTH * HEIGHT];

const SCORE_MAX: i32 = i32::MAX;


pub fn solve(board: &Board, depth: i32) -> usize {
    minimax(board, 0, depth as usize, true, -SCORE_MAX, SCORE_MAX) as usize
}

fn minimax(board: &Board, depth: usize, max_depth: usize, maximizing_player: bool, mut alpha: i32, mut beta: i32) -> i32 {
    let player_won = is_game_over(&board, maximizing_player);
    if player_won {
        return SCORE_MAX*(if maximizing_player {1} else {-1});
    }
    if depth + 1 == max_depth {
        return evaluate_position(board);
    }
    let mut column = WIDTH / 2;
    let mut score: i32;
    

    if maximizing_player {
        score = -SCORE_MAX;
        for (col, new_board) in generate_next_position(board.clone(), PLAYER_A) {
            let new_score = minimax(&new_board, depth + 1, max_depth, false, alpha, beta);
            if new_score >= score {
                score = new_score;
                column = col
            }
            if score > beta {
                break;
            }
            alpha = std::cmp::max(alpha, new_score);
        }
    } else {
        score = i32::MAX;
        for (col, new_board) in generate_next_position(board.clone(), PLAYER_B) {
            let new_score = minimax(&new_board, depth + 1, max_depth, true, alpha, beta);
            if new_score <= score {
                score = new_score;
                column = col
            }
            if score < alpha {
                break;
            }
            beta = std::cmp::min(beta, new_score);
        }
    }

    if depth == 0 {
        return column.try_into().unwrap();
    }
    score
}

fn generate_next_position(board: Board, player: u8) -> impl Iterator<Item = (usize, Board)> {
    (0..WIDTH)
        .filter(move |col| boardLastRowValue!(board, col) == EMPTY)
        .map(move |col| {
            let mut new_board = board.clone();
            for row in 0..HEIGHT {
                if boardValue!(new_board, row, col) == EMPTY {
                    boardValue!(new_board, row, col) = player;
                    break;
                }
            }
            (col, new_board)
        })
}

pub fn is_game_over(board: &Board, maximizing_player: bool) -> bool {
    is_draw(board)
    || !maximizing_player && player_won(board, PLAYER_A)
    || maximizing_player && player_won(board, PLAYER_B)
}

fn is_draw(board: &Board) -> bool{
    (0..WIDTH).all(|col| boardLastRowValue!(board, col) != EMPTY)
}

fn player_won(board: &Board, player: u8) -> bool {
    // CHECK HORIZONTAL
    for row in 0..HEIGHT {
        let mut count = 0;
        for col in 0..WIDTH {
            if boardValue!(board, row, col) == player {
                count += 1;
            } else {
                count = 0;
            }

            if count == 4 {
                return true;
            }
        }
    }

    // CHECK VERTICAL
    for col in 0..WIDTH {
        let mut count = 0;
        for row in 0..HEIGHT {
            if boardValue!(board, row, col) == player {
                count += 1;
            } else {
                count = 0;
            }

            if count == 4 {
                return true;
            }
            if WIDTH - row + count < 4 {
                break;
            }
        }
    }

    // CHECK DIAGONALL LEFT
    for k in 3..(WIDTH+HEIGHT-4) {
        let mut count = 0;
        for col in 0..(k+1) {
            let row: usize = k - col;
            if row < HEIGHT && col < WIDTH {
                if boardValue!(board, row, col) == player {
                    count += 1;
                } else {
                    count = 0;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    // CHECK DIAGONALL RIGHT
    for k in 3..(WIDTH+HEIGHT-4) {
        let mut count = 0;
        for col in 0..(k+1) {
            let row: i32 = HEIGHT as i32 - k as i32 + col as i32;
            if row >= 0 && row < HEIGHT as i32 && col < WIDTH {
                if boardValue!(board, row as usize, col) == player {
                    count += 1;
                } else {
                    count = 0;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    false
}

fn evaluate_position(board: &Board) -> i32 {
    let mut evaluation = 0;
    
    // evaluate horizontal
    for row in 0..HEIGHT {
        let mut max_player_a = 0;
        let mut max_player_b = 0;
        let mut count_player_a = 0;
        let mut count_player_b = 0;

        for col in 0..WIDTH {
            if boardValue!(board, row, col) == EMPTY {
                count_player_a = 0;
                count_player_b = 0;
                continue;
            }
            if boardValue!(board, row, col) == PLAYER_A {
                count_player_a += 1;
                max_player_a = std::cmp::max(count_player_a, max_player_a);
                count_player_b = 0;
            } else {
                count_player_b += 1;
                max_player_b = std::cmp::max(count_player_b, max_player_b);
                count_player_a = 0;
            }

        }

        evaluation += max_player_a - max_player_b;
    }

    // evaluate VERTICAL
    for col in 0..WIDTH {
        let mut max_player_a = 0;
        let mut max_player_b = 0;
        let mut count_player_a = 0;
        let mut count_player_b = 0;

        for row in 0..HEIGHT {
            if boardValue!(board, row, col) == EMPTY {
                count_player_a = 0;
                count_player_b = 0;
                continue;
            }
            if boardValue!(board, row, col) == PLAYER_A {
                count_player_a += 1;
                max_player_a = std::cmp::max(count_player_a, max_player_a);
                count_player_b = 0;
            } else {
                count_player_b += 1;
                max_player_b = std::cmp::max(count_player_b, max_player_b);
                count_player_a = 0;
            }
        }
        
        evaluation += max_player_a - max_player_b;
    }

    // evaluate DIAGONALL LEFT
    for k in 3..(WIDTH+HEIGHT-4) {
        let mut max_player_a = 0;
        let mut max_player_b = 0;
        let mut count_player_a = 0;
        let mut count_player_b = 0;
        for col in 0..(k+1) {
            let row: usize = k - col;
            if row < HEIGHT && col < WIDTH {
                if boardValue!(board, row, col) == EMPTY {
                    count_player_a = 0;
                    count_player_b = 0;
                    continue;
                }
                if boardValue!(board, row, col) == PLAYER_A {
                    count_player_a += 1;
                    max_player_a = std::cmp::max(count_player_a, max_player_a);
                    count_player_b = 0;
                } else {
                    count_player_b += 1;
                    max_player_b = std::cmp::max(count_player_b, max_player_b);
                    count_player_a = 0;
                }
            }

        }

        evaluation += max_player_a - max_player_b;
    }

    // evaluate DIAGONALL RIGHT
    for k in 3..(WIDTH+HEIGHT-4) {
        let mut max_player_a = 0;
        let mut max_player_b = 0;
        let mut count_player_a = 0;
        let mut count_player_b = 0;        
        for col in 0..(k+1) {
            let row: i32 = HEIGHT as i32 - k as i32 + col as i32;
            if row >= 0 && row < HEIGHT as i32 && col < WIDTH {
                if boardValue!(board, row as usize, col) == EMPTY {
                    count_player_a = 0;
                    count_player_b = 0;
                    continue;
                }
                if boardValue!(board, row as usize, col) == PLAYER_A {
                    count_player_a += 1;
                    max_player_a = std::cmp::max(count_player_a, max_player_a);
                    count_player_b = 0;
                } else {
                    count_player_b += 1;
                    max_player_b = std::cmp::max(count_player_b, max_player_b);
                    count_player_a = 0;
                }
            }
        }

        
        evaluation += max_player_a - max_player_b;
    }

    evaluation
}
