use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse FEN string: {0}")]
    FailedToParseFEN(String),
    #[error("'{0}' is not a valid piece")]
    InvalidPiece(char),
    #[error("'{0}' is not a valid file ('a'..='h')")]
    InvalidFile(char),
    #[error("'{0}' is not a valid file ('1'..='8')")]
    InvalidRank(char),
}
