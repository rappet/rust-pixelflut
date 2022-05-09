//! A module for working with Pixelflut commands.

use std::fmt;
use std::str::FromStr;

use crate::error::{PixelflutError, PixelflutErrorKind, PixelflutResult};
use crate::pixel::Pixel;
use std::borrow::Cow;

/// A pixelflut command
///
/// Send to the Server
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Px(Pixel),
    Size,
}

const COMMAND_SIZE_LEN: usize = b"SIZE".len();
const COMMAND_PX_MIN_LEN: usize = 2 + 1 + 1 + 1 + 1 + 1 + 6; // PX 1 1 000000
const COMMAND_PX_MAX_LEN: usize = 2 + 1 + 10 + 1 + 10 + 1 + 8; // PX 1000000000 1000000000 00000000

impl Command {
    pub fn parse_byte_slice(slice: &[u8]) -> PixelflutResult<Command> {
        match slice.len() {
            COMMAND_SIZE_LEN if slice == b"SIZE" => Ok(Command::Size),
            COMMAND_PX_MIN_LEN..=COMMAND_PX_MAX_LEN if slice[0..3] == *b"PX " => {
                Ok(Command::Px(Pixel::parse_byte_slice(&slice[3..])?))
            }
            _ => Err(PixelflutErrorKind::InvalidCommand.into()),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Px(ref pixel) => write!(f, "PX {}", pixel),
            Command::Size => write!(f, "SIZE"),
        }
    }
}

impl FromStr for Command {
    type Err = PixelflutError;

    fn from_str(s: &str) -> PixelflutResult<Command> {
        Command::parse_byte_slice(s.as_bytes())
    }
}

impl From<Pixel> for Command {
    fn from(pixel: Pixel) -> Command {
        Command::Px(pixel)
    }
}

/// A pixelflut command
///
/// Send to the Client
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Response {
    Size { w: u32, h: u32 },
    Error(Cow<'static, str>),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Response::*;
        match self {
            Size { w, h } => write!(f, "SIZE {} {}", w, h),
            Error(msg) => write!(f, "ERROR {}", msg),
        }
    }
}

impl FromStr for Response {
    type Err = PixelflutError;

    fn from_str(s: &str) -> PixelflutResult<Response> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(PixelflutErrorKind::InvalidCommand)?;

        let command = match command {
            "SIZE" => {
                if let Some(w) = iter.next() {
                    Response::Size {
                        w: w.parse()?,
                        h: iter
                            .next()
                            .ok_or(PixelflutErrorKind::WrongNumberOfArguments)?
                            .parse()?,
                    }
                } else {
                    return Err(PixelflutErrorKind::WrongNumberOfArguments.into());
                }
            }
            "ERROR" => {
                if s.len() > 6 {
                    Response::Error(Cow::Owned(s[6..].into()))
                } else {
                    return Err(PixelflutErrorKind::WrongNumberOfArguments.into());
                }
            }
            _ => return Err(PixelflutErrorKind::InvalidCommand.into()),
        };

        if iter.next() == None {
            Ok(command)
        } else {
            Err(PixelflutErrorKind::WrongNumberOfArguments.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    use crate::{Color, Pixel};

    #[test]
    fn command_parses() {
        assert_eq!(Command::parse_byte_slice(b"SIZE").unwrap(), Command::Size);
        assert_eq!(
            Command::parse_byte_slice(b"PX 123 456 123456").unwrap(),
            Command::Px(Pixel::new((123, 456).into(), Color::rgb(0x12, 0x34, 0x56)))
        );
    }

    proptest! {
        #[test]
        fn parse_command_doesnt_crash(s in "\\PC*") {
            let _ = Command::parse_byte_slice(s.as_bytes());
        }

        #[test]
        fn parse_command_all_valid(s in "PX (0|[1-9][0-9]{0,8}) (0|[1-9][0-9]{0,8}) ([0-9a-fA-F]{6}|[0-9a-fA-F]{8})") {
            assert!(Command::parse_byte_slice(s.as_bytes()).is_ok())
        }
    }

    #[test]
    fn response_parses() {}

    #[test]
    fn display() {
        let pxcommand = Command::Px(Pixel::new((45, 67).into(), (0x11, 0x22, 0x55).into()));

        assert_eq!(format!("{}", pxcommand), "PX 45 67 112255");
        assert_eq!(pxcommand, "PX 45 67 112255".parse().unwrap());
        assert_eq!(format!("{}", Command::Size), "SIZE");
        assert_eq!(Command::Size, "SIZE".parse().unwrap());
        assert_eq!(format!("{}", Response::Size { w: 12, h: 34 }), "SIZE 12 34");
        assert_eq!(
            Response::Size { w: 12, h: 34 },
            "SIZE 12 34".parse().unwrap()
        );
        assert!("SIZE Blah".parse::<Response>().is_err());
        assert!("FOO".parse::<Response>().is_err());
        assert!("FOO".parse::<Response>().is_err());
    }
}
