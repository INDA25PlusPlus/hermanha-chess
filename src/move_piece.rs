use crate::board::{BOARD_COLS, BOARD_ROWS, Board, Position};
use crate::pieces::{Color, PieceType};

// ASCII board

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    EmptyFrom,
    WrongTurn,
    IllegalShape,
    Blocked { at: Position },
    SelfCheck,
    SameSquare,
    OutOfBounds,
    CaptureOwn,
}

pub type MoveOk = ();

impl Position {
    pub fn delta(&self, other: Position) -> (i8, i8) {
        (other.row - self.row, other.col - self.col)
    }
}

impl Board {
    pub fn move_piece(
        &mut self,
        from_pos: Position,
        to_pos: Position,
    ) -> Result<MoveOk, MoveError> {
        self.pseudo_legal_move(from_pos, to_pos)?;

        let from_piece = self.get(from_pos).expect("validated: piece on from_pos");
        let to_piece = self.get(to_pos);

        self.set(from_pos, None);
        self.set(to_pos, Some(from_piece));

        if self.is_in_check() {
            self.set(to_pos, to_piece);
            self.set(from_pos, Some(from_piece));
            return Err(MoveError::SelfCheck);
        }

        self.move_turn = match self.move_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        Ok(())
    }

    #[inline]
    pub fn pos_on_board(&self, pos: Position) -> bool {
        pos.row >= 0 && pos.row < BOARD_ROWS && pos.col >= 0 && pos.col < BOARD_COLS
    }

    pub fn pseudo_legal_move(
        &self,
        from_pos: Position,
        to_pos: Position,
    ) -> Result<MoveOk, MoveError> {

        let mut capture = false;
        // check to_pos position on board, believe we dont have to_pos check from_pos
        if !self.pos_on_board(from_pos) || !self.pos_on_board(to_pos) {
            return Err(MoveError::OutOfBounds);
        }
        if from_pos == to_pos {
            return Err(MoveError::SameSquare);
        }

        // get piece on from_pos and to_pos position
        let Some(from_piece) = self.get(from_pos) else {
            return Err(MoveError::EmptyFrom);
        };
        // check turn
        if from_piece.color != self.move_turn {
            return Err(MoveError::WrongTurn);
        }
        // check if own piece color is on square
        if let Some(to_piece) = self.get(to_pos) {
            if to_piece.color == from_piece.color {
                return Err(MoveError::CaptureOwn);
            }
            else {
                capture = true
            }
        };

        let (d_row, d_col) = from_pos.delta(to_pos);
        // check if piece allows move shape
        if !from_piece.move_shape_ok(d_row, d_col, capture, from_pos.row) {
            return Err(MoveError::IllegalShape);
        }

        use PieceType::*;

        let row_offset = d_row.signum();
        let col_offset = d_col.signum();

        // check the path between from and to pos to determine blocking piece
        match from_piece.piece_type {
            Bishop | Queen | Rook | Pawn => {
                if let Some(block_pos) =
                    self.check_clear_path(from_pos, Some(to_pos), row_offset, col_offset)
                {
                    return Err(MoveError::Blocked { at: (block_pos) });
                } else {
                    return Ok(());
                }
            }
            _ => Ok(()),
        }
    }

    // check clear path

    pub fn check_clear_path(
        &self,
        from_pos: Position,
        to_pos: Option<Position>,
        row_offset: i8,
        col_offset: i8,
    ) -> Option<Position> {
        let mut path_row = row_offset + from_pos.row;
        let mut path_col = col_offset + from_pos.col;

        loop {
            let pos = Position {
                row: path_row,
                col: path_col,
            };

            if !self.pos_on_board(pos) {
                break;
            }

            if let Some(target) = to_pos {
                if pos == target {
                    return None;
                }
            }

            if self.get(pos).is_some() {
                return Some(pos);
            }

            path_row += row_offset;
            path_col += col_offset;
        }

        None
    }

    pub fn is_in_check(&self) -> bool {
        use PieceType::*;

        // origin from_pos the king:: Color = moveturn color
        // assume it can move like all other pieces and see if it hits a piece with that piecetype

        let king_pos = match self.move_turn {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        }
        .expect("king position not set");

        // check for queen bishop and rooks
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                if let Some(hit_pos) = self.check_clear_path(king_pos, None, row_offset, col_offset)
                {
                    if let Some(hit_piece) = self.get(hit_pos) {
                        let (d_row, d_col) = hit_pos.delta(king_pos);

                        if hit_piece.color == self.move_turn {
                            continue;
                        }

                        match hit_piece.piece_type {
                            Bishop | Queen | Rook => {
                                if hit_piece.move_shape_ok(d_row, d_col, false, hit_pos.row) {
                                    return true;
                                }
                            }
                            Pawn | King => {
                                if hit_piece.move_shape_ok(d_row, d_col, true, hit_pos.row) {
                                    return true;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }

        // King, pawn and bishop has to_pos be a little different
        const KNIGHT_MOV: [(i8, i8); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for (row_offset, col_offset) in KNIGHT_MOV {
            let hit_pos = Position {
                row: king_pos.row + row_offset,
                col: king_pos.col + col_offset,
            };

            if self.pos_on_board(hit_pos) {
                if let Some(hit_piece) = self.get(hit_pos) {
                    if hit_piece.color == self.move_turn {
                        continue;
                    }

                    if hit_piece.piece_type == Knight {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::*;

    fn pos(r: i8, c: i8) -> Position { Position { row: r, col: c } }

    // chat helped me we this i was lazy
    fn fen_to_ascii_array(fen: &str) -> [String; 8] {
        let mut rows: Vec<String> = Vec::new();

        for row in fen.split('/') {
            let mut expanded = String::new();

            for ch in row.chars() {
                if ch.is_ascii_digit() {
                    let n = ch.to_digit(10).unwrap();
                    for _ in 0..n {
                        expanded.push('.');
                    }
                } else {
                    expanded.push(ch);
                }
            }

            rows.push(expanded);
        }

        assert_eq!(rows.len(), 8, "FEN must have 8 ranks");

        rows.try_into().unwrap()
    }

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

    fn dfs(b: &Board, d: usize, depth_total: usize, totals: &mut [usize]) {
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

    fn perft_layers(board: &mut Board, depth:usize) -> Vec<usize>{
        assert!(depth >= 1);
        let mut totals = vec![0usize; depth];
        dfs(board, depth, depth, &mut totals);
        totals
    }

    fn all_legal_moves_for_fen(fen: &str, depth:usize) -> Vec<usize>{
        let mut board = Board::new();
        let board_ascii = fen_to_ascii_array(fen);

        // apperently we have to do this????? ofcourse chat helped me figure this out. but i think i get it
        let mut board_ascii_refs: [&str; 8] = [
            &board_ascii[0],
            &board_ascii[1],
            &board_ascii[2],
            &board_ascii[3],
            &board_ascii[4],
            &board_ascii[5],
            &board_ascii[6],
            &board_ascii[7],
        ];

        board_ascii_refs.reverse();

        board.setup_ascii(board_ascii_refs);

        perft_layers(&mut board, depth)
    }

    #[test]
    fn test_all_legal_moves_pos_1() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let expected: Vec<usize> = vec![20, 400, 8902];

        let depth = 3;
        let totals = all_legal_moves_for_fen(fen, depth);

        assert_eq!(totals, expected);
    }

    #[test]
    fn test_all_legal_moves_pos_2() {
        // this test can only do 1 depth for now as it requires castles and en passants.
        let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R";
        let expected: Vec<usize> = vec![46, 2039, 97862]; 

        let depth = 3;
        let totals = all_legal_moves_for_fen(fen, depth);

        assert_eq!(totals, expected);
    }

    #[test]
    fn test_all_legal_moves_pos_3() {
        // this test can only do 2 depth for now as it requires castles and en passants for the third.
        let fen: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8";
        let expected: Vec<usize> = vec![14, 191, 2812]; 

        let depth = 3;
        let totals = all_legal_moves_for_fen(fen, depth);

        assert_eq!(totals, expected);
    }
}