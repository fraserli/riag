use std::fmt::Display;

use crate::bitboard::Bitboard;
use crate::fen;
use crate::piece::{Piece, PieceExt};

#[derive(Clone, Copy)]
pub struct Board {
    pub bitboards: [[Bitboard; 6]; 2],
    pub board: [u8; 64],
    pub to_move: Piece,
    pub castle: u8, // KQkq bits
    pub en_passant: u8,
    pub halfmove: usize,
}

impl Board {
    pub fn start() -> Self {
        fen::decode(fen::START)
    }

    pub fn from_fen(fen: &str) -> Self {
        fen::decode(fen)
    }

    pub fn perft(&self, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        for m in self.generate_moves() {
            nodes += self.apply_move(m).perft(depth - 1);
        }
        nodes
    }

    pub fn perft_divide(&self, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        for m in self.generate_moves() {
            let p = self.apply_move(m).perft(depth - 1);
            println!("{}: {}", std::str::from_utf8(&m.name()).unwrap(), p,);
            nodes += p;
        }
        nodes
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        writeln!(f, "   +-----------------+")?;
        for y in (0..8).rev() {
            write!(f, " {} | ", y + 1)?;
            for x in 0..8 {
                f.write_char((self.board[x + y * 8] as Piece).to_unicode())?;
                f.write_char(' ')?;
            }
            writeln!(f, "| {}", y + 1)?;
        }
        writeln!(f, "   +-----------------+\n     a b c d e f g h")?;
        std::fmt::Result::Ok(())
    }
}
