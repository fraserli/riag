pub type Piece = usize; // value representable with u8
use constants::*;

pub mod constants {
    use super::Piece;

    pub const KING: Piece = 0b0000;
    pub const QUEEN: Piece = 0b0001;
    pub const ROOK: Piece = 0b0010;
    pub const BISHOP: Piece = 0b0011;
    pub const KNIGHT: Piece = 0b0100;
    pub const PAWN: Piece = 0b0101;
    pub const EMPTY: Piece = 0b0110;

    pub const WHITE: Piece = 0b0000;
    pub const BLACK: Piece = 0b0001;

    pub const WHITE_KING: Piece = KING | (WHITE << 4);
    pub const WHITE_QUEEN: Piece = QUEEN | (WHITE << 4);
    pub const WHITE_ROOK: Piece = ROOK | (WHITE << 4);
    pub const WHITE_BISHOP: Piece = BISHOP | (WHITE << 4);
    pub const WHITE_KNIGHT: Piece = KNIGHT | (WHITE << 4);
    pub const WHITE_PAWN: Piece = PAWN | (WHITE << 4);

    pub const BLACK_KING: Piece = KING | (BLACK << 4);
    pub const BLACK_QUEEN: Piece = QUEEN | (BLACK << 4);
    pub const BLACK_ROOK: Piece = ROOK | (BLACK << 4);
    pub const BLACK_BISHOP: Piece = BISHOP | (BLACK << 4);
    pub const BLACK_KNIGHT: Piece = KNIGHT | (BLACK << 4);
    pub const BLACK_PAWN: Piece = PAWN | (BLACK << 4);

    pub const PIECE_MASK: Piece = 0b00001111;
}

pub trait PieceExt {
    fn from_alpha(c: char) -> Self;
    fn to_alpha(self) -> char;
    fn to_unicode(self) -> char;
}

impl PieceExt for Piece {
    fn from_alpha(c: char) -> Self {
        match c {
            'K' => WHITE_KING,
            'Q' => WHITE_QUEEN,
            'R' => WHITE_ROOK,
            'B' => WHITE_BISHOP,
            'N' => WHITE_KNIGHT,
            'P' => WHITE_PAWN,
            'k' => BLACK_KING,
            'q' => BLACK_QUEEN,
            'r' => BLACK_ROOK,
            'b' => BLACK_BISHOP,
            'n' => BLACK_KNIGHT,
            'p' => BLACK_PAWN,
            _ => panic!("invalid char for piece: {}", c),
        }
    }

    fn to_alpha(self) -> char {
        match self {
            WHITE_KING => 'K',
            WHITE_QUEEN => 'Q',
            WHITE_ROOK => 'R',
            WHITE_BISHOP => 'B',
            WHITE_KNIGHT => 'N',
            WHITE_PAWN => 'P',
            BLACK_KING => 'k',
            BLACK_QUEEN => 'q',
            BLACK_ROOK => 'r',
            BLACK_BISHOP => 'b',
            BLACK_KNIGHT => 'n',
            BLACK_PAWN => 'p',
            EMPTY => '.',
            _ => unreachable!(),
        }
    }

    fn to_unicode(self) -> char {
        match self {
            WHITE_KING => '♔',
            WHITE_QUEEN => '♕',
            WHITE_ROOK => '♖',
            WHITE_BISHOP => '♗',
            WHITE_KNIGHT => '♘',
            WHITE_PAWN => '♙',
            BLACK_KING => '♚',
            BLACK_QUEEN => '♛',
            BLACK_ROOK => '♜',
            BLACK_BISHOP => '♝',
            BLACK_KNIGHT => '♞',
            BLACK_PAWN => '♟',
            EMPTY => '•',
            _ => unreachable!(),
        }
    }
}
