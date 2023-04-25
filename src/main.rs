use std::time::Instant;

use riag::board::Board;
use riag::magic;

fn main() {
    let mut t = Instant::now();
    magic::force_generate();
    println!(
        "Magic numbers generated in {:.3} s\n",
        (Instant::now() - t).as_secs_f32()
    );

    let b = Board::start();

    println!("perft(6):");

    t = Instant::now();
    let p = b.perft_divide(6);
    let s = (Instant::now() - t).as_secs_f32();

    println!("\nNodes searched: {}", p);
    println!("Time: {:.3} s", s);
    println!("Speed: {:.2} MN/s", (p as f32 * 0.000001) / s);
}
