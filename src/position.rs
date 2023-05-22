use pleco::Board;

pub struct Position {
    board: Board,
}

impl Position {
    pub fn new() -> Self {
        let position = Position {
            board: Board::start_pos(),
        };
        position
    }
    pub fn set_position(&mut self, fen: &str) {
        match Board::from_fen(fen) {
            Ok(b) => self.board = b,
            Err(e) => return,
        }
    }
    pub fn do_move(&mut self, m: &str) {
        self.board.apply_uci_move(m);
    }
}
