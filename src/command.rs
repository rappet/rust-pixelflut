//! A module for working with Pixelflut commands.
//!
//! # Examples
//!
//! Parsing commands:
//! ```
//! use rust_pixelflut::Command;
//! use rust_pixelflut::error::Result;
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

use color::Color;
use error::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Px { x: u32, y: u32, color: Color },
    Size,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Px { ref x, ref y, ref color } => write!(f, "PX {} {} {}", x, y, color),
            Command::Size => write!(f, "SIZE"),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Command> {
        let mut iter = s.split_whitespace();

        let command = iter.next().ok_or(Error::InvalidCommand)?;

        let command = match command {
            "PX" => {
                Command::Px {
                    x: iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?,
                    y: iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?,
                    color: iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?,
                }
            },
            "SIZE" => Command::Size,
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
        use color::Color;
        use error::{Error, Result};

        assert_eq!(
            format!("{}", Command::Px { x: 45, y: 67, color: Color::rgb(0x11, 0x22, 0x55) } ),
            "PX 45 67 112255"
        );
        assert_eq!(
            Ok(Command::Px { x: 45, y: 67, color: Color::rgb(0x11, 0x22, 0x55) }),
            "PX 45 67 112255".parse()
        );
        assert_eq!( Ok(Command::Size), "SIZE".parse() );
        assert_eq!( Err(Error::WrongNumberOfArguments) as Result<Command>, "SIZE Blah".parse() );
        assert_eq!( Err(Error::InvalidCommand) as Result<Command>, "FOO".parse() );
    }
}
