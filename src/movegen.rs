use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::Board;
use crate::magic::{BISHOP_MAGIC, ROOK_MAGIC};
use crate::piece::{constants::*, Piece};

const FILE_A: Bitboard = 0x101010101010101;
const FILE_B: Bitboard = 0x202020202020202;
const FILE_G: Bitboard = 0x4040404040404040;
const FILE_H: Bitboard = 0x8080808080808080;
const RANK_1: Bitboard = 0xFF;
const RANK_2: Bitboard = 0xFF00;
const RANK_7: Bitboard = 0xFF000000000000;
const RANK_8: Bitboard = 0xFF00000000000000;

const KING_MOVES: [Bitboard; 64] = {
    let mut squares = [0; 64];

    let mut pos = 0;
    while pos < 64 {
        let bb = 1 << pos;
        squares[pos] = (bb & !FILE_A) >> 1
            | (bb & !FILE_H) << 1
            | (bb & !RANK_8) << 8
            | (bb & !RANK_1) >> 8
            | (bb & !(RANK_8 | FILE_H)) << 9
            | (bb & !(RANK_1 | FILE_H)) >> 7
            | (bb & !(RANK_1 | FILE_A)) >> 9
            | (bb & !(RANK_8 | FILE_A)) << 7;
        pos += 1;
    }

    squares
};

const KNIGHT_MOVES: [Bitboard; 64] = {
    let mut squares = [0; 64];
    let mut pos = 0;
    while pos < 64 {
        let bb = 1 << pos;
        squares[pos] = (bb & !(RANK_7 | RANK_8 | FILE_H)) << 17 // NNE
                | (bb & !(RANK_8 | FILE_G | FILE_H)) << 10 // NEE
                | (bb & !(RANK_1 | FILE_G | FILE_H)) >> 6 // SEE
                | (bb & !(RANK_1 | RANK_2 | FILE_H)) >> 15 // SSE
                | (bb & !(RANK_1 | RANK_2 | FILE_A)) >> 17 // SSW
                | (bb & !(RANK_1 | FILE_A | FILE_B)) >> 10 // SWW
                | (bb & !(RANK_8 | FILE_A | FILE_B)) << 6 // NWW
                | (bb & !(RANK_7 | RANK_8 | FILE_A)) << 15; // NNW
        pos += 1;
    }
    squares
};

const PAWN_CAPTURES: [[Bitboard; 64]; 2] = {
    let mut squares = [[0; 64]; 2];
    let mut pos = 8;
    while pos < 56 {
        let bb = 1 << pos;
        squares[WHITE][pos] |= (bb & !FILE_A) << 7 | (bb & !FILE_H) << 9;
        squares[BLACK][pos] |= (bb & !FILE_H) >> 7 | (bb & !FILE_A) >> 9;
        pos += 1;
    }
    squares
};

#[derive(Clone, Copy)]
pub struct Move(u16);

impl Move {
    pub fn new(pos: usize, dest: usize, promotion: Piece) -> Self {
        Move(pos as u16 | (dest as u16) << 6 | (promotion as u16) << 12)
    }

    fn pos(&self) -> usize {
        const MASK: u16 = 0b0000000000111111;
        (self.0 & MASK) as usize
    }

    fn dest(&self) -> usize {
        const MASK: u16 = 0b0000111111000000;
        (self.0 & MASK) as usize >> 6
    }

    fn promotion(&self) -> Piece {
        const MASK: u16 = 0b1111000000000000;
        (self.0 & MASK) as Piece >> 12
    }

    pub fn name(&self) -> [u8; 4] {
        let pos = self.pos();
        let dest = self.dest();

        [
            (b'a' + (pos % 8) as u8),
            (b'1' + (pos / 8) as u8),
            (b'a' + (dest % 8) as u8),
            (b'1' + (dest / 8) as u8),
        ]
    }
}

impl Board {
    pub fn generate_pseudolegal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let bitboards = &self.bitboards[self.to_move];
        let colour_bitboards = get_colour_bitboards(&self.bitboards);

        let player = colour_bitboards[self.to_move];
        let opponent = colour_bitboards[self.to_move ^ 1];
        let all = player | opponent;

        // King
        for pos in bitboards[KING].bit_indices() {
            let mut dest = KING_MOVES[pos] & !player;
            let attacked = self.attacked_squares(self.to_move ^ 1);

            if self.to_move == WHITE {
                const KMASK: Bitboard = 0x60;
                const QMASK: Bitboard = 0xE;
                const KCMASK: Bitboard = 0x70;
                const QCMASK: Bitboard = 0x1C;
                if self.castle & (1 << 3) != 0
                    && all & KMASK == 0
                    && KCMASK & attacked == 0
                    && self.board[7] as Piece == WHITE_ROOK
                {
                    dest |= 0x40;
                }
                if self.castle & (1 << 2) != 0
                    && all & QMASK == 0
                    && QCMASK & attacked == 0
                    && self.board[0] as Piece == WHITE_ROOK
                {
                    dest |= 0x4;
                }
            } else {
                const KMASK: Bitboard = 0x6000000000000000;
                const QMASK: Bitboard = 0xE00000000000000;
                const KCMASK: Bitboard = 0x7000000000000000;
                const QCMASK: Bitboard = 0x1C00000000000000;
                if self.castle & (1 << 1) != 0
                    && all & KMASK == 0
                    && KCMASK & attacked == 0
                    && self.board[63] as Piece == BLACK_ROOK
                {
                    dest |= 0x4000000000000000;
                }
                if self.castle & 1 != 0
                    && all & QMASK == 0
                    && QCMASK & attacked == 0
                    && self.board[56] as Piece == BLACK_ROOK
                {
                    dest |= 0x400000000000000;
                }
            }

            encode_moves(&mut moves, pos, dest);
        }

        // Queen
        for pos in bitboards[QUEEN].bit_indices() {
            let dest = (ROOK_MAGIC.attacks(pos, all) | BISHOP_MAGIC.attacks(pos, all)) & !player;
            encode_moves(&mut moves, pos, dest);
        }

        // Rook
        for pos in bitboards[ROOK].bit_indices() {
            let dest = ROOK_MAGIC.attacks(pos, all) & !player;
            encode_moves(&mut moves, pos, dest);
        }

        // Bishop
        for pos in bitboards[BISHOP].bit_indices() {
            let dest = BISHOP_MAGIC.attacks(pos, all) & !player;
            encode_moves(&mut moves, pos, dest);
        }

        // Knight
        for pos in bitboards[KNIGHT].bit_indices() {
            let dest = KNIGHT_MOVES[pos] & !player;
            encode_moves(&mut moves, pos, dest);
        }

        // Pawn
        for pos in bitboards[PAWN].bit_indices() {
            let ep = if self.en_passant != u8::MAX {
                1 << self.en_passant
            } else {
                0
            };

            let bb = 1 << pos;

            let dest = if self.to_move == WHITE {
                (PAWN_CAPTURES[WHITE][pos] & (opponent | ep))
                    | ((bb << 8) & !all)
                    | (((((bb & RANK_2) << 8) & !all) << 8) & !all)
            } else {
                (PAWN_CAPTURES[BLACK][pos] & (opponent | ep))
                    | ((bb >> 8) & !all)
                    | (((((bb & RANK_7) >> 8) & !all) >> 8) & !all)
            };

            encode_pawn_moves(&mut moves, pos, dest);
        }

        moves
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        self.generate_pseudolegal_moves()
            .into_iter()
            .filter(|&m| {
                let b = self.apply_move(m);
                b.bitboards[self.to_move][KING] & b.attacked_squares(self.to_move ^ 1) == 0
            })
            .collect()
    }

    pub fn apply_move(&self, m: Move) -> Self {
        let pos = m.pos();
        let dest = m.dest();
        let promotion = m.promotion();

        let piece = self.board[pos] as Piece & PIECE_MASK;
        let diff = dest as isize - pos as isize;

        let mut board = self.board;
        let mut bitboards = self.bitboards;

        move_piece(pos, dest, &mut board, &mut bitboards, promotion);

        // Capture pawn for en passant
        if piece == PAWN && dest == self.en_passant as usize {
            if self.to_move == WHITE {
                bitboards[BLACK][PAWN] &= !(1 << (dest - 8));
                board[dest - 8] = EMPTY as u8;
            } else {
                bitboards[WHITE][PAWN] &= !(1 << (dest + 8));
                board[dest + 8] = EMPTY as u8;
            }
        }

        // Move rook for castle
        if piece == KING && diff == 2 {
            // King side
            if self.to_move == WHITE {
                move_piece(7, 5, &mut board, &mut bitboards, EMPTY);
            } else {
                move_piece(63, 61, &mut board, &mut bitboards, EMPTY);
            }
        } else if piece == KING && diff == -2 {
            // Queen side
            if self.to_move == WHITE {
                move_piece(0, 3, &mut board, &mut bitboards, EMPTY);
            } else {
                move_piece(56, 59, &mut board, &mut bitboards, EMPTY);
            }
        }

        // Update castle rights
        let mut castle = self.castle;
        if self.to_move == WHITE {
            if piece == KING {
                castle &= !0b1100;
            } else if piece == ROOK && pos == 0 {
                castle &= !0b0100;
            } else if piece == ROOK && pos == 7 {
                castle &= !0b1000;
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if piece == KING {
                castle &= !0b0011;
            } else if piece == ROOK && pos == 56 {
                castle &= !0b0001;
            } else if piece == ROOK && pos == 63 {
                castle &= !0b0010;
            }
        }

        // Set en passant square
        let en_passant = if piece == PAWN && diff.abs() == 16 {
            if self.to_move == WHITE {
                pos as u8 + 8
            } else if self.to_move == BLACK {
                pos as u8 - 8
            } else {
                u8::MAX
            }
        } else {
            u8::MAX
        };

        let halfmove = if self.board[dest] as Piece != EMPTY || piece == PAWN {
            0
        } else {
            self.halfmove + 1
        };

        Self {
            bitboards,
            board,
            to_move: self.to_move ^ 1,
            castle,
            en_passant,
            halfmove,
        }
    }

    pub fn attacked_squares(&self, colour: Piece) -> Bitboard {
        let mut squares = 0;

        let bitboards = &self.bitboards[colour];
        let [w, b] = get_colour_bitboards(&self.bitboards);
        let blockers = w | b;

        for pos in bitboards[KING].bit_indices() {
            squares |= KING_MOVES[pos];
        }

        for pos in bitboards[QUEEN].bit_indices() {
            squares |= ROOK_MAGIC.attacks(pos, blockers) | BISHOP_MAGIC.attacks(pos, blockers);
        }

        for pos in bitboards[ROOK].bit_indices() {
            squares |= ROOK_MAGIC.attacks(pos, blockers);
        }

        for pos in bitboards[BISHOP].bit_indices() {
            squares |= BISHOP_MAGIC.attacks(pos, blockers);
        }

        for pos in bitboards[KNIGHT].bit_indices() {
            squares |= KNIGHT_MOVES[pos];
        }

        for pos in bitboards[PAWN].bit_indices() {
            squares |= PAWN_CAPTURES[colour][pos];
        }

        squares
    }
}

fn move_piece(
    pos: usize,
    dest: usize,
    board: &mut [u8; 64],
    bitboards: &mut [[Bitboard; 6]; 2],
    promotion: Piece,
) {
    let piece = board[pos] as Piece & PIECE_MASK;
    let colour = board[pos] as Piece >> 4;

    bitboards[colour][piece] &= !(1 << pos);
    board[pos] = EMPTY as u8;

    if promotion == EMPTY {
        bitboards[colour][piece] |= 1 << dest;
        board[dest] = (piece | (colour << 4)) as u8;
    } else {
        bitboards[colour][promotion] |= 1 << dest;
        board[dest] = (promotion | (colour << 4)) as u8;
    }

    for p in [QUEEN, KING, ROOK, BISHOP, KNIGHT, PAWN] {
        bitboards[colour ^ 1][p] &= !(1 << dest);
    }
}

fn encode_moves(moves: &mut Vec<Move>, pos: usize, dest: Bitboard) {
    for i in dest.bit_indices() {
        moves.push(Move::new(pos, i, EMPTY));
    }
}

fn encode_pawn_moves(moves: &mut Vec<Move>, pos: usize, dest: Bitboard) {
    for i in dest.bit_indices() {
        match i {
            0..=7 | 56..=63 => {
                for piece in [QUEEN, ROOK, BISHOP, KNIGHT] {
                    moves.push(Move::new(pos, i, piece));
                }
            }
            _ => moves.push(Move::new(pos, i, EMPTY)),
        }
    }
}

pub fn get_colour_bitboards(bitboards: &[[Bitboard; 6]; 2]) -> [Bitboard; 2] {
    let w = bitboards[WHITE];
    let b = bitboards[BLACK];
    [
        w[KING] | w[QUEEN] | w[ROOK] | w[BISHOP] | w[KNIGHT] | w[PAWN],
        b[KING] | b[QUEEN] | b[ROOK] | b[BISHOP] | b[KNIGHT] | b[PAWN],
    ]
}
