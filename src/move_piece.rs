use crate::board::{Board, Position};
use crate::pieces::{Color, PieceType};

impl Position {
    pub fn delta(self, other: Position) -> (i8,i8){
        (other.row as i8 - self.row as i8, other.col as i8 - self.col as i8)
    }
}

impl Board{
    pub fn move_piece(&mut self, from: Position, to: Position) -> bool{ // returns bool for now just for debugging
        
        // check valid moves
        if !self.psuedo_legal_move(from, to) {
            return false;
        }

        if let Some(piece) = self.get(from) {
            self.set(from, None);
            self.set(to, Some(piece));
            if self.move_turn == Color::Black{
                self.move_turn = Color::White
            }else{
                self.move_turn = Color::Black
            }
            true
        } else {
            false
        }
    }

    pub fn pos_on_board(&self, pos:Position) -> bool{
        (pos.row as usize) <= self.squares.len() && (pos.col as usize) <= self.squares[0].len()
    }

    pub fn psuedo_legal_move(&self, from: Position, to: Position) -> bool{
        // check to position on board, believe we dont have to check from pos
        if !self.pos_on_board(to) || from == to {return false;}

        // get piece on from and to position
        let Some(from_piece) = self.get(from) else {return false};
        if from_piece.color != self.move_turn {
            return false;
        }

        // check if own piece is on square
        if let Some(to_piece) = self.get(to) {
            if to_piece.color == from_piece.color{
                return false;
            }
        };

        let (d_row, d_col) = from.delta(to);

        // check if piece allows move shape
        if !from_piece.move_shape_ok(d_row, d_col, from_piece.color) {
            return false;
        }

        use PieceType::*;

        let row_offset = d_row.signum();
        let col_offset=d_col.signum();

        match from_piece.piece_type {
            Bishop | Queen | Rook | Pawn => self.check_clear_path(from, to, row_offset, col_offset),
            _ => return true
        }

    }

    // check clear path

    pub fn check_clear_path(&self, from: Position, to: Position, row_offset:i8, col_offset:i8)->bool{
        let mut path_row = row_offset + from.row as i8;
        let mut path_col = col_offset + from.col as i8;


        while path_row != to.row as i8 || path_col != to.col as i8 {
            match self.get(Position { row: (path_row as u8), col: (path_col as u8) }){
                None => {path_row += row_offset; path_col += col_offset},
                _ => return false
            }
        } 
        return true

        }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move(){
        let mut from = Position{row:1, col:7};
        let mut to = Position{row:3, col:7};
        let mut board = Board::new();
        board.setup_standard();
        println!("\n THIS MOVE IS {}", board.move_piece(from, to));
        from.col = 6;
        to.col = 6;
    }
}