use crate::board::{BOARD_COLS, BOARD_ROWS, Board, Position};
use crate::movegen::all_legal_moves;
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
    KingHasMoved,
    RookHasMoved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal { is_capture: bool },
    EnPassant,
    Castle,
    PawnPromotion { is_capture: bool },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveOk {
    Done,
    NeedsPromotion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    Checkmate(Color),
    Stalemate,
}

impl Board {
    pub fn move_piece(
        &mut self,
        from_pos: Position,
        to_pos: Position,
        prom_piece_type: Option<PieceType>,
    ) -> Result<MoveOk, MoveError> {
        self.basic_precheck(from_pos, to_pos)?;
        let move_type: MoveType = self.classify_move_type(from_pos, to_pos);

        match move_type {
            MoveType::Castle => self.castle_is_legal(from_pos, to_pos),
            MoveType::EnPassant => self.normal_is_legal(from_pos, to_pos, true),
            MoveType::Normal { is_capture } => self.normal_is_legal(from_pos, to_pos, is_capture),
            MoveType::PawnPromotion { is_capture } => {
                self.normal_is_legal(from_pos, to_pos, is_capture)
            }
        }?;

        self.move_in_check(from_pos, to_pos, move_type)?;

        if let MoveType::PawnPromotion { .. } = move_type
            && prom_piece_type.is_none()
        {
            return Ok(MoveOk::NeedsPromotion);
        }

        self.set_values(from_pos, to_pos, move_type, prom_piece_type);

        Ok(MoveOk::Done)
    }

    pub fn basic_precheck(&self, from_pos: Position, to_pos: Position) -> Result<(), MoveError> {
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
        if let Some(to_piece) = self.get(to_pos)
            && to_piece.color == from_piece.color
        {
            return Err(MoveError::CaptureOwn);
        };

        Ok(())
    }

    pub fn classify_move_type(&self, from_pos: Position, to_pos: Position) -> MoveType {
        // en_passant
        if self.is_en_passant(from_pos, to_pos) {
            return MoveType::EnPassant;
        }

        // castle
        if self.is_castle(from_pos, to_pos) {
            return MoveType::Castle;
        }

        let is_capture = self.is_capture(from_pos, to_pos);

        // promotion
        if self.is_promotion(from_pos, to_pos) {
            return MoveType::PawnPromotion { is_capture };
        }

        MoveType::Normal { is_capture }
    }

    pub fn is_capture(&self, from_pos: Position, to_pos: Position) -> bool {
        let from_piece = self.get(from_pos).expect("validated: piece on from_pos");
        if let Some(to_piece) = self.get(to_pos)
            && to_piece.color != from_piece.color
        {
            return true;
        }
        false
    }

    pub fn is_castle(&self, from_pos: Position, to_pos: Position) -> bool {
        let Some(from_piece) = self.get(from_pos) else {
            return false;
        };
        if from_piece.piece_type != PieceType::King {
            return false;
        }

        let (dr, dc) = from_pos.delta(to_pos);
        if dc.abs() != 2 || dr != 0 {
            return false;
        }

        true
    }

    pub fn is_en_passant(&self, from_pos: Position, to_pos: Position) -> bool {
        let from_piece = self.get(from_pos).expect("validated: piece on from_pos");
        if from_piece.piece_type != PieceType::Pawn {
            return false;
        }
        let (dr, dc) = from_pos.delta(to_pos);
        if (dr.abs() != 1) || (dc.abs() != 1) {
            return false;
        }

        if let Some(ep_pos) = self.en_passant
            && to_pos == ep_pos
        {
            return true;
        }
        false
    }

    pub fn is_promotion(&self, from_pos: Position, to_pos: Position) -> bool {
        let from_piece = self.get(from_pos).expect("validated: piece on from_pos");
        if from_piece.piece_type == PieceType::Pawn
            && (to_pos.row == BOARD_ROWS - 1 || to_pos.row == 0)
        {
            return true;
        }

        false
    }

    pub fn find_rook(&self, from_pos: Position, to_pos: Position) -> Position {
        let from_piece = self.get(from_pos).expect("validated: from_pos has piece");

        let (_, dc) = from_pos.delta(to_pos);

        match from_piece.color {
            Color::Black => {
                if dc.signum() == 1 {
                    Position {
                        row: BOARD_ROWS - 1,
                        col: BOARD_COLS - 1,
                    }
                } else {
                    Position {
                        row: BOARD_ROWS - 1,
                        col: 0,
                    }
                }
            }
            Color::White => {
                if dc.signum() == 1 {
                    Position {
                        row: 0,
                        col: BOARD_COLS - 1,
                    }
                } else {
                    Position { row: 0, col: 0 }
                }
            }
        }
    }

    pub fn castle_is_legal(&self, from_pos: Position, to_pos: Position) -> Result<(), MoveError> {
        let from_piece = self.get(from_pos).expect("validated: from_pos has piece");

        let (dr, dc) = from_pos.delta(to_pos);

        if from_piece.has_moved {
            return Err(MoveError::KingHasMoved);
        }

        let expected_king_row = match from_piece.color {
            Color::White => 0,
            Color::Black => 7,
        };
        let expected_king_col = 4;

        if from_pos.row != expected_king_row || from_pos.col != expected_king_col {
            return Err(MoveError::KingHasMoved);
        }

        let rook_pos = self.find_rook(from_pos, to_pos);

        let Some(rook_piece) = self.get(rook_pos) else {
            return Err(MoveError::RookHasMoved);
        };
        if rook_piece.piece_type != PieceType::Rook {
            return Err(MoveError::RookHasMoved);
        }
        if rook_piece.has_moved {
            return Err(MoveError::RookHasMoved);
        }

        if let Some(blocked_pos) = self.check_clear_path(from_pos, Some(rook_pos), dr, dc.signum())
        {
            return Err(MoveError::Blocked { at: (blocked_pos) });
        }

        let path_pos = Position {
            row: from_pos.row,
            col: from_pos.col + dc.signum(),
        };

        if self.is_square_attacked(path_pos) {
            return Err(MoveError::SelfCheck);
        }
        if self.is_square_attacked(from_pos) {
            return Err(MoveError::SelfCheck);
        }
        if self.is_square_attacked(to_pos) {
            return Err(MoveError::SelfCheck);
        }

        Ok(())
    }

    pub fn normal_is_legal(
        &self,
        from_pos: Position,
        to_pos: Position,
        capture: bool,
    ) -> Result<(), MoveError> {
        let from_piece = self.get(from_pos).expect("validate: from_pos has piece");

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
                }
            }
            _ => return Ok(()),
        }

        Ok(())
    }

    /// checks if path is blocked or not
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

            if let Some(target) = to_pos
                && pos == target
            {
                return None;
            }

            if self.get(pos).is_some() {
                return Some(pos);
            }

            path_row += row_offset;
            path_col += col_offset;
        }

        None
    }

    /// Sends rays from pos out in all possible directions and check Horse movement aswell
    /// if it hits a piece, check piece and if its movement is legal
    pub fn is_square_attacked(&self, pos: Position) -> bool {
        use PieceType::*;

        // check for queen bishop and rooks
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                if let Some(hit_pos) = self.check_clear_path(pos, None, row_offset, col_offset)
                    && let Some(hit_piece) = self.get(hit_pos)
                {
                    let (d_row, d_col) = hit_pos.delta(pos);

                    if hit_piece.color != self.move_turn {
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
                row: pos.row + row_offset,
                col: pos.col + col_offset,
            };

            if self.pos_on_board(hit_pos)
                && let Some(hit_piece) = self.get(hit_pos)
                && hit_piece.color != self.move_turn
                && hit_piece.piece_type == Knight
            {
                return true;
            }
        }

        false
    }

    /// This functions checks if a move will give check.
    /// We try to do the move in a clone and checks if king is attacked
    pub fn move_in_check(
        &self,
        from_pos: Position,
        to_pos: Position,
        move_type: MoveType,
    ) -> Result<(), MoveError> {
        let mut board_clone = self.clone();
        let from_piece = board_clone
            .get(from_pos)
            .expect("validated: from_pos has piece");

        if move_type == MoveType::EnPassant {
            let en_passanted_pos = Position {
                row: (from_pos.row),
                col: (to_pos.col),
            };
            board_clone.set(en_passanted_pos, None);
        }

        if move_type == MoveType::Castle {
            let rook_from = board_clone.find_rook(from_pos, to_pos);
            let rook_to = Position {
                row: from_pos.row,
                col: from_pos.col + (to_pos.col - from_pos.col).signum(),
            };
            let rook_piece = board_clone.get(rook_from).expect("validated: rook_from");
            board_clone.set(rook_from, None);
            board_clone.set(rook_to, Some(rook_piece));
        }

        board_clone.set(from_pos, None);
        board_clone.set(to_pos, Some(from_piece));

        let king_pos = if from_piece.piece_type == PieceType::King {
            to_pos
        } else {
            match from_piece.color {
                Color::White => board_clone.white_king,
                Color::Black => board_clone.black_king,
            }
            .expect("validated: king position set")
        };

        if board_clone.is_square_attacked(king_pos) {
            return Err(MoveError::SelfCheck);
        }

        Ok(())
    }

    /// When a move is legal, we need to set alot of values.
    /// first of all move the pieces on the board, then set:
    /// kings position if changed
    /// has moved, for the moved piece
    /// remove pieces if capture (happens automatically if not en passant)
    /// switch move_turn
    pub fn set_values(
        &mut self,
        from_pos: Position,
        to_pos: Position,
        move_type: MoveType,
        prom_piece_type: Option<PieceType>,
    ) {
        let mut from_piece = self.get(from_pos).expect("validated: from_pos has piece");
        let (dr, dc) = from_pos.delta(to_pos);

        let set_ep = matches!(from_piece.piece_type, PieceType::Pawn) && dr.abs() == 2 && dc == 0;

        if move_type == MoveType::EnPassant {
            let en_passanted_pos = Position {
                row: (from_pos.row),
                col: (to_pos.col),
            };
            self.set(en_passanted_pos, None)
        }

        if move_type == MoveType::Castle {
            let rook_from = self.find_rook(from_pos, to_pos);
            let rook_to = Position {
                row: from_pos.row,
                col: from_pos.col + dc.signum(),
            };
            let mut rook_piece = self.get(rook_from).expect("validated: rook_from has piece");
            rook_piece.has_moved = true;
            self.set(rook_from, None);
            self.set(rook_to, Some(rook_piece))
        }

        if let MoveType::PawnPromotion { .. } = move_type {
            let pt = prom_piece_type.expect("validated: prom_piece_type has PieceType");
            from_piece.piece_type = pt
        }

        self.set(from_pos, None);
        from_piece.has_moved = true;
        self.set(to_pos, Some(from_piece));

        self.en_passant = if set_ep {
            Some(Position {
                row: to_pos.row - dr.signum(),
                col: to_pos.col,
            })
        } else {
            None
        };

        self.move_turn = match self.move_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn is_check_mate(&self) -> bool {
        let king_pos = match self.move_turn {
            Color::White => self.white_king.expect("validated: white king position set"),
            Color::Black => self.black_king.expect("validated: black king position set"),
        };

        self.is_square_attacked(king_pos) && all_legal_moves(self).is_empty()
    }

    pub fn is_stale_mate(&self) -> bool {
        let king_pos = match self.move_turn {
            Color::White => self.white_king.expect("validated: white king position set"),
            Color::Black => self.black_king.expect("validated: black king position set"),
        };

        !self.is_square_attacked(king_pos) && all_legal_moves(self).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Color, Piece, PieceType};

    fn create_test_board() -> Board {
        Board::default()
    }

    fn setup_piece(board: &mut Board, pos: Position, piece_type: PieceType, color: Color) {
        board.set(pos, Some(Piece::new(piece_type, color)));
    }

    #[test]
    fn test_basic_precheck_empty_square() {
        let board = create_test_board();
        let from_pos = Position { row: 4, col: 4 };
        let to_pos = Position { row: 4, col: 5 };

        let result = board.basic_precheck(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::EmptyFrom));
    }

    #[test]
    fn test_basic_precheck_same_square() {
        let mut board = create_test_board();
        let pos = Position { row: 4, col: 4 };
        setup_piece(&mut board, pos, PieceType::Pawn, Color::White);

        let result = board.basic_precheck(pos, pos);
        assert_eq!(result, Err(MoveError::SameSquare));
    }

    #[test]
    fn test_basic_precheck_wrong_turn() {
        let mut board = create_test_board();
        board.move_turn = Color::White;
        let from_pos = Position { row: 4, col: 4 };
        let to_pos = Position { row: 4, col: 5 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::Black);

        let result = board.basic_precheck(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::WrongTurn));
    }

    #[test]
    fn test_basic_precheck_out_off_bounds() {
        let mut board = create_test_board();
        board.move_turn = Color::White;
        let from_pos = Position { row: 8, col: 4 };
        let to_pos = Position { row: 8, col: 3 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::Black);

        let result = board.basic_precheck(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::OutOfBounds));
    }

    #[test]
    fn test_basic_precheck_capture_own_piece() {
        let mut board = create_test_board();
        board.move_turn = Color::White;
        let from_pos = Position { row: 4, col: 4 };
        let to_pos = Position { row: 4, col: 5 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::White);
        setup_piece(&mut board, to_pos, PieceType::Rook, Color::White);

        let result = board.basic_precheck(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::CaptureOwn));
    }

    #[test]
    fn test_basic_precheck_valid_move() {
        let mut board = create_test_board();
        board.move_turn = Color::White;
        let from_pos = Position { row: 4, col: 4 };
        let to_pos = Position { row: 4, col: 5 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::White);

        let result = board.basic_precheck(from_pos, to_pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_illegal_shape() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 4 };
        let to_pos = Position { row: 0, col: 6 };
        setup_piece(&mut board, from_pos, PieceType::Bishop, Color::White);

        let result = board.normal_is_legal(from_pos, to_pos, false);
        assert_eq!(result, Err(MoveError::IllegalShape));
    }

    #[test]
    fn test_is_blocked() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 4 };
        let to_pos = Position { row: 3, col: 7 };
        let block_pos = Position { row: 2, col: 6 };
        setup_piece(&mut board, from_pos, PieceType::Bishop, Color::White);
        setup_piece(&mut board, block_pos, PieceType::Bishop, Color::Black);

        let result = board.normal_is_legal(from_pos, to_pos, false);
        assert_eq!(result, Err(MoveError::Blocked { at: block_pos }));
    }

    #[test]
    fn test_is_self_check() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 3 };
        let to_pos = Position { row: 0, col: 4 };
        let attacker_pos = Position { row: 2, col: 6 };
        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, attacker_pos, PieceType::Bishop, Color::Black);

        let result = board.move_in_check(from_pos, to_pos, MoveType::Normal { is_capture: false });
        assert_eq!(result, Err(MoveError::SelfCheck));
    }

    #[test]
    fn test_castle_king_has_moved() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 4 };
        let to_pos = Position { row: 0, col: 6 };
        let rook_pos = Position { row: 0, col: 7 };

        board.set(
            from_pos,
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
                has_moved: true,
            }),
        );
        setup_piece(&mut board, rook_pos, PieceType::Rook, Color::White);

        let result = board.castle_is_legal(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::KingHasMoved));
    }

    #[test]
    fn test_castle_rook_has_moved() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 4 };
        let to_pos = Position { row: 0, col: 6 };
        let rook_pos = Position { row: 0, col: 7 };

        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        board.set(
            rook_pos,
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
                has_moved: true,
            }),
        );

        let result = board.castle_is_legal(from_pos, to_pos);
        assert_eq!(result, Err(MoveError::RookHasMoved));
    }

    #[test]
    fn test_is_castle_true() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 4 };
        let to_pos = Position { row: 0, col: 6 };
        let rook_pos = Position { row: 0, col: 7 };
        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, rook_pos, PieceType::Rook, Color::White);

        let result = board.is_castle(from_pos, to_pos);
        assert!(result);
    }

    #[test]
    fn test_is_promotion_true_white_pawn() {
        let mut board = create_test_board();
        let from_pos = Position { row: 6, col: 4 };
        let to_pos = Position { row: 7, col: 4 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::White);

        let result = board.is_promotion(from_pos, to_pos);
        assert!(result);
    }

    #[test]
    fn test_is_promotion_true_black_pawn() {
        let mut board = create_test_board();
        let from_pos = Position { row: 1, col: 4 };
        let to_pos = Position { row: 0, col: 4 };
        setup_piece(&mut board, from_pos, PieceType::Pawn, Color::Black);

        let result = board.is_promotion(from_pos, to_pos);
        assert!(result);
    }

    #[test]
    fn test_is_check_mate() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 0 };
        let queen_pos = Position { row: 1, col: 2 };
        let rook_pos = Position { row: 0, col: 2 };

        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, rook_pos, PieceType::Rook, Color::Black);
        setup_piece(&mut board, queen_pos, PieceType::Queen, Color::Black);

        let result = board.is_check_mate();
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_stale_mate() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 0 };
        let queen_pos = Position { row: 1, col: 2 };

        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, queen_pos, PieceType::Queen, Color::Black);

        let result = board.is_stale_mate();
        assert_eq!(result, true);
    }

    #[test]
    fn test_game_over_stale_mate() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 0 };
        let queen_pos = Position { row: 1, col: 2 };

        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, queen_pos, PieceType::Queen, Color::Black);

        let result = board.game_over();
        assert_eq!(result, Some(GameResult::Stalemate));
    }

    #[test]
    fn test_game_over_check_mate() {
        let mut board = create_test_board();
        let from_pos = Position { row: 0, col: 0 };
        let queen_pos = Position { row: 1, col: 2 };
        let rook_pos = Position { row: 0, col: 2 };

        setup_piece(&mut board, from_pos, PieceType::King, Color::White);
        setup_piece(&mut board, rook_pos, PieceType::Rook, Color::Black);
        setup_piece(&mut board, queen_pos, PieceType::Queen, Color::Black);

        let result = board.game_over();
        assert_eq!(result, Some(GameResult::Checkmate(Color::Black)));
    }
}
