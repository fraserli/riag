use std::ops::{BitOr, Not};

use strum::{EnumCount, EnumIter};

use self::Colour as C;
use self::Piece as P;
use self::UncolouredPiece as U;
use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, EnumCount)]
pub enum Piece {
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncolouredPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn colour(self) -> Colour {
        match self {
            P::BlackKing
            | P::BlackQueen
            | P::BlackRook
            | P::BlackBishop
            | P::BlackKnight
            | P::BlackPawn => Colour::Black,
            P::WhiteKing
            | P::WhiteQueen
            | P::WhiteRook
            | P::WhiteBishop
            | P::WhiteKnight
            | P::WhitePawn => Colour::White,
        }
    }

    pub fn piece(self) -> UncolouredPiece {
        match self {
            P::BlackKing | P::WhiteKing => U::King,
            P::BlackQueen | P::WhiteQueen => U::Queen,
            P::BlackRook | P::WhiteRook => U::Rook,
            P::BlackBishop | P::WhiteBishop => U::Bishop,
            P::BlackKnight | P::WhiteKnight => U::Knight,
            P::BlackPawn | P::WhitePawn => U::Pawn,
        }
    }

    pub fn as_index(self) -> usize {
        self as usize
    }
}

impl TryFrom<char> for Piece {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'k' => Ok(P::BlackKing),
            'q' => Ok(P::BlackQueen),
            'r' => Ok(P::BlackRook),
            'b' => Ok(P::BlackBishop),
            'n' => Ok(P::BlackKnight),
            'p' => Ok(P::BlackPawn),
            'K' => Ok(P::WhiteKing),
            'Q' => Ok(P::WhiteQueen),
            'R' => Ok(P::WhiteRook),
            'B' => Ok(P::WhiteBishop),
            'N' => Ok(P::WhiteKnight),
            'P' => Ok(P::WhitePawn),
            _ => Err(Error::InvalidPiece(value)),
        }
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            P::BlackKing => 'k',
            P::BlackQueen => 'q',
            P::BlackRook => 'r',
            P::BlackBishop => 'b',
            P::BlackKnight => 'n',
            P::BlackPawn => 'p',
            P::WhiteKing => 'K',
            P::WhiteQueen => 'Q',
            P::WhiteRook => 'R',
            P::WhiteBishop => 'B',
            P::WhiteKnight => 'N',
            P::WhitePawn => 'P',
        }
    }
}

impl BitOr<UncolouredPiece> for Colour {
    type Output = Piece;

    fn bitor(self, rhs: UncolouredPiece) -> Self::Output {
        match (self, rhs) {
            (C::Black, U::King) => P::BlackKing,
            (C::Black, U::Queen) => P::BlackQueen,
            (C::Black, U::Rook) => P::BlackRook,
            (C::Black, U::Bishop) => P::BlackBishop,
            (C::Black, U::Knight) => P::BlackKnight,
            (C::Black, U::Pawn) => P::BlackPawn,
            (C::White, U::King) => P::WhiteKing,
            (C::White, U::Queen) => P::WhiteQueen,
            (C::White, U::Rook) => P::WhiteRook,
            (C::White, U::Bishop) => P::WhiteBishop,
            (C::White, U::Knight) => P::WhiteKnight,
            (C::White, U::Pawn) => P::WhitePawn,
        }
    }
}

impl Not for Colour {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
