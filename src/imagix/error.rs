use std::{io, fmt};
use image::error;

#[derive(Debug)]
pub enum ImagixError {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String)
}

impl From<io::Error> for ImagixError {
    fn from(error: io::Error) -> Self {
        ImagixError::FileIOError(format!("{}", error))
    }
}

impl From<error::ImageError> for ImagixError {
    fn from(error: error::ImageError) -> Self {
        ImagixError::ImageResizingError(format!("{}", error))
    }
}

impl From<io::ErrorKind> for ImagixError {
    fn from(error: io::ErrorKind) -> Self {
        ImagixError::UserInputError(format!("{}", error))
    }
}

impl fmt::Display for ImagixError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            out,
            "{}",
            ImagixError::FormatError("Error occured".to_string())
        )
    }
}