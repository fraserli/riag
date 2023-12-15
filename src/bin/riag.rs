use std::io::Write;

use riag::Board;

fn main() -> Result<(), std::io::Error> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;

        match Board::from_fen(line.trim()) {
            Ok(board) => print!("# {}\n{}", board.fen(), board),
            Err(e) => eprintln!("{}", e),
        }
    }
}
