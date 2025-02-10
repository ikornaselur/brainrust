use nom::error::Error as NomError;
use nom::Err as NomErr;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BRError>;

#[derive(Error, Debug)]
pub enum BRError {
    #[error("Error: {0}")]
    Error(String),

    #[error("Unable to parse input file")]
    ParseError(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    NomError(String),
}

impl<I: std::fmt::Debug> From<NomErr<NomError<I>>> for BRError {
    fn from(err: NomErr<NomError<I>>) -> Self {
        match err {
            NomErr::Incomplete(_) => BRError::NomError("Incomplete input".to_string()),
            NomErr::Error(e) => BRError::NomError(format!("{:?}", e)),
            NomErr::Failure(e) => BRError::NomError(format!("{:?}", e)),
        }
    }
}
