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
        let success = self.board.set(input, self.turn);
        if !success {
            println!("Illegal move, try again");
            return;
        }

        // Perhaps piece color (and turn) should just be a boolean to make this trivial
        if self.turn == piece::Color::White {
            self.turn = piece::Color::Black
        } else {
            self.turn = piece::Color::White
        }
    }
}
