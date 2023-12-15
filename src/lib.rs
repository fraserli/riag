pub mod board;
pub mod error;

pub use board::piece::{Colour, Piece, UncolouredPiece};
pub use board::{Bitboard, Board, CastlingRights, Square};
