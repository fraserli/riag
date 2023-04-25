use crate::board::Board;
use crate::piece::{constants::*, Piece, PieceExt};

pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn decode(fen: &str) -> Board {
    let mut bitboards = [[0; 6]; 2];
    let mut board = [EMPTY as u8; 64];

    let mut it = fen.split(' ');
    let placement = it.next().unwrap();
    let to_move = match it.next().unwrap() {
        "w" => WHITE,
        "b" => BLACK,
        _ => panic!(),
    };
    let castle_chars = it.next().unwrap();
    let en_passant_coords = it.next().unwrap();
    let halfmove: usize = it.next().unwrap().parse().unwrap();
    let _fullmove: usize = it.next().unwrap().parse().unwrap();

    for (y, row) in placement.split('/').rev().enumerate() {
        let mut x = 0;
        for c in row.chars() {
            if c.is_numeric() {
                x += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_alpha(c);
                let pos = x + y * 8;

                bitboards[piece >> 4][piece & PIECE_MASK] |= 1 << pos;
                board[pos] = piece as u8;

                x += 1;
            }
        }
        assert_eq!(x, 8);
    }

    let mut castle = 0b0000;

    for c in castle_chars.chars() {
        match c {
            'K' => castle |= 0b1000,
            'Q' => castle |= 0b0100,
            'k' => castle |= 0b0010,
            'q' => castle |= 0b0001,
            '-' => {}
            _ => panic!(),
        }
    }

    let en_passant = if en_passant_coords != "-" {
        let mut chars = en_passant_coords.chars();
        let x = chars.next().unwrap();
        let y = chars.next().unwrap();
        x as u8 - b'a' + (y.to_digit(10).unwrap() as u8 - 1) * 8
    } else {
        u8::MAX
    };

    Board {
        bitboards,
        board,
        to_move,
        castle,
        en_passant,
        halfmove,
    }
}
