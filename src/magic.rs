use std::sync::LazyLock;

use crate::bitboard::Bitboard;

pub static ROOK_MAGIC: LazyLock<Magic> = LazyLock::new(generate_rook_magic);
pub static BISHOP_MAGIC: LazyLock<Magic> = LazyLock::new(generate_bishop_magic);

pub struct Magic {
    entries: [MagicEntry; 64],
    attack_table: Vec<Bitboard>,
}

#[derive(Default, Clone, Copy)]
struct MagicEntry {
    magic: u64,
    mask: Bitboard,
    shift: usize,
    offset: usize,
}

impl Magic {
    pub fn attacks(&self, pos: usize, occupancy: Bitboard) -> Bitboard {
        let e = &self.entries[pos];
        let index = ((occupancy & e.mask).wrapping_mul(e.magic) as usize >> e.shift) + e.offset;
        self.attack_table[index]
    }
}

pub fn force_generate() {
    LazyLock::force(&ROOK_MAGIC);
    LazyLock::force(&BISHOP_MAGIC);
}

fn generate_rook_magic() -> Magic {
    generate_magic_numbers(&BLOCKER_MASK_ROOK, generate_rook_attacks)
}

fn generate_bishop_magic() -> Magic {
    generate_magic_numbers(&BLOCKER_MASK_BISHOP, generate_bishop_attacks)
}

fn bitmask_permutations(mask: Bitboard) -> Vec<Bitboard> {
    let count = 1 << mask.count_ones();
    let mut permutations = Vec::with_capacity(count);

    for i in 0..count as u64 {
        let mut curr_bit = 0;
        let mut p = mask;
        for j in 0..Bitboard::BITS {
            if mask & (1 << j) != 0 {
                p ^= ((i & (1 << curr_bit)) >> curr_bit) << j;
                curr_bit += 1;
            }
        }
        permutations.push(p)
    }

    permutations
}

fn generate_magic_numbers(
    masks: &[Bitboard; 64],
    generate_attacks: fn(pos: usize, blockers: Bitboard) -> Bitboard,
) -> Magic {
    let mut entries = [MagicEntry::default(); 64];
    let mut attack_table = Vec::new();

    let rng = fastrand::Rng::new();

    for pos in 0..64 {
        let mask = masks[pos];
        let bits = mask.count_ones() as usize;
        let shift = 64 - bits;

        let permutations = 1 << bits;
        let blockers = bitmask_permutations(mask);
        let attack_boards: Vec<Bitboard> =
            blockers.iter().map(|&b| generate_attacks(pos, b)).collect();

        let mut used = vec![0; permutations];

        'outer: loop {
            let magic = rng.u64(..) & rng.u64(..) & rng.u64(..);

            if (mask.wrapping_mul(magic) & 0xFF00000000000000).count_ones() < 6 {
                continue;
            }

            for i in 0..permutations {
                let index = blockers[i].wrapping_mul(magic) as usize >> shift;

                if used[index] != 0 && used[index] != attack_boards[i] {
                    used.fill(0);
                    continue 'outer;
                } else {
                    used[index] = attack_boards[i];
                }
            }

            entries[pos] = MagicEntry {
                magic,
                mask,
                shift,
                offset: attack_table.len(),
            };
            attack_table.extend_from_slice(&used);
            break;
        }
    }

    Magic {
        entries,
        attack_table,
    }
}

const BLOCKER_MASK_ROOK: [Bitboard; 64] = {
    let mut occupancy = [0; 64];
    let mut pos = 0;
    while pos < 64 {
        let x = pos % 8;
        let y = pos / 8;
        let mut o = 0;
        let mut i = 1;
        while i < 7 {
            if i != x {
                o |= 1 << (i + y * 8);
            }
            if i != y {
                o |= 1 << (x + i * 8);
            }
            i += 1;
        }
        occupancy[pos] = o;
        pos += 1;
    }
    occupancy
};

const BLOCKER_MASK_BISHOP: [Bitboard; 64] = {
    let mut occupancy = [0; 64];
    let mut pos = 0;
    while pos < 64 {
        let x = pos as u64 % 8;
        let y = pos as u64 / 8;
        let mut o: u64 = 0;

        let mut a = x;
        let mut b = y;
        while a < 7 && b < 7 {
            o |= 1 << (a + b * 8);
            a += 1;
            b += 1;
        }
        let mut a = x;
        let mut b = y;
        while a < 7 && b > 0 {
            o |= 1 << (a + b * 8);
            a += 1;
            b -= 1;
        }
        let mut a = x;
        let mut b = y;
        while a > 0 && b > 0 {
            o |= 1 << (a + b * 8);
            a -= 1;
            b -= 1;
        }
        let mut a = x;
        let mut b = y;
        while a > 0 && b < 7 {
            o |= 1 << (a + b * 8);
            a -= 1;
            b += 1;
        }

        occupancy[pos] = o & !(1 << pos);
        pos += 1;
    }
    occupancy
};

fn generate_rook_attacks(pos: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = 0;

    let x = pos % 8;
    let y = pos / 8;

    for i in x + 1..8 {
        attacks |= 1 << (i + y * 8);
        if blockers & 1 << (i + y * 8) != 0 {
            break;
        }
    }
    for i in (0..x).rev() {
        attacks |= 1 << (i + y * 8);
        if blockers & 1 << (i + y * 8) != 0 {
            break;
        }
    }
    for i in y + 1..8 {
        attacks |= 1 << (x + i * 8);
        if blockers & 1 << (x + i * 8) != 0 {
            break;
        }
    }
    for i in (0..y).rev() {
        attacks |= 1 << (x + i * 8);
        if blockers & 1 << (x + i * 8) != 0 {
            break;
        }
    }

    attacks
}

fn generate_bishop_attacks(pos: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = 0;

    let x = pos % 8;
    let y = pos / 8;

    for (a, b) in (x + 1..8).zip(y + 1..8) {
        attacks |= 1 << (a + b * 8);
        if blockers & 1 << (a + b * 8) != 0 {
            break;
        }
    }
    for (a, b) in (x + 1..8).zip((0..y).rev()) {
        attacks |= 1 << (a + b * 8);
        if blockers & 1 << (a + b * 8) != 0 {
            break;
        }
    }
    for (a, b) in (0..x).rev().zip(y + 1..8) {
        attacks |= 1 << (a + b * 8);
        if blockers & 1 << (a + b * 8) != 0 {
            break;
        }
    }
    for (a, b) in (0..x).rev().zip((0..y).rev()) {
        attacks |= 1 << (a + b * 8);
        if blockers & 1 << (a + b * 8) != 0 {
            break;
        }
    }

    attacks
}
