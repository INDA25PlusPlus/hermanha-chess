#[cfg(test)]
mod tests {
    use hermanha_chess::*;

    #[test]
    fn test_all_legal_moves_pos_1() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        let mut board = Board::new();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![20, 400, 8902];
        let depth = 3;
        let totals = board.perft_layers(depth);

        assert_eq!(totals, expected);
    }

    #[test]
    fn test_all_legal_moves_pos_2() {
        let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R";
        let mut board = Board::new();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![48, 2039, 97862]; 
        let depth = 3;
        let totals = board.perft_layers(depth);

        assert_eq!(totals, expected);
    }

    #[test]
    fn test_all_legal_moves_pos_3() {
        let fen: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8";
        let mut board = Board::new();
        board.setup_fen(fen);
        
        let expected: Vec<usize> = vec![14, 191, 2812]; 
        let depth = 3;
        let totals = board.perft_layers(depth);

        assert_eq!(totals, expected);
    }
}