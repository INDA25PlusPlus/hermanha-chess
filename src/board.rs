use crate::pieces::{Color, Piece, PieceType};

pub const BOARD_ROWS: i8 = 8;
pub const BOARD_COLS: i8 = 8;

// ASCII board
pub const FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; BOARD_COLS as usize]; BOARD_ROWS as usize],
    pub move_turn: Color,
    pub white_king: Option<Position>,
    pub black_king: Option<Position>, //cache the kings insted of looping through board looking for it??? good??
    pub en_passant: Option<Position>
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [[None; BOARD_COLS as usize]; BOARD_ROWS as usize],
            move_turn: Color::White,
            white_king: None,
            black_king: None,
            en_passant: None
        }
    }

    pub fn get(&self, position: Position) -> Option<Piece> {
        if !self.pos_on_board(position) {
            return None;
        }
        self.squares[position.row as usize][position.col as usize]
    }

    pub fn set(&mut self, position: Position, piece: Option<Piece>) {
        if !self.pos_on_board(position) {
            return;
        }

        if let Some(p) = piece {
            if p.piece_type == PieceType::King {
                match p.color {
                    Color::Black => self.black_king = Some(position),
                    Color::White => self.white_king = Some(position),
                }
            }
        }

        self.squares[position.row as usize][position.col as usize] = piece;
    }

    pub fn setup_fen(&mut self, fen: &str) {
        self.squares = [[None; BOARD_COLS as usize]; BOARD_ROWS as usize];
        self.white_king = None;
        self.black_king = None;
        self.en_passant = None;

        let mut fen_row = 0;
        let mut col = 0;
        let mut parts = fen.split_whitespace();
        let board_part = parts.next().unwrap_or("");
        for ch in board_part.chars() {
            match ch {
                '/' => {
                    fen_row += 1;
                    col = 0;
                }
                '1'..='8' => {
                    col += ch.to_digit(10).unwrap() as i8;
                }
                _ => {
                    // Flip the row so that FEN's row 0 becomes board's row 7
                    let row = BOARD_ROWS - 1 - fen_row;
                    let pos = Position { row, col };
                    let piece = match ch {
                        'p' => Some(Piece::new(PieceType::Pawn, Color::Black)),
                        'r' => Some(Piece::new(PieceType::Rook, Color::Black)),
                        'n' => Some(Piece::new(PieceType::Knight, Color::Black)),
                        'b' => Some(Piece::new(PieceType::Bishop, Color::Black)),
                        'q' => Some(Piece::new(PieceType::Queen, Color::Black)),
                        'k' => Some(Piece::new(PieceType::King, Color::Black)),
                        'P' => Some(Piece::new(PieceType::Pawn, Color::White)),
                        'R' => Some(Piece::new(PieceType::Rook, Color::White)),
                        'N' => Some(Piece::new(PieceType::Knight, Color::White)),
                        'B' => Some(Piece::new(PieceType::Bishop, Color::White)),
                        'Q' => Some(Piece::new(PieceType::Queen, Color::White)),
                        'K' => Some(Piece::new(PieceType::King, Color::White)),
                        _ => None,
                    };
                    self.set(pos, piece);
                    col += 1;
                }
            }
        }
    }

    pub fn print_ascii(&self) {
        // took some help from chat to debug using this. easier to see
        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                match self.squares[row as usize][col as usize] {
                    Some(piece) => {
                        let ch = match (piece.piece_type, piece.color) {
                            (PieceType::Pawn, Color::White) => 'P',
                            (PieceType::Rook, Color::White) => 'R',
                            (PieceType::Knight, Color::White) => 'N',
                            (PieceType::Bishop, Color::White) => 'B',
                            (PieceType::Queen, Color::White) => 'Q',
                            (PieceType::King, Color::White) => 'K',

                            (PieceType::Pawn, Color::Black) => 'p',
                            (PieceType::Rook, Color::Black) => 'r',
                            (PieceType::Knight, Color::Black) => 'n',
                            (PieceType::Bishop, Color::Black) => 'b',
                            (PieceType::Queen, Color::Black) => 'q',
                            (PieceType::King, Color::Black) => 'k',
                        };
                        print!("{}", ch);
                    }
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_starting_board() {
        let mut board = Board::new();
        board.setup_fen(FEN);
    }
}
