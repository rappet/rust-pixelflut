//! A module for working with Pixelflut commands.

use std::fmt;
use std::str::FromStr;

use crate::error::{PixelflutError, PixelflutErrorKind, PixelflutResult};
use crate::pixel::{Color, Coordinate, Pixel};
use std::borrow::Cow;

/// A pixelflut command
///
/// Send to the Server
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Px(Pixel),
    Size,
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
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(PixelflutErrorKind::InvalidCommand)?;

        let command = match command {
            "PX" => Command::Px(Pixel::new(
                Coordinate::new(
                    iter.next()
                        .ok_or(PixelflutErrorKind::WrongNumberOfArguments)?
                        .parse()?,
                    iter.next()
                        .ok_or(PixelflutErrorKind::WrongNumberOfArguments)?
                        .parse()?,
                ),
                iter.next()
                    .ok_or(PixelflutErrorKind::WrongNumberOfArguments)?
                    .parse::<Color>()?,
            )),
            "SIZE" => {
                if iter.next().is_some() {
                    return Err(PixelflutErrorKind::WrongNumberOfArguments.into());
                } else {
                    Command::Size
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
    use crate::command::{Command, Response};
    use crate::Pixel;

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
