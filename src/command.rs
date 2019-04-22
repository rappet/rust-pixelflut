//! A module for working with Pixelflut commands.
//!
//! # Examples
//!
//! Parsing commands:
//! ```
//! use pixelflut::{Command, Pixel, Coordinate};
//! use pixelflut::error::Result;
//!
//! let command: Result<Command> = "PX 10 20 aabbcc".parse();
//!
//! match command {
//!     Ok(Command::Px( Pixel{ position: Coordinate{ x, y }, .. } ))
//!         => println!("draw pixel on {x} {y}", x = x, y = y),
//!     Ok(Command::Size) => println!("resturn size of field"),
//!     Err(err) => println!("client send shit: {}", err),
//! }
//! ```

use std::{fmt};
use std::str::FromStr;

use pixel::{Pixel, Coordinate, Color};
use error::{Error, ErrorKind, Result};

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
            Command::Px ( ref pixel ) => write!(f, "PX {}", pixel),
            Command::Size => write!(f, "SIZE"),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Command> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(ErrorKind::InvalidCommand)?;

        let command = match command {
            "PX" => { Command::Px( Pixel::new(
                Coordinate::new(
                    iter.next()
                        .ok_or(ErrorKind::WrongNumberOfArguments)?.parse()?,
                    iter.next()
                        .ok_or(ErrorKind::WrongNumberOfArguments)?.parse()?
                ),
                iter.next().ok_or(ErrorKind::WrongNumberOfArguments)?.parse::<Color>()?
            ) ) },
            "SIZE" => {
                if iter.next().is_some() {
                    return Err(ErrorKind::WrongNumberOfArguments.into())
                } else {
                    Command::Size
                }
            },
            _ => return Err(ErrorKind::InvalidCommand.into()),
        };

        if iter.next() == None {
            Ok(command)
        } else {
            Err(ErrorKind::WrongNumberOfArguments.into())
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Response {
    Size { w: u32, h: u32 },
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Response::Size { w, h } =>
                write!(f, "SIZE {} {}", w, h),
        }
    }
}

impl FromStr for Response {
    type Err = Error;

    fn from_str(s: &str) -> Result<Response> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(ErrorKind::InvalidCommand)?;

        let command = match command {
            "SIZE" => {
                if let Some(w) = iter.next() {
                    Response::Size {
                        w: w.parse()?,
                        h: iter.next()
                            .ok_or(ErrorKind::WrongNumberOfArguments)?.parse()?,
                    }
                } else {
                    return Err(ErrorKind::WrongNumberOfArguments.into())
                }
            },
            _ => return Err(ErrorKind::InvalidCommand.into()),
        };

        if iter.next() == None {
            Ok(command)
        } else {
            Err(ErrorKind::WrongNumberOfArguments.into())
        }

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use command::{Command, Response};
        use pixel::Pixel;

        let pxcommand = Command::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        assert_eq!( format!("{}", pxcommand), "PX 45 67 112255" );
        assert_eq!( pxcommand, "PX 45 67 112255".parse().unwrap() );
        assert_eq!( format!("{}", Command::Size), "SIZE" );
        assert_eq!( Command::Size, "SIZE".parse().unwrap() );
        assert_eq!(
            format!("{}", Response::Size { w: 12, h: 34 } ),
            "SIZE 12 34"
        );
        assert_eq!(
            Response::Size { w: 12, h: 34 },
            "SIZE 12 34".parse().unwrap()
        );
        assert!( "SIZE Blah".parse::<Response>().is_err() );
        assert!( "FOO".parse::<Response>().is_err() );
        assert!( "FOO".parse::<Response>().is_err() );
    }

}
