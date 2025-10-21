use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadingError {
    #[error("EOF, Tried to read {0} but No bytes left to consume")]
    CleanEOF(String),
    #[error("incomplete: {0}")]
    Incomplete(String),
    #[error("too large: {0}")]
    TooLarge(String),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Error)]
pub enum WritingError {
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("Serde failure: {0}")]
    Serde(String),
    #[error("Failed to serialize packet: {0}")]
    Message(String),
}
