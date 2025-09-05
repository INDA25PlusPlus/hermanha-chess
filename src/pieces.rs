#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
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

    pub fn offset(&self) -> &'static [(i8, i8)] {
        use PieceType::*;
        match self.piece_type {
            Pawn => &[(0, 1)],
            Bishop => &[(1, 1), (-1, 1), (1, -1), (-1, -1)],
            Rook => &[(1, 0), (-1, 0), (0, 1), (0, -1)],
            Knight => &[
                (2, 1), (1, 2), (-1, 2), (-2, 1),
                (-2, -1), (-1, -2), (1, -2), (2, -1)
            ],
            Queen => &[
                (1, 0), (-1, 0), (0, 1), (0, -1),
                (1, 1), (-1, 1), (1, -1), (-1, -1)
            ],
            King => &[
                (1, 0), (-1, 0), (0, 1), (0, -1),
                (1, 1), (-1, 1), (1, -1), (-1, -1)
            ],
        }
    }
}

