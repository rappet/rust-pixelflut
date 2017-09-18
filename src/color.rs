//! A module for working with colors used in Pixelflut.

use std::{fmt};
use std::str::FromStr;

use error::{Error, Result};

/// `Color` is a type that represents a RGB-Color with an option alpha channel.
/// Each component has 8 bits.
///
/// If no alpha channel is present, it is allways ignored if possible. If the
/// alpha channel is needet, it is assumed to be 255.
///
/// This type is used with the PX Command and can be created using  the
/// [rgb](#method.rgb), [rgba](#method.rgba) or [parse] methos.
///
/// # Examples
///
/// ```
/// let a: Color = "ff0000".parse();
/// let b = Color::rgb(255, 0, 0);
/// assert_eq!(a.unwrap(), b);
///
/// let c: Color = "ff0000dd".parse();
/// let d = Color::rgba(0xff, 0, 0, 0xdd);
/// assert_eq!(c.unwrap(), d);
/// ```
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl Color {
    /// Constructs a new `Color` without using an alpha channel.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// Color::rgb(255, 255, 255);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: None,
        }
    }

    /// Constructs a new `Color` using an alpha channel.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// Color::rgba(255, 255, 255, 255);
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: Some(a),
        }
    }

    /// Returns a 4-Tuple with the components red, green, blue and alpha
    ///
    /// If no alpha channel is present, 255 is returned as the alpha channel.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// assert_eq!((255, 0, 0, 255), Color::rgb(255, 0, 0).normalized())
    /// ```
    pub fn normalized(&self) -> (u8, u8, u8, u8) {
        match self.a {
            Some(a) => (self.r, self.g, self.b, a),
            None    => (self.r, self.g, self.b, 255),
        }
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Color> {
        match s.len() {
            6  => Ok( Color::rgb (
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
            )),
            8  => Ok( Color::rgba (
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
                u8::from_str_radix(&s[6..8], 16)?,
            )),
            _ => Err(Error::ColorLength)

        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.a {
            Some(a) => write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a),
            None    => write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b),
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: 0x{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Error};
    use std::num::ParseIntError;

    #[test]
    fn test_rgb() {
        assert_eq!(Color { r: 0x11, g: 0x22, b: 0x33, a: None },
                   Color::rgb(0x11, 0x22, 0x33));
    }

    #[test]
    fn test_rgba() {
        assert_eq!(Color { r: 0x11, g: 0x22, b: 0x33, a: Some(0x44)},
                   Color::rgba(0x11, 0x22, 0x33, 0x44));
    }

    #[test]
    fn test_normalized() {
        assert_eq!((0x11, 0x22, 0x33, 0xff),
                   Color::rgb(0x11, 0x22, 0x33).normalized());
        assert_eq!((0x11, 0x22, 0x33, 0x44),
                   Color::rgba(0x11, 0x22, 0x33, 0x44).normalized());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Color::rgb(0x11, 0x22, 0x33), "112233".parse().unwrap());
        assert_eq!(Color::rgba(0x11, 0x22, 0x33, 0xee), "112233ee".parse().unwrap());
        assert_eq!(Error::ColorLength, "".parse::<Color>().unwrap_err());
        assert_eq!(Error::ColorLength, "123".parse::<Color>().unwrap_err());
        assert_eq!(Error::ColorLength, "12345".parse::<Color>().unwrap_err());
        assert!(" 1 2 3".parse::<Color>().is_err()); // Could be better
        assert!("112g33".parse::<Color>().is_err()); // Could be better
    }

}
