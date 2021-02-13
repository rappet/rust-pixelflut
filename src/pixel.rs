//! A module that contians pixels for pixelflut.
use crate::error::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

pub static MAX_FORMATTED_COORDINATE_SIZE: usize = 10;
pub static MAX_FORMATTED_COLOR_SIZE: usize = 8;
pub static MAX_FORMATTED_PIXEL_SIZE: usize = 3
    + MAX_FORMATTED_COORDINATE_SIZE
    + 1
    + MAX_FORMATTED_COORDINATE_SIZE
    + 1
    + MAX_FORMATTED_COLOR_SIZE;
pub static MAX_FORMATTED_PIXEL_SIZE_NEWLINE: usize = MAX_FORMATTED_PIXEL_SIZE + 1;

/// pixelflut pixel
#[derive(Copy, Clone, PartialEq, Hash, Debug, Default)]
pub struct Pixel {
    pub position: Coordinate,
    pub color: Color,
}

impl Pixel {
    /// construct a new `Pixel` with a `Coordinate` and a `Color`
    pub fn new(position: Coordinate, color: Color) -> Pixel {
        Pixel { position, color }
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
                iter.next()
                    .ok_or(ErrorKind::WrongNumberOfArguments)?
                    .parse()?,
                iter.next()
                    .ok_or(ErrorKind::WrongNumberOfArguments)?
                    .parse()?,
            ),
            iter.next()
                .ok_or(ErrorKind::WrongNumberOfArguments)?
                .parse::<Color>()?,
        );
        if iter.next().is_some() {
            Err(ErrorKind::WrongNumberOfArguments.into())
        } else {
            Ok(pixel)
        }
    }
}

impl<P: Into<Coordinate>, C: Into<Color>> From<(P, C)> for Pixel {
    fn from((position, color): (P, C)) -> Self {
        Pixel::new(position.into(), color.into())
    }
}

/// coordinate on a pixelflut grid
#[derive(Copy, Clone, PartialEq, Hash, Debug, Default)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

impl Coordinate {
    /// Constructs a new `Coordinate` with given x and y position.
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from(coordinate: (u32, u32)) -> Coordinate {
        Coordinate::new(coordinate.0, coordinate.1)
    }
}

impl Into<(u32, u32)> for Coordinate {
    fn into(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
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
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: None }
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
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a: Some(a),
        }
    }

    /// Returns the alpha channel of the `Color`.
    ///
    /// If the color does not have an alpha channel, `255` will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// use pixelflut::Color;
    ///
    /// let without_alpha = Color::rgb(123, 123, 123);
    /// assert_eq!(without_alpha.alpha(), 255);
    ///
    /// let with_alpha = Color::rgba(123, 123, 123, 123);
    /// assert_eq!(with_alpha.alpha(), 123)
    /// ```
    pub const fn alpha(self) -> u8 {
        if let Some(alpha) = self.a {
            alpha
        } else {
            255
        }
    }

    /// Constructs a new `Color` using the alpha channel only, if a != 255.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pixelflut::Color;
    /// assert_eq!(Color::packed(123, 23, 42, 255), Color::rgb(123, 23, 42));
    /// assert_eq!(Color::packed(123, 23, 42, 64), Color::rgba(123, 23, 42, 64));
    /// ```
    pub const fn packed(r: u8, g: u8, b: u8, a: u8) -> Color {
        match a {
            255 => Color::rgb(r, g, b),
            a => Color::rgba(r, g, b, a),
        }
    }

    /// Strips the alpha channel if not existent or value is 255.Color
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use pixelflut::Color;
    /// assert_eq!(Color::rgba(12, 34, 56, 255).pack(), Color::rgb(12, 34, 56));
    /// assert_eq!(Color::rgb(12, 34, 56).pack(), Color::rgb(12, 34, 56));
    /// assert_eq!(Color::rgba(12, 34, 56, 78).pack(), Color::rgba(12, 34, 56, 78));
    /// ```
    pub const fn pack(&self) -> Color {
        match self.a {
            None | Some(255) => Color::rgb(self.r, self.g, self.b),
            _ => *self,
        }
    }

    /// Returns a 4-Tuple with the components red, green, blue and alpha
    ///
    /// If no alpha channel is present, 255 is returned as the alpha channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixelflut::Color;
    /// assert_eq!((255, 0, 0, 255), Color::rgb(255, 0, 0).normalized())
    /// ```
    pub const fn normalized(self) -> (u8, u8, u8, u8) {
        match self.a {
            Some(a) => (self.r, self.g, self.b, a),
            None => (self.r, self.g, self.b, 255),
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Returns a RGB Color
    fn from(color: (u8, u8, u8)) -> Color {
        Color::rgb(color.0, color.1, color.2)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    /// Returns a RGBA Color
    fn from(color: (u8, u8, u8, u8)) -> Color {
        Color::rgba(color.0, color.1, color.2, color.3)
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8) {
        (color.r, color.g, color.b)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8, u8) {
        (color.r, color.g, color.b, color.alpha())
    }
}

#[cfg(feature = "image")]
impl From<image::Rgb<u8>> for Color {
    /// Returns a Rgb Color
    fn from(color: image::Rgb<u8>) -> Color {
        let [r, g, b] = color.data;
        Color::rgb(r, g, b)
    }
}

#[cfg(feature = "image")]
impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb([self.r, self.g, self.b])
    }
}

#[cfg(feature = "image")]
impl From<image::Rgba<u8>> for Color {
    /// Returns a Rgba Color
    fn from(color: image::Rgba<u8>) -> Color {
        let [r, g, b, a] = color.data;
        Color::packed(r, g, b, a)
    }
}

#[cfg(feature = "image")]
impl Into<image::Rgba<u8>> for Color {
    fn into(self) -> image::Rgba<u8> {
        image::Rgba([self.r, self.g, self.b, self.a.unwrap_or(255)])
    }
}

impl FromStr for Color {
    type Err = Error;

    /// Converts a string to a color
    ///
    /// # Examples
    ///
    /// ```
    /// use pixelflut::Color;
    /// assert_eq!(Color::rgb(0x11, 0x22, 0x33), "112233".parse().unwrap());
    /// assert_eq!(Color::rgba(0x11, 0x22, 0x33, 0xee), "112233ee".parse().unwrap());
    /// assert!("".parse::<Color>().is_err());
    /// assert!("123".parse::<Color>().is_err());
    /// assert!("12345".parse::<Color>().is_err());
    /// assert!(" 1 2 3".parse::<Color>().is_err());
    /// assert!("112g33".parse::<Color>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Color> {
        match s.len() {
            6 => Ok(Color::rgb(
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
            )),
            8 => Ok(Color::rgba(
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
                u8::from_str_radix(&s[6..8], 16)?,
            )),
            _ => Err(ErrorKind::Parse.with_description("color length is wrong")),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.a {
            Some(a) => write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a),
            None => write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b),
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
    use crate::{Color, Coordinate, Pixel};

    #[test]
    fn test_pixel_from_str() {
        assert_eq!(
            Pixel::new(Coordinate::new(10, 20), Color::rgb(0x11, 0x22, 0x33),),
            "10 20 112233".parse().unwrap()
        );
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!(
            Color {
                r: 0x11,
                g: 0x22,
                b: 0x33,
                a: None
            },
            Color::rgb(0x11, 0x22, 0x33)
        );
    }

    #[test]
    fn test_color_rgba() {
        assert_eq!(
            Color {
                r: 0x11,
                g: 0x22,
                b: 0x33,
                a: Some(0x44)
            },
            Color::rgba(0x11, 0x22, 0x33, 0x44)
        );
    }

    #[test]
    fn test_color_normalized() {
        assert_eq!(
            (0x11, 0x22, 0x33, 0xff),
            Color::rgb(0x11, 0x22, 0x33).normalized()
        );
        assert_eq!(
            (0x11, 0x22, 0x33, 0x44),
            Color::rgba(0x11, 0x22, 0x33, 0x44).normalized()
        );
    }
}
