use std::convert::TryInto;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

macro_rules! boardValue {
    ($board: expr, $row: expr, $col: expr) => {
        $board[$row * WIDTH + $col]
    };
}
macro_rules! boardLastRowValue {
    ($board: expr, $col: expr) => {
        boardValue!($board, HEIGHT - 1, $col)
    };
}

pub const EMPTY: u8 = 0;
pub const PLAYER_IA: u8 = 1;
pub const PLAYER: u8 = 2;

pub type Board = [u8; WIDTH * HEIGHT];

const SCORE_MAX: i32 = i32::MAX >> 1;

pub fn solve(board: &Board, depth: i32) -> usize {
    minimax(board, 0, depth as usize, true, -SCORE_MAX, SCORE_MAX) as usize
}

fn minimax(
    board: &Board,
    depth: usize,
    max_depth: usize,
    maximizing_player: bool,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if is_draw(board) {
        return 0;
    }
    if maximizing_player && player_won(board, PLAYER) {
        return -SCORE_MAX;
    }
    if !maximizing_player && player_won(board, PLAYER_IA) {
        return SCORE_MAX;
    }
    if depth + 1 == max_depth {
        return evaluate_position(board);
    }
    let mut column = WIDTH / 2;
    let mut score: i32;

    if maximizing_player {
        score = i32::MIN;
        for (col, new_board) in generate_next_position(*board, PLAYER_IA) {
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
        for (col, new_board) in generate_next_position(*board, PLAYER) {
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
            let mut new_board = board;
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
    || !maximizing_player && player_won(board, PLAYER_IA)
    || maximizing_player && player_won(board, PLAYER)
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
    for k in 3..(WIDTH + HEIGHT - 3) {
        let mut count = 0;
        for col in 0..(k + 1) {
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
    for k in 3..(WIDTH + HEIGHT - 3) {
        let mut count = 0;
        for col in 0..(k + 1) {
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
    evaluate_player_position(board, PLAYER_IA) - evaluate_player_position(board, PLAYER)
}

fn evaluate_player_position(board: &Board, player: u8) -> i32 {
    let mut evaluation = 0;

    // evaluate horizontal
    for row in 0..HEIGHT {
        let mut max_player = 0;
        let mut count_player = 0;
        let mut count_empty = 0;

        for col in 0..WIDTH {
            if boardValue!(board, row, col) == EMPTY {
                // count_player = 0;
                count_empty += 1;
                continue;
            }
            if boardValue!(board, row, col) == player {
                count_player += 1;
                max_player = std::cmp::max(count_player, max_player);
            }
        }

        if count_player == 3 && count_empty > 0 {
            evaluation += 100;
        } else {
            evaluation += count_player*5 + count_empty;
        }
    }

    // evaluate VERTICAL
    for col in 0..WIDTH {
        let mut max_player = 0;
        let mut count_player = 0;
        let mut count_empty = 0;

        for row in 0..HEIGHT {
            if boardValue!(board, row, col) == EMPTY {
                count_player = 0;
                count_empty += 1;
                continue;
            }
            if boardValue!(board, row, col) == player {
                count_player += 1;
                max_player = std::cmp::max(count_player, max_player);
            }
        }

        if max_player == 3 && count_empty > 0 {
            evaluation += 100;
        } else {
            evaluation += count_player*5 + count_empty;
        }
    }

    // evaluate DIAGONALL LEFT
    for k in 3..(WIDTH + HEIGHT - 3) {
        let mut max_player = 0;
        let mut count_player = 0;
        let mut count_empty = 0;
        for col in 0..(k + 1) {
            let row: usize = k - col;
            if row < HEIGHT && col < WIDTH {
                if boardValue!(board, row, col) == EMPTY {
                    // count_player = 0;
                    count_empty += 1;
                    continue;
                }
                if boardValue!(board, row, col) == player {
                    count_player += 1;
                    max_player = std::cmp::max(count_player, max_player);
                }
            }
        }

        if max_player == 3 && count_empty > 0 {
            evaluation += 100;
        } else {
            evaluation += count_player + count_empty;
        }
    }

    // evaluate DIAGONALL RIGHT
    for k in 3..(WIDTH + HEIGHT - 3) {
        let mut max_player = 0;
        let mut count_player = 0;
        let mut count_empty = 0;
        for col in 0..(k + 1) {
            let row: i32 = HEIGHT as i32 - k as i32 + col as i32;
            if row >= 0 && row < HEIGHT as i32 && col < WIDTH {
                if boardValue!(board, row as usize, col) == EMPTY {
                    // count_player = 0;
                    count_empty += 1;
                    continue;
                }
                if boardValue!(board, row as usize, col) == player {
                    count_player += 1;
                    max_player = std::cmp::max(count_player, max_player);
                }
            }
        }

        if max_player == 3 && count_empty > 0 {
            evaluation += 100;
        } else {
            evaluation += count_player + count_empty;
        }
    }

    evaluation
}
