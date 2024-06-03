use crate::piece::*;
use std::fmt;
// #[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Board(pub Vec<Option<Piece>>);
impl Board {
    pub fn new() -> Self {
        let mut b = Board(vec![None; 64]);
        b.set();
        b
    }

    pub fn set(&mut self) {
        self.0[0] = Some(Piece {
            piece: Type::Rook,
            color: Color::Black,
            position: 0,
        });
        self.0[1] = Some(Piece {
            piece: Type::Knight,
            color: Color::Black,
            position: 1,
        });
        self.0[2] = Some(Piece {
            piece: Type::Bishop,
            color: Color::Black,
            position: 2,
        });
        self.0[3] = Some(Piece {
            piece: Type::Queen,
            color: Color::Black,
            position: 3,
        });
        self.0[4] = Some(Piece {
            piece: Type::King,
            color: Color::Black,
            position: 4,
        });
        self.0[5] = Some(Piece {
            piece: Type::Bishop,
            color: Color::Black,
            position: 5,
        });
        self.0[6] = Some(Piece {
            piece: Type::Knight,
            color: Color::Black,
            position: 6,
        });
        self.0[7] = Some(Piece {
            piece: Type::Rook,
            color: Color::Black,
            position: 7,
        });
        for i in 8..16 {
            self.0[i] = Some(Piece {
                piece: Type::Pawn,
                color: Color::Black,
                position: i as u8,
            });
        }
        for i in 48..56 {
            self.0[i] = Some(Piece {
                piece: Type::Pawn,
                color: Color::White,
                position: i as u8,
            });
        }
        self.0[56] = Some(Piece {
            piece: Type::Rook,
            color: Color::White,
            position: 56,
        });
        self.0[57] = Some(Piece {
            piece: Type::Knight,
            color: Color::White,
            position: 57,
        });
        self.0[58] = Some(Piece {
            piece: Type::Bishop,
            color: Color::White,
            position: 58,
        });
        self.0[59] = Some(Piece {
            piece: Type::Queen,
            color: Color::White,
            position: 59,
        });
        self.0[60] = Some(Piece {
            piece: Type::King,
            color: Color::White,
            position: 60,
        });
        self.0[61] = Some(Piece {
            piece: Type::Bishop,
            color: Color::White,
            position: 61,
        });
        self.0[62] = Some(Piece {
            piece: Type::Knight,
            color: Color::White,
            position: 62,
        });
        self.0[63] = Some(Piece {
            piece: Type::Rook,
            color: Color::White,
            position: 63,
        });
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..64 {
            if x % 8 == 0 {
                write!(f, "\n")?;
            }
            if let Some(piece) = self.0[x] {
                write!(f, "{} ", piece)?;
            } else {
                write!(f, ". ")?;
            }
        }
        Ok(())
    }
}
