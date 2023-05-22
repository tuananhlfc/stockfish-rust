use crate::types::*;

enum Side {
    White,
    Black,
}

pub struct Position {
    board: [Piece; 64],
    side_to_move: Side,
}

const NORTH: isize = 8;
const EAST: isize = 1;
const SOUTH: isize = -NORTH;
const WEST: isize = -EAST;
const PIECE_TO_CHAR: &str = " PNBRQK  pnbrqk"; // borrow this idea from office Stockfish

impl Position {
    pub fn new() -> Self {
        let mut position = Position {
            board: [0; 64],
            side_to_move: Side::White,
        };
        position.set_position(START_POS_FEN);
        position
    }
    pub fn set_position(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let piece_placement = parts[0];
        let mut sq: isize = 56; // TODO: remove magic number
        for ch in piece_placement.chars() {
            if ch.is_digit(10) {
                sq += EAST * ch.to_digit(10).unwrap() as isize;
            } else if ch == '/' {
                sq += 2 * SOUTH;
            } else if PIECE_TO_CHAR.contains(ch) {
                let idx = PIECE_TO_CHAR.find(ch).unwrap();
                self.set_piece(idx, sq as usize);
                sq += 1;
            }
        }
        // active color
        self.side_to_move = match parts[1] {
            "w" => Side::White,
            _ => Side::Black,
        };
        // castling right
        // 4. En passant square.
        // 5-6. Halfmove clock and fullmove number
    }
    pub fn do_move(&mut self, m: &str) {
        println!("Move {m}")
    }
    fn set_piece(&mut self, pc: usize, sq: usize) {
        // println!("Sq {sq} = {pc}")
        self.board[sq] = pc;
    }
}
