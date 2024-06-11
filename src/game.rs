use crate::board;
use crate::piece;

pub struct Game {
    pub board: board::Board,
    pub turn: piece::Color,
    pub state: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: board::Board::new(),
            turn: piece::Color::White,
            state: true,
        }
    }

    pub fn pass(&mut self, input: &str) {
        let success = self.board.set(input.trim(), self.turn);
        if !success {
            println!("Illegal move, try again");
            return;
        }
        if self.turn == piece::Color::White {
            self.turn = piece::Color::Black
        } else {
            self.turn = piece::Color::White
        }
    }
}
