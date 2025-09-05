use crate::pieces::{Piece, PieceType, Color};

pub const BOARD_ROWS: u8 = 8;
pub const BOARD_COLS: u8 = 8;


#[derive(Debug, Clone, Copy, PartialEq)] // chat thought me this :D
pub struct Position{
    pub row: u8,
    pub col: u8,
}

pub struct Board{
    pub squares: [[Option<Piece>; BOARD_COLS as usize]; BOARD_ROWS as usize],
    pub move_turn: Color,
}

impl Board{
    pub fn new() -> Self{
        Board{
            squares: [[None; BOARD_COLS as usize]; BOARD_ROWS as usize],
            move_turn: Color::White,
        }
    }

    pub fn get(&self, position: Position) -> Option<Piece>{
        self.squares[position.row as usize][position.col as usize]
    }

    pub fn set(&mut self, position: Position, piece: Option<Piece>){
        self.squares[position.row as usize][position.col as usize] = piece;
    }

    pub fn setup_standard(&mut self){
        self.squares = [[None; BOARD_COLS as usize]; BOARD_ROWS as usize];

        // Place pawns
        for col in 0..BOARD_ROWS {
            self.squares[1][col as usize] = Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
            self.squares[6][col as usize] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        }
        // Place rooks
        self.squares[0][0] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        self.squares[0][7] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        self.squares[7][0] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
        self.squares[7][7] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });

        // Place knights
        self.squares[0][1] = Some(Piece { piece_type: PieceType::Knight, color: Color::White });
        self.squares[0][6] = Some(Piece { piece_type: PieceType::Knight, color: Color::White });
        self.squares[7][1] = Some(Piece { piece_type: PieceType::Knight, color: Color::Black });
        self.squares[7][6] = Some(Piece { piece_type: PieceType::Knight, color: Color::Black });

        // Place bishops
        self.squares[0][2] = Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
        self.squares[0][5] = Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
        self.squares[7][2] = Some(Piece { piece_type: PieceType::Bishop, color: Color::Black });
        self.squares[7][5] = Some(Piece { piece_type: PieceType::Bishop, color: Color::Black });

        // Place queens
        self.squares[0][3] = Some(Piece { piece_type: PieceType::Queen, color: Color::White });
        self.squares[7][3] = Some(Piece { piece_type: PieceType::Queen, color: Color::Black });

        // Place kings
        self.squares[0][4] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        self.squares[7][4] = Some(Piece { piece_type: PieceType::King, color: Color::Black });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_starting_board() {
        let mut board = Board::new();
        board.setup_standard();
        println!("{:?}", board.squares)
    }
}