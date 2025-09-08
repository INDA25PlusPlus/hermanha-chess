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
        if !from_piece.move_shape_ok(d_row, d_col, capture) {
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
                                if hit_piece.move_shape_ok(d_row, d_col, false) {
                                    return true;
                                }
                            }
                            Pawn | King => {
                                if hit_piece.move_shape_ok(d_row, d_col, true) {
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