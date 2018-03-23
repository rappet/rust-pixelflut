//! A module for working with Pixelflut commands.
//!
//! # Examples
//!
//! Parsing commands:
//! ```
//! use pixelflut::Command;
//! use pixelflut::error::Result;
//!
//! let command: Result<Command> = "PX 10 20 aabbcc".parse();
//!
//! match command {
//!     Ok(Command::Px { x, y, _ }) => println!("draw pixel on {x} {y}", x, y),
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
pub enum ServerCommand {
    Px(Pixel),
    Size,
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerCommand::Px ( ref pixel ) => write!(f, "PX {}", pixel),
            ServerCommand::Size => write!(f, "SIZE"),
        }
    }
}

impl FromStr for ServerCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<ServerCommand> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(ErrorKind::InvalidCommand)?;

        let command = match command {
            "PX" => { ServerCommand::Px( Pixel::new(
                Coordinate::new(
                    iter.next()
                        .ok_or(ErrorKind::WrongNumberOfArguments)?.parse()?,
                    iter.next()
                        .ok_or(ErrorKind::WrongNumberOfArguments)?.parse()?
                ),
                iter.next().ok_or(ErrorKind::WrongNumberOfArguments)?.parse::<Color>()?
            ) ) },
            "SIZE" => {
                if let Some(_) = iter.next() {
                    return Err(ErrorKind::WrongNumberOfArguments.into())
                } else {
                    ServerCommand::Size
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

impl From<Pixel> for ServerCommand {
    fn from(pixel: Pixel) -> ServerCommand {
        ServerCommand::Px(pixel)
    }
}

///
///
///
///
///

/// A pixelflut command
/// 
/// Send to the Client
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClientCommand {
    SizeResponse { w: u32, h: u32 },
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientCommand::SizeResponse { w, h } =>
                write!(f, "SIZE {} {}", w, h),
        }
    }
}

impl FromStr for ClientCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<ClientCommand> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(ErrorKind::InvalidCommand)?;

        let command = match command {
            "SIZE" => {
                if let Some(w) = iter.next() {
                    ClientCommand::SizeResponse {
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
        use command::{ServerCommand, ClientCommand};
        use pixel::Pixel;

        let pxcommand = ServerCommand::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        assert_eq!( format!("{}", pxcommand), "PX 45 67 112255" );
        assert_eq!( pxcommand, "PX 45 67 112255".parse().unwrap() );
        assert_eq!( format!("{}", ServerCommand::Size), "SIZE" );
        assert_eq!( ServerCommand::Size, "SIZE".parse().unwrap() );
        assert_eq!(
            format!("{}", ClientCommand::SizeResponse { w: 12, h: 34 } ),
            "SIZE 12 34"
        );
        assert_eq!(
            ClientCommand::SizeResponse { w: 12, h: 34 },
            "SIZE 12 34".parse().unwrap()
        );
        assert!( "SIZE Blah".parse::<ClientCommand>().is_err() );
        assert!( "FOO".parse::<ServerCommand>().is_err() );
        assert!( "FOO".parse::<ClientCommand>().is_err() );
    }

}
