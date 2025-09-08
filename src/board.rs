use crate::pieces::{Color, Piece, PieceType};

pub const BOARD_ROWS: i8 = 8;
pub const BOARD_COLS: i8 = 8;

// ASCII board
pub const ASCII: [&str; 8] = [
    "RNBQKBNR", "PPPPPPPP", "........", "........", "........", "........", "pppppppp", "rnbqkbnr",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

pub struct Board {
    pub squares: [[Option<Piece>; BOARD_COLS as usize]; BOARD_ROWS as usize],
    pub move_turn: Color,
    pub white_king: Option<Position>,
    pub black_king: Option<Position>, //cache the kings insted of looping through board looking for it??? good??
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [[None; BOARD_COLS as usize]; BOARD_ROWS as usize],
            move_turn: Color::White,
            white_king: None,
            black_king: None,
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

    // loop through list of strings as ascii characters to place pieces on board
    pub fn setup_ascii(&mut self, ascii: [&str; 8]) {
        self.squares = [[None; BOARD_COLS as usize]; BOARD_ROWS as usize];
        self.white_king = None;
        self.black_king = None;

        for (row, row_str) in ascii.iter().enumerate() {
            for (col, ch) in row_str.chars().enumerate() {
                let pos = Position {
                    row: row as i8,
                    col: col as i8,
                };

                let piece = match ch {
                    'p' => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: Color::Black,
                    }),
                    'r' => Some(Piece {
                        piece_type: PieceType::Rook,
                        color: Color::Black,
                    }),
                    'n' => Some(Piece {
                        piece_type: PieceType::Knight,
                        color: Color::Black,
                    }),
                    'b' => Some(Piece {
                        piece_type: PieceType::Bishop,
                        color: Color::Black,
                    }),
                    'q' => Some(Piece {
                        piece_type: PieceType::Queen,
                        color: Color::Black,
                    }),
                    'k' => Some(Piece {
                        piece_type: PieceType::King,
                        color: Color::Black,
                    }),
                    'P' => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: Color::White,
                    }),
                    'R' => Some(Piece {
                        piece_type: PieceType::Rook,
                        color: Color::White,
                    }),
                    'N' => Some(Piece {
                        piece_type: PieceType::Knight,
                        color: Color::White,
                    }),
                    'B' => Some(Piece {
                        piece_type: PieceType::Bishop,
                        color: Color::White,
                    }),
                    'Q' => Some(Piece {
                        piece_type: PieceType::Queen,
                        color: Color::White,
                    }),
                    'K' => Some(Piece {
                        piece_type: PieceType::King,
                        color: Color::White,
                    }),
                    '.' => None,
                    _ => None,
                };

                self.set(pos, piece);
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
        board.setup_ascii(ASCII);
    }
}
