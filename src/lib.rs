pub mod pieces;
pub mod board;
pub mod rules;
pub mod movegen;

pub use board::{Board, Position, BOARD_COLS, BOARD_ROWS};
pub use pieces::{Color, Piece, PieceType};
pub use rules::{MoveError, MoveOk};

pub const STARTING_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Board {
    pub fn start_pos() -> Self {
        let mut board = Board::new();
        board.setup_fen(STARTING_BOARD);
        board
    }

    pub fn play(&mut self, from: (i8,i8), to: (i8, i8)) -> Result<MoveOk, MoveError> {
        let from_pos = Position{row: from.0, col: from.1};
        let to_pos = Position{row: to.0, col: to.1};
        self.move_piece(from_pos, to_pos, None)
    }

    pub fn legal_moves(&self) -> Vec<(Position, Position, Option<PieceType>)> {
        movegen::all_legal_moves(self)
    }

    pub fn perft_layers(&mut self, depth: usize) -> Vec<usize> {
        movegen::perft_layers(self, depth)
    }
}