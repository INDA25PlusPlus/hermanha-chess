use crate::board::{Board, Position};
use crate::pieces::{Color, PieceType};

// ASCII board

impl Position {
    pub fn delta(self, other: Position) -> (i8, i8) {
        (
            other.row as i8 - self.row as i8,
            other.col as i8 - self.col as i8,
        )
    }
}

impl Board {
    pub fn move_piece(&mut self, from: Position, to: Position) -> bool {
        // returns bool for now just for debugging

        if let Some(from_piece) = self.get(from) {
            if !self.psuedo_legal_move(from, to) {
                return false;
            };

            let to_piece = self.get(to);

            self.set(from, None);
            self.set(to, Some(from_piece));

            if self.is_in_check() {
                self.set(to, to_piece);
                self.set(from, Some(from_piece));
                return false;
            }

            self.move_turn = match self.move_turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }

        true
    }

    pub fn pos_on_board(&self, pos: Position) -> bool {
        pos.row >= 0
            && pos.row < self.squares.len() as i8
            && pos.col >= 0
            && pos.col < self.squares[0].len() as i8
    }

    pub fn psuedo_legal_move(&self, from: Position, to: Position) -> bool {
        // check to position on board, believe we dont have to check from pos
        if !self.pos_on_board(to) || from == to {
            return false;
        }

        // get piece on from and to position
        let Some(from_piece) = self.get(from) else {
            return false;
        };
        if from_piece.color != self.move_turn {
            return false;
        }

        // check if own piece color is on square
        if let Some(to_piece) = self.get(to) {
            if to_piece.color == from_piece.color {
                return false;
            }
        };

        let (d_row, d_col) = from.delta(to);

        // check if piece allows move shape
        if !from_piece.move_shape_ok(d_row, d_col, false) {
            return false;
        }

        use PieceType::*;

        let row_offset = d_row.signum();
        let col_offset = d_col.signum();

        match from_piece.piece_type {
            Bishop | Queen | Rook | Pawn => self
                .check_clear_path(from, Some(to), row_offset, col_offset)
                .is_none(), // should return true if check clear path returns None. false if it returns a piece.
            _ => true,
        }
    }

    // check clear path

    pub fn check_clear_path(
        &self,
        from: Position,
        to: Option<Position>,
        row_offset: i8,
        col_offset: i8,
    ) -> Option<Position> {
        let mut path_row = row_offset + from.row as i8;
        let mut path_col = col_offset + from.col as i8;

        while self.pos_on_board(Position {
            row: (path_row),
            col: (path_col),
        }) {
            let pos = Position {
                row: path_row,
                col: path_col,
            };

            if let Some(target) = to {
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
        use Color::*;
        use PieceType::*;

        // origin from the king:: Color = moveturn color
        // assume it can move like all other pieces and see if it hits a piece with that piecetype

        let Some(king_pos) = (match self.move_turn {
            White => self.white_king,
            Black => self.black_king,
        }) else {
            panic!("king position not set")
        };

        // check for queen bishop and rooks
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                if let Some(hit_pos) = self.check_clear_path(king_pos, None, row_offset, col_offset)
                {
                    if let Some(hit_piece) = self.get(hit_pos) {
                        let (d_row, d_col) = king_pos.delta(hit_pos);

                        if hit_piece.color == self.move_turn {
                            continue;
                        }

                        match hit_piece.piece_type {
                            Bishop | Queen | Rook => {
                                if hit_piece.move_shape_ok(row_offset, col_offset, false) {
                                    println!("{:?}", self.move_turn);
                                    println!("{:?}", king_pos);
                                    println!("{:?}", hit_piece.piece_type);
                                    println!("{:?}", hit_pos);
                                    return true;
                                }
                            }
                            Pawn | King => {
                                if hit_piece.move_shape_ok(-d_row, -d_col, true) {
                                    println!("{:?}", hit_piece.piece_type);
                                    println!("{:?}", hit_pos);
                                    return true;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }

        // King, pawn and bishop has to be a little different
        const KNIGHT_MOV: [[i8; 2]; 8] = [
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
        ];

        for [row_offset, col_offset] in KNIGHT_MOV {
            let pos_row = king_pos.row as i8 + row_offset;
            let pos_col = king_pos.col as i8 + col_offset;
            let pos = Position {
                row: pos_row,
                col: pos_col,
            };

            if !self.pos_on_board(pos) {
                continue;
            }

            if let Some(piece) = self.get(pos) {
                if piece.piece_type == Knight {
                    println!("{:?}", self.move_turn);
                    println!("{:?}", king_pos);
                    println!("hit knight");
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut from = Position { row: 0, col: 4 };
        let mut to = Position { row: 0, col: 5 };
        let mut board = Board::new();

        let mut ascii: [&str; 8] = [
            "...k....", "........", "........", "........", "........", ".....b..", "r.....p.",
            "....K...",
        ];

        ascii.reverse();

        board.setup_ascii(ascii);
        // board.print_ascii();
        println!("\n THIS MOVE IS {}", board.move_piece(from, to));
        // board.print_ascii();

        from.col = 6;
        to.col = 6;
    }
}
