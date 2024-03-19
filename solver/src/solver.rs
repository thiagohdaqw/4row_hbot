const WIDTH: usize = 7;
const HEIGHT: usize = 6;

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




pub fn solve(board: Board, depth: usize, player: u8) -> i32 {
    let maximizing_player = player == PLAYER_A;
    minimax(board, 0, depth, maximizing_player);
    i32::MAX
}

fn minimax(board: Board, depth: usize, max_depth: usize, maximizing_player: bool) -> i32 {
    
    if depth + 1 == max_depth || is_game_over(&board) {
        if is_game_over(&board) {
            println!();
            
            println!("{depth} {maximizing_player}");
            for row in 0..HEIGHT {
                for col in 0..WIDTH {
                    print!("{} ", board[row*WIDTH + col]);
                }
                println!();
            }
            println!();
        }
        return evaluate_position(board);
    }
    let mut column = WIDTH / 2;
    let mut score: i32;
    

    if maximizing_player {
        score = i32::MIN;
        for (col, new_board) in generate_next_position(board, PLAYER_A) {
            let new_score = minimax(new_board, depth + 1, max_depth, false);
            if new_score > score {
                score = new_score;
                column = col
            }
        }
    } else {
        score = i32::MAX;
        for (col, new_board) in generate_next_position(board, PLAYER_B) {
            let new_score = minimax(new_board, depth + 1, max_depth, true);
            if new_score < score {
                score = new_score;
                column = col
            }
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

fn is_game_over(board: &Board) -> bool {
    is_draw(board)
    || player_won(board, PLAYER_A)
    || player_won(board, PLAYER_B)
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

fn evaluate_position(_board: Board) -> i32 {
    0
}
