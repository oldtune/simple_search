use std::error::Error;
use std::fmt::{write, Display};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    UnExpectedError,
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            AppError::IOError(err) => Some(err),
            AppError::UnExpectedError => None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AppError::IOError(err) => write!(f, "Got some io error {}", err),
            AppError::UnExpectedError => {
                write!(f, "Unexpected my man")
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
         Self::IOError(err)
    }
}
