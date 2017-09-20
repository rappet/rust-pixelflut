use std::{fmt, io, error, result};
use std::error::Error as StdError;
use std::str::Utf8Error;
use std::io::Error as IoError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    ColorLength,
    ParseInt(ParseIntError),
    InvalidCommand,
    WrongNumberOfArguments,
    Utf8Error,
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
            Error::Utf8Error => "Wrong UTF-8",
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
            (&Error::Utf8Error, &Error::Utf8Error) => true,
            (&Error::LineTooLong, &Error::LineTooLong) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pixelflut Error: {}", self.description())
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

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::Utf8Error
    }
}

impl Into<IoError> for Error {
    fn into(self) -> IoError {
        IoError::new(io::ErrorKind::InvalidData, self.description())
    }
}
