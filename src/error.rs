use std::convert::From;
use std::error;
use std::fmt;
use std::num::ParseIntError;
use std::result;
use std::str::Utf8Error;

/// Pixelflut [`Result`] alias
pub type PixelflutResult<T> = result::Result<T, PixelflutError>;

/// Pixelflut error type
///
/// Use [PixelflutErrorKind] to match specific errors.
pub struct PixelflutError {
    repr: Repr,
}

impl fmt::Debug for PixelflutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

#[derive(Debug)]
enum Repr {
    Io(std::io::Error),
    ParseInt(ParseIntError),
    Utf8(Utf8Error),
    Simple(PixelflutErrorKind),
    Description(PixelflutErrorKind, &'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PixelflutErrorKind {
    Io,
    InvalidCommand,
    WrongNumberOfArguments,
    Parse,
    State,
    ServerError,
}

impl PixelflutErrorKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            PixelflutErrorKind::Io => "io error",
            PixelflutErrorKind::InvalidCommand => "invalid command",
            PixelflutErrorKind::WrongNumberOfArguments => "wrong number of arguments",
            PixelflutErrorKind::Parse => "parse error",
            PixelflutErrorKind::State => "invalid state",
            PixelflutErrorKind::ServerError => "got error from server",
        }
    }

    pub(crate) fn with_description(self, description: &'static str) -> PixelflutError {
        PixelflutError {
            repr: Repr::Description(self, description),
        }
    }
}

impl From<PixelflutErrorKind> for PixelflutError {
    #[inline]
    fn from(kind: PixelflutErrorKind) -> PixelflutError {
        PixelflutError {
            repr: Repr::Simple(kind),
        }
    }
}

impl PixelflutError {
    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> PixelflutErrorKind {
        match self.repr {
            Repr::Io(_) => PixelflutErrorKind::Io,
            Repr::ParseInt(_) => PixelflutErrorKind::Parse,
            Repr::Utf8(_) => PixelflutErrorKind::Parse,
            Repr::Simple(kind) => kind,
            Repr::Description(kind, _) => kind,
        }
    }
}

impl fmt::Display for PixelflutError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.repr {
            Repr::Io(ref err) => write!(fmt, "io error: {}", err),
            Repr::ParseInt(ref err) => write!(fmt, "parse int error: {}", err),
            Repr::Utf8(err) => write!(fmt, "utf8 error: {}", err),
            Repr::Simple(kind) => write!(fmt, "{}", kind.as_str()),
            Repr::Description(kind, description) => {
                write!(fmt, "{}: {}", kind.as_str(), description)
            }
        }
    }
}

impl error::Error for PixelflutError {
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

impl From<std::io::Error> for PixelflutError {
    fn from(err: std::io::Error) -> PixelflutError {
        PixelflutError {
            repr: Repr::Io(err),
        }
    }
}

impl From<ParseIntError> for PixelflutError {
    fn from(err: ParseIntError) -> PixelflutError {
        PixelflutError {
            repr: Repr::ParseInt(err),
        }
    }
}

impl From<Utf8Error> for PixelflutError {
    fn from(err: Utf8Error) -> PixelflutError {
        PixelflutError {
            repr: Repr::Utf8(err),
        }
    }
}

impl From<bstr::Utf8Error> for PixelflutError {
    fn from(_err: bstr::Utf8Error) -> PixelflutError {
        PixelflutErrorKind::Parse.with_description("UTF-8 error")
    }
}
