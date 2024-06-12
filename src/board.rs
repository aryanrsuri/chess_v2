use crate::piece::Color;
use crate::piece::Piece;
use crate::piece::Type;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Turn {
    from: usize,
    attacker: Option<Piece>,
    to: usize,
    defender: Option<Piece>,
}

/* This is a way to implement a history of captures:
* (true, none)
* (true, some)
* (false, none)
* >> pub struct Res(bool, Option<Piece>);
* */

#[derive(Debug)]
pub struct Board(pub Vec<Option<Piece>>);
impl Board {
    pub fn new() -> Self {
        let mut b = Board(vec![None; 64]);
        b.flush();
        b
    }
    pub fn set(&mut self, input: &str, side: Color) -> bool {
        let moves: Vec<&str> = input.split(" ").collect();
        let turn: Turn = self.parse(moves).unwrap();
        if self.valid(&turn, side) {
            let mut attacker = self.0[turn.from].take().unwrap();
            attacker.position = turn.to;
            attacker.moved = true;
            self.0[turn.to] = Some(attacker);
            return true;
        }
        false
    }

    fn get(&mut self, index: usize) -> Option<Piece> {
        self.0[index]
    }

    fn valid(&self, turn: &Turn, side: Color) -> bool {
        /* Non-Collision Validation:
         * Defender is none
         * Attacker is a piece
         * Attacker piece is the same color of side (could be abstracted)
         * */
        if turn.defender.is_none()
            && turn.attacker.is_some()
            && turn.attacker.unwrap().color == side
        {
            let attacker = turn.attacker.unwrap();
            return match attacker {
                Piece {
                    position: _,
                    piece: Type::Pawn,
                    color: _,
                    moved: _,
                } => {
                    let max = if attacker.moved { 1 } else { 2 };
                    let aln = turn.from % 8 == turn.to % 8;
                    let del = ((turn.to >> 3) as isize - (turn.from >> 3) as isize).abs();
                    if aln && del <= max {
                        return true;
                    }
                    return false;
                }
                _ => true,
            };
        }
        /* Collision Validation:
         * Defender is a piece
         * Attacker is a piece
         * Attacker piece is the same color of side (could be abstracted)
         * */
        if turn.defender.is_some()
            && turn.attacker.is_some()
            && turn.attacker.unwrap().color == side
            && turn.defender.unwrap().color != turn.attacker.unwrap().color
        {
            println!("Collision Possibility");
            let attacker = turn.attacker.unwrap();
            return match attacker {
                Piece {
                    position: _,
                    piece: Type::Pawn,
                    color: _,
                    moved: _,
                } => {
                    let aln = ((turn.from % 8) as isize - (turn.to % 8) as isize).abs() == 1;
                    let del = ((turn.to >> 3) as isize - (turn.from >> 3) as isize).abs();
                    // No condition for En pessant
                    if aln && del == 1 {
                        return true;
                    }
                    return false;
                }
                _ => false,
            };
        }
        false
    }

    fn parse(&mut self, moves: Vec<&str>) -> Option<Turn> {
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

    fn flush(&mut self) {
        self.0[0] = Some(Piece {
            piece: Type::Rook,
            color: Color::White,
            position: 0,
            moved: false,
        });
        self.0[1] = Some(Piece {
            piece: Type::Knight,
            color: Color::White,
            position: 1,
            moved: false,
        });
        self.0[2] = Some(Piece {
            piece: Type::Bishop,
            color: Color::White,
            position: 2,
            moved: false,
        });
        self.0[3] = Some(Piece {
            piece: Type::King,
            color: Color::White,
            position: 3,
            moved: false,
        });
        self.0[4] = Some(Piece {
            piece: Type::Queen,
            color: Color::White,
            position: 4,
            moved: false,
        });
        self.0[5] = Some(Piece {
            piece: Type::Bishop,
            color: Color::White,
            position: 5,
            moved: false,
        });
        self.0[6] = Some(Piece {
            piece: Type::Knight,
            color: Color::White,
            position: 6,
            moved: false,
        });
        self.0[7] = Some(Piece {
            piece: Type::Rook,
            color: Color::White,
            position: 7,
            moved: false,
        });

        for i in 8..16 {
            self.0[i] = Some(Piece {
                piece: Type::Pawn,
                color: Color::White,
                position: i,
                moved: false,
            });
        }

        for i in 48..56 {
            self.0[i] = Some(Piece {
                piece: Type::Pawn,
                color: Color::Black,
                position: i,
                moved: false,
            });
        }

        self.0[56] = Some(Piece {
            piece: Type::Rook,
            color: Color::Black,
            position: 56,
            moved: false,
        });
        self.0[57] = Some(Piece {
            piece: Type::Knight,
            color: Color::Black,
            position: 57,
            moved: false,
        });
        self.0[58] = Some(Piece {
            piece: Type::Bishop,
            color: Color::Black,
            position: 58,
            moved: false,
        });
        self.0[59] = Some(Piece {
            piece: Type::King,
            color: Color::Black,
            position: 59,
            moved: false,
        });
        self.0[60] = Some(Piece {
            piece: Type::Queen,
            color: Color::Black,
            position: 60,
            moved: false,
        });
        self.0[61] = Some(Piece {
            piece: Type::Bishop,
            color: Color::Black,
            position: 61,
            moved: false,
        });
        self.0[62] = Some(Piece {
            piece: Type::Knight,
            color: Color::Black,
            position: 62,
            moved: false,
        });
        self.0[63] = Some(Piece {
            piece: Type::Rook,
            color: Color::Black,
            position: 63,
            moved: false,
        });
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..64 {
            let r = 7 - (x >> 3);
            let c = x % 8;
            let i = (r << 3) | c;
            if c == 0 {
                write!(f, "\n   {}  ", r + 1)?;
            }
            if let Some(piece) = self.0[i] {
                write!(f, "{} ", piece)?;
            } else {
                write!(f, ". ")?;
            }
        }
        write!(f, "\n\n      a b c d e f g h")?;
        Ok(())
    }
}
