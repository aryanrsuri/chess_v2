use crate::board;
use crate::piece;

pub struct Game {
    board: board::Board,
    turn: piece::Color,
    state: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
            turn: piece::Color::White,
            state: true,
        }
    }
}
