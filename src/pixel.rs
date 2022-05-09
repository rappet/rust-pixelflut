//! A module that contians pixels for pixelflut.
use bstr::ByteSlice;

use crate::error::{PixelflutError, PixelflutErrorKind, PixelflutResult};
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

/// Pixelflut pixel containing a coordinate and a color
#[derive(Copy, Clone, PartialEq, Hash, Debug, Default)]
pub struct Pixel {
    pub position: Coordinate,
    pub color: Color,
}

impl Pixel {
    /// construct a new `Pixel` with a `Coordinate` and a `Color`
    #[must_use]
    pub fn new(position: Coordinate, color: Color) -> Self {
        Self { position, color }
    }

    pub fn parse_byte_slice(slice: &[u8]) -> PixelflutResult<Self> {
        let split_index = slice
            .find_iter(&b" ")
            .nth(1)
            .ok_or(PixelflutErrorKind::WrongNumberOfArguments)?;
        let position = Coordinate::parse_byte_slice(&slice[..split_index])?;
        let color = Color::parse_byte_slice(&slice[split_index + 1..])?;
        Ok(Self { position, color })
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.position, self.color)
    }
}

impl FromStr for Pixel {
    type Err = PixelflutError;

    fn from_str(s: &str) -> PixelflutResult<Self> {
        let mut iter = s.split_whitespace();
        let pixel = Self::new(
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
        );
        if iter.next().is_some() {
            Err(PixelflutErrorKind::WrongNumberOfArguments.into())
        } else {
            Ok(pixel)
        }
    }
}

impl<P: Into<Coordinate>, C: Into<Color>> From<(P, C)> for Pixel {
    fn from((position, color): (P, C)) -> Self {
        Self::new(position.into(), color.into())
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
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn parse_byte_slice(slice: &[u8]) -> PixelflutResult<Self> {
        let mut it = slice.splitn(2, |b| *b == b' ');
        // TODO replace decimal parsing with something faster
        let x: u32 = atoi::atoi(it.next().ok_or(
            PixelflutErrorKind::Parse.with_description("First coordinate from pixel is missing"),
        )?)
        .ok_or(PixelflutErrorKind::Parse.with_description("Failed parsing first coordinate"))?;
        let y: u32 = atoi::atoi(it.next().ok_or(
            PixelflutErrorKind::Parse.with_description("Second coordinate from pixel is missing"),
        )?)
        .ok_or(PixelflutErrorKind::Parse.with_description("Failed parsing second coordinate"))?;
        Ok(Self { x, y })
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from(coordinate: (u32, u32)) -> Self {
        Self::new(coordinate.0, coordinate.1)
    }
}

impl From<Coordinate> for (u32, u32) {
    fn from(coordinate: Coordinate) -> (u32, u32) {
        (coordinate.x, coordinate.y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

/// RGB color type with optional alpha channel
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
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: None }
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
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: Some(a),
        }
    }

    pub fn parse_byte_slice(slice: &[u8]) -> PixelflutResult<Self> {
        match slice.len() {
            6 => {
                let r = parse_hex_byte(&slice[0..2]);
                let g = parse_hex_byte(&slice[2..4]);
                let b = parse_hex_byte(&slice[4..6]);

                match (r, g, b) {
                    (Some(r), Some(g), Some(b)) => Ok(Self::rgb(r, g, b)),
                    _ => Err(PixelflutErrorKind::Parse
                        .with_description("Could not parse hex value in RGB color code")),
                }
            }
            8 => {
                let r = parse_hex_byte(&slice[0..2]);
                let g = parse_hex_byte(&slice[2..4]);
                let b = parse_hex_byte(&slice[4..6]);
                let a = parse_hex_byte(&slice[6..8]);
                match (r, g, b, a) {
                    (Some(r), Some(g), Some(b), Some(a)) => Ok(Self::rgba(r, g, b, a)),
                    _ => Err(PixelflutErrorKind::Parse
                        .with_description("Could not parse hex value in RGBA color code")),
                }
            }
            _ => Err(PixelflutErrorKind::Parse.with_description(
                "Color has wrong length. Falid formats are [0-9a-fA-F]{6}|[0-9a-fA-F]{8}",
            )),
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
    #[must_use]
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
    #[must_use]
    pub const fn packed(r: u8, g: u8, b: u8, a: u8) -> Self {
        match a {
            255 => Self::rgb(r, g, b),
            a => Self::rgba(r, g, b, a),
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
    #[must_use]
    pub const fn pack(&self) -> Self {
        match self.a {
            None | Some(255) => Self::rgb(self.r, self.g, self.b),
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
    #[must_use]
    pub const fn normalized(self) -> (u8, u8, u8, u8) {
        match self.a {
            Some(a) => (self.r, self.g, self.b, a),
            None => (self.r, self.g, self.b, 255),
        }
    }
}

#[inline]
const fn parse_hex_byte(slice: &[u8]) -> Option<u8> {
    #[inline]
    const fn parse_hex_nibble(nibble: u8) -> Option<u8> {
        match nibble {
            b'0'..=b'9' => Some(nibble - b'0'),
            b'a'..=b'f' => Some(nibble - b'a' + 10),
            b'A'..=b'F' => Some(nibble - b'A' + 10),
            _ => None,
        }
    }

    if slice.len() == 2 {
        match (parse_hex_nibble(slice[0]), parse_hex_nibble(slice[1])) {
            (Some(hi), Some(lo)) => Some((hi << 4) | lo),
            _ => None,
        }
    } else {
        None
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Returns a RGB Color
    fn from(color: (u8, u8, u8)) -> Self {
        Self::rgb(color.0, color.1, color.2)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    /// Returns a RGBA Color
    fn from(color: (u8, u8, u8, u8)) -> Self {
        Self::rgba(color.0, color.1, color.2, color.3)
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
        let [r, g, b] = color.0;
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
        let [r, g, b, a] = color.0;
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
    type Err = PixelflutError;

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
    fn from_str(s: &str) -> PixelflutResult<Self> {
        Self::parse_byte_slice(s.as_bytes())
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
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn pixel_parses() {
        assert_eq!(
            Pixel::new(Coordinate::new(10, 20), Color::rgb(0x11, 0x22, 0x33),),
            Pixel::parse_byte_slice(b"10 20 112233").unwrap()
        );
    }

    proptest! {
        #[test]
        fn parse_pixel_doesnt_crash(s in "\\PC*") {
            let _ = Pixel::parse_byte_slice(s.as_bytes());
        }

        #[test]
        fn parse_pixel_all_valid(s in "(0|[1-9][0-9]{0,8}) (0|[1-9][0-9]{0,8}) ([0-9a-fA-F]{6}|[0-9a-fA-F]{8})") {
            assert!(Pixel::parse_byte_slice(s.as_bytes()).is_ok())
        }
    }

    #[test]
    fn coordinate_parses() {
        assert_eq!(
            Coordinate::parse_byte_slice(b"0 0").unwrap(),
            Coordinate::new(0, 0)
        );
        assert_eq!(
            Coordinate::parse_byte_slice(b"1 2").unwrap(),
            Coordinate::new(1, 2)
        );
        assert_eq!(
            Coordinate::parse_byte_slice(b"1234 2345").unwrap(),
            Coordinate::new(1234, 2345)
        );
        assert_eq!(
            Coordinate::parse_byte_slice(b"1000000000 1000000000").unwrap(),
            Coordinate::new(1_000_000_000, 1_000_000_000)
        );
    }

    proptest! {
        #[test]
        fn parse_coordinate_doesnt_crash(s in "\\PC*") {
            let _ = Coordinate::parse_byte_slice(s.as_bytes());
        }

        #[test]
        fn parse_coordinate_all_valid(s in "(0|[1-9][0-9]{0,8}) (0|[1-9][0-9]{0,8})") {
            assert!(Coordinate::parse_byte_slice(s.as_bytes()).is_ok())
        }
    }

    #[test]
    fn color_parses() {
        assert_eq!(
            Color::parse_byte_slice(b"000000").unwrap(),
            Color::rgb(0, 0, 0)
        );
        assert_eq!(
            Color::parse_byte_slice(b"00000000").unwrap(),
            Color::rgba(0, 0, 0, 0)
        );
        assert_eq!(
            Color::parse_byte_slice(b"123456").unwrap(),
            Color::rgb(0x12, 0x34, 0x56)
        );
        assert_eq!(
            Color::parse_byte_slice(b"123456Ab").unwrap(),
            Color::rgba(0x12, 0x34, 0x56, 0xab)
        );
        assert!(Color::parse_byte_slice(b"").is_err());
    }

    proptest! {
        #[test]
        fn parse_color_doesnt_crash(s in "\\PC*") {
            let _ = Color::parse_byte_slice(s.as_bytes());
        }

        #[test]
        fn parse_color_all_valid(s in "[0-9a-fA-F]{6}|[0-9a-fA-F]{8}") {
            assert!(Color::parse_byte_slice(s.as_bytes()).is_ok())
        }
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

    #[test]
    fn hex_byte_parses() {
        assert_eq!(parse_hex_byte(b"12"), Some(0x12));
        assert_eq!(parse_hex_byte(b"a0"), Some(0xa0));
        assert_eq!(parse_hex_byte(b"A0"), Some(0xa0));
        assert_eq!(parse_hex_byte(b"00"), Some(0x00));
        assert_eq!(parse_hex_byte(b"1"), None);
        assert_eq!(parse_hex_byte(b"123"), None);
    }

    proptest! {
        #[test]
        fn parse_hex_bytes_doesnt_crash(s in "\\PC*") {
            let _ = parse_hex_byte(s.as_bytes());
        }

        #[test]
        fn parse_hex_bytes_all_valid(s in "[0-9a-fA-F]{2}") {
            assert!(parse_hex_byte(s.as_bytes()).is_some())
        }
    }
}
