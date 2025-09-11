use crate::{board::*};

fn pos(r: i8, c: i8) -> Position { Position { row: r, col: c } }

pub fn all_legal_moves(board: &Board) -> Vec<(Position, Position)> {
    let mut legal_moves: Vec<(Position, Position)> = Vec::new();

    for from_row in 0..BOARD_ROWS{
        for from_col in 0..BOARD_COLS {
            let from_pos = pos(from_row, from_col);

            for to_row in 0..BOARD_ROWS {
                for to_col in 0..BOARD_COLS{
                    let to_pos = pos(to_row, to_col);
                    let mut tmp = board.clone();

                    if tmp.move_piece(from_pos, to_pos).is_ok() {

                        legal_moves.push((from_pos, to_pos));
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    legal_moves
}

pub fn dfs(b: &Board, d: usize, depth_total: usize, totals: &mut [usize]) {
    let moves = all_legal_moves(b);
    let idx = depth_total - d;
    totals[idx] += moves.len();

    if d == 1 { return; }

    for (from, to) in moves {
        let mut next = b.clone();
        next.move_piece(from, to).unwrap();
        dfs(&next, d - 1, depth_total, totals);
    }
}

pub fn perft_layers(board: &mut Board, depth:usize) -> Vec<usize>{
    assert!(depth >= 1);
    let mut totals = vec![0usize; depth];
    dfs(board, depth, depth, &mut totals);
    totals
}