use std::{fmt, io, error, result};
use std::error::Error as StdError;
use std::io::Error as IoError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    ColorLength,
    ParseInt(ParseIntError),
    InvalidCommand,
    WrongNumberOfArguments,
    LineTooLong,
}

pub type Result<T> = result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::ColorLength => "Invalid color length",
            Error::ParseInt(ref err) => err.description(),
            Error::InvalidCommand => "Invalid command",
            Error::WrongNumberOfArguments => "Wrong number of arguments",
            Error::LineTooLong => "Line is longer than longest possible command",
        }
    }
}

impl PartialEq for Error {
    // Specific IO Errors are ignored
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::Io(_), &Error::Io(_)) => true,
            (&Error::ColorLength, &Error::ColorLength) => true,
            (&Error::ParseInt(ref a), &Error::ParseInt(ref b)) => a == b,
            (&Error::InvalidCommand, &Error::InvalidCommand) => true,
            (&Error::WrongNumberOfArguments, &Error::WrongNumberOfArguments) => true,
            (&Error::LineTooLong, &Error::LineTooLong) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "Io Error: {}", err),
            Error::ColorLength => write!(f, "PixelflutError: wrong length for color field"),
            Error::ParseInt(ref err) => write!(f, "ParseIntError: {}", err),
            Error::InvalidCommand => write!(f, "PixelflutError: Invalid Command"),
            Error::WrongNumberOfArguments => write!(f, "PixelflutError: Wrong number of arguments"),
            Error::LineTooLong => write!(f, "PixelflutError: A client send a too long line"),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl Into<IoError> for Error {
    fn into(self) -> IoError {
        IoError::new(io::ErrorKind::InvalidData, self.description())
    }
}
