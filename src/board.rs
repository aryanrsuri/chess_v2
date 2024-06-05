use crate::piece::*;
use core::panic;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Turn {
    from: usize,
    attacker: Option<Piece>,
    to: usize,
    defender: Option<Piece>,
}
// #[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Board(pub Vec<Option<Piece>>);
impl Board {
    pub fn new() -> Self {
        let mut b = Board(vec![None; 64]);
        b.flush();
        b
    }
    pub fn set(&mut self, input: &str) {
        let moves: Vec<&str> = input.split(" ").collect();
        let turn: Turn = self.parse(moves).unwrap();
        if turn.defender.is_none() && turn.attacker.is_some() {
            println!("orig {:#?}", self.get(turn.from));
            let mut attacker = self.0[turn.from].take().unwrap();
            attacker.position = turn.to;
            self.0[turn.to] = Some(attacker);
            println!("new {:#?}", self.get(turn.to))
        }
    }

    pub fn get(&mut self, index: usize) -> Option<Piece> {
        return self.0[index];
    }

    pub fn parse(&mut self, moves: Vec<&str>) -> Option<Turn> {
        if moves.len() != 2 {
            return None;
        }
        let from = &moves[0];
        let mut chars = from.chars();
        let x = chars.next()?;
        let y = chars.next()?;
        let xi = (x as usize) - 'a' as usize;
        let yi = (y as usize) - '1' as usize;
        let a = (yi << 3) | xi;
        let asquare = self.get(a);

        let to = &moves[1];
        let mut chars = to.chars();
        let x = chars.next()?;
        let y = chars.next()?;
        let xi = (x as usize) - 'a' as usize;
        let yi = (y as usize) - '1' as usize;
        let d = (yi << 3) | xi;
        let dsquare = self.get(d);

        Some(Turn {
            from: a,
            attacker: asquare,
            to: d,
            defender: dsquare,
        })
    }

    pub fn flush(&mut self) {
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
                position: i,
            });
        }
        for i in 48..56 {
            self.0[i] = Some(Piece {
                piece: Type::Pawn,
                color: Color::White,
                position: i,
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
