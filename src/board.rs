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

/*
 * Chess board, indexed.
 *
 * 56 57 58 59 60 61 62 63
 * 48 49 50 51 52 53 54 55
 * 40 41 42 43 44 45 46 47
 * 32 33 34 35 36 37 38 39
 * 24 25 26 27 28 29 30 31
 * 16 17 18 19 20 21 22 23
 * 8  9  10 11 12 13 14 15
 * 0  1  2  3  4  5  7  8
 *
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

    fn collides(&self, turn: &Turn) -> bool {
        /* Determines whether the piece collides with another piece of the same color
         * >> a1 a3 (pawn on a2)
         * >> true  (rook will collide with a pawn)
         * Find all indexes between the column movement
         * Fina all indexes between diagonal movement
         * and all index between the row movement. If there are any pieces of the same
         * attacker colour (ignoring Knight) then collides = true
         * */
        if turn.attacker.unwrap().piece == Type::Knight {
            return false;
        }
        // There probably is a more terse and effecient way to do this
        let from_col = turn.from % 8;
        let from_row = turn.from >> 3;
        let to_col = turn.to % 8;
        let to_row = turn.to >> 3;
        let y_aln: bool = from_col == to_col;
        let x_aln: bool = from_row == to_row;
        if y_aln {
            let indeces = ((turn.to as isize - turn.from as isize).abs() >> 3) as usize;
            let op: isize = if turn.attacker.unwrap().color == Color::White {
                1
            } else {
                -1
            };
            for x in 1..indeces {
                let pass = (turn.from as isize + op * (8 * x as isize)) as usize;
                if self.0[pass].is_some_and(|p| p.color == turn.attacker.unwrap().color) {
                    return true;
                }
            }
        }
        // !TODO: Implement X alignment collision
        if x_aln {
            let indeces = ((turn.to as isize - turn.from as isize).abs() >> 3) as usize;
            let op: isize = if turn.attacker.unwrap().color == Color::White {
                1
            } else {
                -1
            };
            for x in 1..indeces {
                println!("{}", turn.from as isize + op * (8 * x as isize));
            }
        }
        false
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
            && !self.collides(&turn)
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
            && !self.collides(&turn)
        {
            // println!("Collision Possibility");
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
