#[cfg(test)]
mod tests {
    use hermanha_chess::*;
    const DEPTH: usize = 4;

    #[test]
    fn test_all_legal_moves_pos_1() {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![20, 400, 8902, 197281, 4865609];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }

    #[test]
    fn test_all_legal_moves_pos_2() {
        let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R";
        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![48, 2039, 97862, 4085603, 193690690];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }

    #[test]
    fn test_all_legal_moves_pos_3() {
        let fen: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8";
        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![14, 191, 2812, 43238];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }

    #[test]
    fn test_all_legal_moves_pos_4() {
        let fen: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1";
        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![6, 264, 9467, 422333];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }

    #[test]
    fn test_all_legal_moves_pos_5() {
        let fen: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R";
        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![44, 1486, 62379, 2103487];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }

    #[test]
    fn test_all_legal_moves_pos_6() {
        let fen: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1";
        let mut board = Board::default();
        board.setup_fen(fen);

        let expected: Vec<usize> = vec![46, 2079, 89890, 3894594];
        let totals = board.perft_layers(DEPTH);

        assert_eq!(totals, expected[..DEPTH]);
    }
}
