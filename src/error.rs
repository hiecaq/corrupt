use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidStyleBundle(#[from] ron::de::Error),
    #[error(transparent)]
    FailToPrint(#[from] crossterm::ErrorKind),
    #[error("failed to flush stdout")]
    FailToFlush(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
