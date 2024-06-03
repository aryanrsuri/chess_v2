use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Piece {
    pub piece: Type,
    pub color: Color,
    pub position: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    Black,
    White,
}

impl Type {
    pub fn to_string(&self, color: Color) -> String {
        let ty = format!("{}", self);
        if color == Color::White {
            ty.to_uppercase()
        } else {
            ty
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.piece.to_string(self.color);
        write!(f, "{}", s)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Type::Pawn => "p",
                Type::Knight => "n",
                Type::Bishop => "b",
                Type::Rook => "r",
                Type::Queen => "q",
                Type::King => "k",
            }
        )
    }
}
