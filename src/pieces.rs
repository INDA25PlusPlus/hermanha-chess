#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn move_shape_ok(self, d_row: i8, d_col:i8, color: Color) -> bool {
        use PieceType::*;
        let abs_dr = d_row.abs();
        let abs_dc = d_col.abs();

        match self.piece_type {
            Pawn => {
                let fwd = match color { Color::White => 1, Color::Black => -1}; // got some help with this logic
                d_col == 0 && (d_row == fwd || d_row == 2*fwd)
            }
            King => abs_dr <= 1 && abs_dc <= 1,
            Bishop => abs_dc == abs_dr,
            Rook => abs_dc == 0 || abs_dr == 0,
            Knight => (abs_dc == 1 && abs_dr == 2) || (abs_dc == 2 && abs_dr == 1),
            Queen => (abs_dc == abs_dr) || (abs_dc == 0 || abs_dr == 0),
        }
    }
}

