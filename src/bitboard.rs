pub type Bitboard = u64;

pub trait BitboardExt {
    fn bit_indices(self) -> BitIndices;
    fn print_bitboard(self);
}

impl BitboardExt for Bitboard {
    fn bit_indices(self) -> BitIndices {
        BitIndices { remainder: self }
    }

    fn print_bitboard(self) {
        for y in (0..8).rev() {
            for x in 0..8 {
                print!("{} ", (self & (1 << (x + y * 8))) >> (x + y * 8));
            }
            println!();
        }
    }
}

pub struct BitIndices {
    remainder: Bitboard,
}

impl Iterator for BitIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder == 0 {
            None
        } else {
            let index = self.remainder.trailing_zeros() as usize;
            self.remainder &= !(1 << index);
            Some(index)
        }
    }
}
