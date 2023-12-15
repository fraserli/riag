use std::ops::BitOr;

use chumsky::prelude::*;
use strum::EnumCount;

use crate::error::Error;
use crate::{Bitboard, Board, CastlingRights, Colour, Piece, Square};

impl Board {
    /// Creates the board from a [FEN string](https://www.chessprogramming.org/Forsyth-Edwards_Notation).
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        match parser().parse(fen).into_result() {
            Ok(b) => Ok(b),
            Err(_) => Err(Error::FailedToParseFEN(fen.to_owned())),
        }
    }

    /// Exports the board to FEN.
    pub fn fen(&self) -> String {
        let mut output = String::new();

        for (i, rank) in self.board.chunks(8).rev().enumerate() {
            let mut it = rank.iter().peekable();
            while let Some(&piece) = it.next() {
                if let Some(p) = piece {
                    output.push(char::from(p));
                } else {
                    let mut n = 1;
                    while it.next_if(|&&p| p.is_none()).is_some() {
                        n += 1
                    }
                    output.push_str(&format!("{}", n));
                }
            }

            if i != 7 {
                output.push('/');
            }
        }

        output.push_str(&format!(
            " {} {} {} {} {}",
            match self.to_move {
                Colour::White => 'w',
                Colour::Black => 'b',
            },
            self.castling_rights,
            match self.en_passant {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            },
            self.halfmove,
            self.fullmove,
        ));

        output
    }
}

// TODO: improve FEN parsing error messages
pub fn parser<'a>() -> impl Parser<'a, &'a str, Board> {
    let emptysquares = text::digits(10)
        .exactly(1)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(|n: usize| vec![None; n]);
    let piece = one_of("KQRBNPkqrbnp").map(|c: char| vec![Some(Piece::try_from(c).unwrap())]);

    let rank = choice((emptysquares, piece))
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .try_map(|its, _| {
            TryInto::<[Option<Piece>; 8]>::try_into(its.into_iter().flatten().collect::<Vec<_>>())
                .map_err(|_| EmptyErr::default())
        });

    let board = rank
        .separated_by(just('/'))
        .exactly(8)
        .collect_exactly::<[_; 8]>()
        .map(
            |mut ranks: [[Option<Piece>; 8]; 8]| -> [Option<Piece>; 64] {
                ranks.reverse(); // to conform with LERF mapping
                unsafe { std::mem::transmute(ranks) }
            },
        );

    let colour = choice((just('b').to(Colour::Black), just('w').to(Colour::White)));

    let castling_right = one_of("KQkq").map(|c| match c {
        'K' => CastlingRights::WHITE_KINGSIDE,
        'Q' => CastlingRights::WHITE_QUEENSIDE,
        'k' => CastlingRights::BLACK_KINGSIDE,
        'q' => CastlingRights::BLACK_QUEENSIDE,
        _ => unreachable!(),
    });

    let castling_rights = choice((
        just('-').to(CastlingRights::empty()),
        castling_right.foldl(castling_right.repeated(), BitOr::bitor),
    ));

    let en_passant = choice((
        just('-').to(None),
        one_of("abcdefgh")
            .then(one_of("12345678"))
            .map(Square::try_from)
            .unwrapped()
            .map(Some),
    ));

    let number = text::int(10).from_str().unwrapped();

    group((
        board,
        text::whitespace(),
        colour,
        text::whitespace(),
        castling_rights,
        text::whitespace(),
        en_passant,
        text::whitespace(),
        number.or_not(),
        text::whitespace(),
        number.or_not(),
        text::whitespace(),
        end(),
    ))
    .map(
        |(board, _, to_move, _, castling_rights, _, en_passant, _, halfmove, _, fullmove, _, _)| {
            Board {
                board,
                bitboards: bitboards(board),
                to_move,
                castling_rights,
                en_passant,
                halfmove: halfmove.unwrap_or(0),
                fullmove: fullmove.unwrap_or(1),
            }
        },
    )
}

fn bitboards(board: [Option<Piece>; 64]) -> [Bitboard; Piece::COUNT] {
    let mut bitboards = [0; Piece::COUNT];

    board
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| p.map(|p| (i, p)))
        .for_each(|(i, p)| bitboards[p.as_index()] |= 1 << i);

    bitboards
}
