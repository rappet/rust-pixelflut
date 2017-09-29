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

use pixel::{Pixel, Coordinate};
use error::{Error, Result};

/// A pixelflut command
/// 
/// could be client- or serverbound.
/// Check with `is_clientbound` or `is_serverbound`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Px(Pixel),
    Size,
    SizeResponse { w: u32, h: u32 },
}

impl Command {
    /// Check if command can be send to client.
    pub fn is_clientbound(&self) -> bool {
        match self {
            &Command::Px (_) => false,
            &Command::Size => false,
            &Command::SizeResponse { .. } => true,
        }
    }

    /// Chekc if command can be send to server.
    pub fn is_serverbound(&self) -> bool {
        match self {
            &Command::Px(_) => true,
            &Command::Size => true,
            &Command::SizeResponse { .. } => false,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Px ( ref pixel ) => write!(f, "PX {}", pixel),
            Command::Size => write!(f, "SIZE"),
            Command::SizeResponse { w, h } =>
                write!(f, "SIZE {} {}", w, h),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Command> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(Error::InvalidCommand)?;

        let command = match command {
            "PX" => { Command::Px( Pixel::new(
                Coordinate::new(
                    iter.next()
                        .ok_or(Error::WrongNumberOfArguments)?.parse()?,
                    iter.next()
                        .ok_or(Error::WrongNumberOfArguments)?.parse()?
                ),
                iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?
            ) ) },
            "SIZE" => {
                if let Some(w) = iter.next() {
                    Command::SizeResponse {
                        w: w.parse()?,
                        h: iter.next()
                            .ok_or(Error::WrongNumberOfArguments)?.parse()?,
                    }
                } else {
                    Command::Size
                }
            },
            _ => return Err(Error::InvalidCommand),
        };

        if iter.next() == None {
            Ok(command)
        } else {
            Err(Error::WrongNumberOfArguments)
        }

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use command::Command;
        use pixel::{Pixel, Coordinate, Color};

        let pxcommand = Command::Px( Pixel::new(
            Coordinate::new( 45, 67 ),
            Color::rgb(0x11, 0x22, 0x55),
        ) );

        assert_eq!( format!("{}", pxcommand), "PX 45 67 112255" );
        assert_eq!( pxcommand, "PX 45 67 112255".parse().unwrap() );
        assert_eq!( format!("{}", Command::Size), "SIZE" );
        assert_eq!( Command::Size, "SIZE".parse().unwrap() );
        assert_eq!(
            format!("{}", Command::SizeResponse { w: 12, h: 34 } ),
            "SIZE 12 34"
        );
        assert_eq!(
            Command::SizeResponse { w: 12, h: 34 },
            "SIZE 12 34".parse().unwrap()
        );
        assert!( "SIZE Blah".parse::<Command>().is_err() );
        assert!( "FOO".parse::<Command>().is_err() );
    }

    #[test]
    fn is_clientbound() {
        assert!( !"PX 12 34 112233".parse::<Command>().unwrap().is_clientbound() );
        assert!( !"SIZE".parse::<Command>().unwrap().is_clientbound() );
        assert!( "SIZE 12 34".parse::<Command>().unwrap().is_clientbound() );
    }

    #[test]
    fn is_serverbound() {
        assert!( "PX 12 34 112233".parse::<Command>().unwrap().is_serverbound() );
        assert!( "SIZE".parse::<Command>().unwrap().is_serverbound() );
        assert!( !"SIZE 12 34".parse::<Command>().unwrap().is_serverbound() );
    }
}
