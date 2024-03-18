const WIDTH: usize = 7;
const HEIGHT: usize = 6;

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
    if depth + 1 == max_depth || is_game_over(board) {
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
        .filter(move |col| board[WIDTH * (HEIGHT - 1) + col] == EMPTY)
        .map(move |col| {
            let mut new_board = board.clone();
            for row in 0..HEIGHT {
                let index = WIDTH * row + col;
                if new_board[index] == EMPTY {
                    new_board[index] = player;
                    break;
                }
            }
            (col, new_board)
        })
}

fn is_game_over(_board: Board) -> bool {
    false
}

fn evaluate_position(_board: Board) -> i32 {
    0
}
