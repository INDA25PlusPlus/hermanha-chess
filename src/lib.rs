pub mod board;
pub mod movegen;
pub mod pieces;
pub mod rules;

pub use board::{BOARD_COLS, BOARD_ROWS, Board, Position};
pub use pieces::{Color, Piece, PieceType};
pub use rules::{GameResult, MoveError, MoveOk};

pub const STARTING_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Board {
    pub fn start_pos() -> Self {
        let mut board = Board::new();
        board.setup_fen(STARTING_BOARD);
        board
    }

    pub fn play(
        &mut self,
        from: (i8, i8),
        to: (i8, i8),
        prom_piece_type: Option<PieceType>,
    ) -> Result<MoveOk, MoveError> {
        let from_pos = Position {
            row: from.0,
            col: from.1,
        };
        let to_pos = Position {
            row: to.0,
            col: to.1,
        };
        self.move_piece(from_pos, to_pos, prom_piece_type)
    }

    pub fn legal_moves(&self) -> Vec<(Position, Position, Option<PieceType>)> {
        movegen::all_legal_moves(self)
    }

    pub fn perft_layers(&mut self, depth: usize) -> Vec<usize> {
        movegen::perft_layers(self, depth)
    }

    pub fn game_over(&self) -> Option<GameResult> {
        if self.is_check_mate() {
            Some(GameResult::Checkmate(match self.move_turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }))
        } else if self.is_stale_mate() {
            Some(GameResult::Stalemate)
        } else {
            None
        }
    }
}
