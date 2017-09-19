//! A module that contians pixels for pixelflut.
use std::fmt;
use std::str::FromStr;
use error::{Error, Result};

/// pixelflut pixel
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Pixel {
    position: Coordinate,
    color: Color,
}

impl Pixel {
    /// construct a new `Pixel` with a `Coordinate` and a `Color`
    fn new(position: Coordinate, color: Color) -> Pixel {
        Pixel {
            position: position,
            color: color,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.position, self.color)
    }
}

impl FromStr for Pixel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Pixel> {
        let mut iter = s.split_whitespace();
        let pixel = Pixel::new(
            Coordinate::new(
                iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?,
                iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?
            ),
            iter.next().ok_or(Error::WrongNumberOfArguments)?.parse()?
        );
        if iter.next().is_some() {
            Err(Error::WrongNumberOfArguments)
        } else {
            Ok(pixel)
        }
    }
}

/// coordinate on a pixelflut grid
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    /// Constructs a new `Coordinate` with given x and y position.
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
        }
    }

    /// returns x and y coordinate
    ///
    /// ```no_run
    /// use pixelflut::Coordinate;
    /// let pos = Coordinate::new(1, 2);
    /// let (x, y) = pos.xy();
    /// assert_eq!(1, x);
    /// assert_eq!(2, y);
    /// ```
    pub fn xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Copy, Clone, PartialEq, Hash)]
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
    /// use pixelflut::Color;
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
    /// use pixelflut::Color;
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
    /// use pixelflut::Color;
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
    use Pixel;
    use Coordinate;
    use Color;
    use error::Error;

    #[test]
    fn test_pixel_from_str() {
        assert_eq!(Pixel::new(
            Coordinate::new(10, 20),
            Color::rgb(0x11, 0x22, 0x33),
        ), "10 20 112233".parse().unwrap());
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!(Color { r: 0x11, g: 0x22, b: 0x33, a: None },
                   Color::rgb(0x11, 0x22, 0x33));
    }

    #[test]
    fn test_color_rgba() {
        assert_eq!(Color { r: 0x11, g: 0x22, b: 0x33, a: Some(0x44)},
                   Color::rgba(0x11, 0x22, 0x33, 0x44));
    }

    #[test]
    fn test_color_normalized() {
        assert_eq!((0x11, 0x22, 0x33, 0xff),
                   Color::rgb(0x11, 0x22, 0x33).normalized());
        assert_eq!((0x11, 0x22, 0x33, 0x44),
                   Color::rgba(0x11, 0x22, 0x33, 0x44).normalized());
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!(Color::rgb(0x11, 0x22, 0x33), "112233".parse().unwrap());
        assert_eq!(Color::rgba(0x11, 0x22, 0x33, 0xee), "112233ee".parse().unwrap());
        assert_eq!(Error::ColorLength, "".parse::<Color>().unwrap_err());
        assert_eq!(Error::ColorLength, "123".parse::<Color>().unwrap_err());
        assert_eq!(Error::ColorLength, "12345".parse::<Color>().unwrap_err());
        assert!(" 1 2 3".parse::<Color>().is_err()); // Could be better
        assert!("112g33".parse::<Color>().is_err()); // Could be better
    }
}
