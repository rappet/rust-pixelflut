use std::error;
use std::fmt;
use std::result;
use std::convert::From;
use std::str::Utf8Error;
use std::num::ParseIntError;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    repr: Repr,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

#[derive(Debug)]
enum Repr {
    Io(std::io::Error),
    ParseInt(ParseIntError),
    Utf8(Utf8Error),
    Simple(ErrorKind),
    Description(ErrorKind, &'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    Io,
    InvalidCommand,
    WrongNumberOfArguments,
    Parse,
}

impl ErrorKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ErrorKind::Io => "Io error",
            ErrorKind::InvalidCommand => "invalid command",
            ErrorKind::WrongNumberOfArguments => "wrong number of arguments",
            ErrorKind::Parse => "parse error",
        }
    }

    pub(crate) fn with_description(self, description: &'static str) -> Error {
        Error {
            repr: Repr::Description(self, description)
        }
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error {
            repr: Repr::Simple(kind)
        }
    }
}

impl Error {
    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Io(_) => ErrorKind::Io,
            Repr::ParseInt(_) => ErrorKind::Parse,
            Repr::Utf8(_) => ErrorKind::Parse,
            Repr::Simple(kind) => kind,
            Repr::Description(kind, _) => kind,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.repr {
            Repr::Io(ref err) => write!(fmt, "io error: {}", err),
            Repr::ParseInt(ref err) => write!(fmt, "parse int error: {}", err),
            Repr::Utf8(err) => write!(fmt, "utf8 error: {}", err),
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
            Repr::Description(kind, description) => write!(fmt, "{}: {}", kind.as_str(), description),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(..) => self.kind().as_str(),
            Repr::Description(_, description) => description,
            Repr::Io(ref err) => err.description(),
            Repr::ParseInt(ref err) => err.description(),
            Repr::Utf8(ref err) => err.description(),
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Io(ref err) => err.source(),
            Repr::ParseInt(ref err) => err.source(),
            Repr::Utf8(ref err) => err.source(),
            Repr::Simple(..) => None,
            Repr::Description(..) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            repr: Repr::Io(err),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error {
            repr: Repr::ParseInt(err),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error {
            repr: Repr::Utf8(err),
        }
    }
}