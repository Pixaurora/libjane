use std::io::Error as IOError;
use std::{error::Error, fmt::Display};

use evalexpr::EvalexprError;
#[cfg(feature = "binary")]
use hex::FromHexError;
use image::ImageError;

#[derive(Debug)]
pub enum GraphingError {
    ExpressionError(EvalexprError),
    IOError(IOError),
    ParsingError { message: String },
}

impl GraphingError {
    pub fn parsing(message: String) -> Self {
        GraphingError::ParsingError { message }
    }
}

impl From<EvalexprError> for GraphingError {
    fn from(value: EvalexprError) -> Self {
        GraphingError::ExpressionError(value)
    }
}

#[cfg(feature = "binary")]
impl From<FromHexError> for GraphingError {
    fn from(value: FromHexError) -> Self {
        GraphingError::parsing(format!("Failed to convert your hex code: `{}`", value))
    }
}

impl From<IOError> for GraphingError {
    fn from(value: IOError) -> Self {
        GraphingError::IOError(value)
    }
}

impl From<ImageError> for GraphingError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::IoError(value) => GraphingError::IOError(value),
            _ => GraphingError::parsing(format!("Image error: `{}`", value)),
        }
    }
}

impl Display for GraphingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpressionError(err) => err.fmt(f),
            Self::IOError(err) => err.fmt(f),
            Self::ParsingError { message } => write!(f, "{message}"),
        }
    }
}

impl Error for GraphingError {}

pub type GraphingResult<T> = Result<T, GraphingError>;
