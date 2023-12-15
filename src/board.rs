mod fen;
pub mod piece;

use std::fmt::Display;
use std::ops::Index;

use bitflags::bitflags;
use owo_colors::OwoColorize;
use strum::{EnumCount, IntoEnumIterator};

use crate::error::Error;
use crate::{Colour, Piece, UncolouredPiece};

/// A representation of a chess board state
#[derive(Clone, Copy, Debug)]
pub struct Board {
    board: [Option<Piece>; 64], // positions encoded with little-endian rank-file mapping
    bitboards: [Bitboard; Piece::COUNT], // indexed with Piece
    to_move: Colour,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove: usize,
    fullmove: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Square(usize);

pub type Bitboard = u64;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE  = 0b1000;
        const WHITE_QUEENSIDE = 0b0100;
        const BLACK_KINGSIDE  = 0b0010;
        const BLACK_QUEENSIDE = 0b0001;
    }
}

impl Board {
    pub fn start() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn print_bitboards(&self) {
        for p in Piece::iter() {
            println!("{:?}: {}", p, self.bitboards[p.as_index()]);
            for rank in (0..8).rev() {
                for file in 0..8 {
                    print!(
                        "{}",
                        if self.bitboards[p.as_index()] & (1 << (file + rank * 8)) != 0 {
                            '1'
                        } else {
                            '0'
                        }
                    );
                }
                println!();
            }
            println!();
        }
    }
}

impl Index<Square> for Board {
    type Output = Option<Piece>;

    fn index(&self, square: Square) -> &Self::Output {
        &self.board[square.0]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print_piece = |piece: Option<Piece>| {
            if let Some(p) = piece {
                use UncolouredPiece as U;
                let s = match p.piece() {
                    U::King => "♚",
                    U::Queen => "♛",
                    U::Rook => "♜",
                    U::Bishop => "♝",
                    U::Knight => "♞",
                    U::Pawn => "♟",
                };

                if p.colour() == Colour::White {
                    s.fg_rgb::<255, 255, 255>().to_string()
                } else {
                    s.fg_rgb::<0, 0, 0>().to_string()
                }
            } else {
                " ".to_owned()
            }
        };

        f.write_str("╔══════════════════════╗\n║   A B C D E F G H    ║\n")?;
        for rank in (0..8).rev() {
            write!(f, "║ {} ", rank + 1)?;
            for file in 0..8 {
                let p = format!("{} ", print_piece(self.board[file + rank * 8]));
                if (rank + file + 1) % 2 == 0 {
                    write!(f, "{}", p.bg_rgb::<162, 115, 20>())?;
                } else {
                    write!(f, "{}", p.bg_rgb::<250, 200, 125>())?;
                }
            }
            writeln!(f, " {} ║", rank + 1)?;
        }
        f.write_str("║   A B C D E F G H    ║\n╚══════════════════════╝\n")?;

        Ok(())
    }
}

impl TryFrom<(char, char)> for Square {
    type Error = Error;

    fn try_from((f, r): (char, char)) -> Result<Self, Self::Error> {
        if !('a'..='h').contains(&f) {
            return Err(Error::InvalidFile(f));
        }
        if !('1'..='8').contains(&r) {
            return Err(Error::InvalidRank(r));
        }

        let file = f as usize - 'a' as usize;
        let rank = r as usize - '1' as usize;

        Ok(Self(file + rank * 8))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.0 % 8];
        let rank = ['1', '2', '3', '4', '5', '6', '7', '8'][self.0 / 8];

        write!(f, "{}{}", file, rank)
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return f.write_str("-");
        }

        if self.contains(CastlingRights::WHITE_KINGSIDE) {
            f.write_str("K")?;
        }
        if self.contains(CastlingRights::WHITE_QUEENSIDE) {
            f.write_str("Q")?;
        }
        if self.contains(CastlingRights::BLACK_KINGSIDE) {
            f.write_str("k")?;
        }
        if self.contains(CastlingRights::BLACK_QUEENSIDE) {
            f.write_str("q")?;
        }

        Ok(())
    }
}
