use crate::pixel::MAX_FORMATTED_PIXEL_SIZE_NEWLINE;
use crate::Pixel;
use std::io::Write;
use std::sync::Arc;

pub static PIXEL_BUFFER_DEFAULT_CAPACITY: usize = 8 * 1024;

pub struct PixelBuffer {
    buffer: Vec<u8>,
    number_writer: NumberWriter,
}

impl PixelBuffer {
    /// Constructs a new PixelBuffer with the specified capacity.
    ///
    /// The capacity is given in bytes. If you need a Capacity in Pixels,
    /// consider [with_capacity_pixels].
    ///
    /// [with_capacity_pixels]: Self::with_capacity_pixels
    pub fn with_capacity(capacity: usize) -> PixelBuffer {
        PixelBuffer {
            buffer: Vec::with_capacity(capacity),
            number_writer: NumberWriter::default(),
        }
    }

    /// Constructs a new PixelfBuffer with the specified estimated capacity in Pixels.
    ///
    /// The capacity is given in pixels.
    /// The actual capacity would be higher in most cases,
    /// as the worst-cases length of a formated Pixel is used.
    pub fn with_capacity_pixels(pixels: usize) -> PixelBuffer {
        Self::with_capacity(pixels * MAX_FORMATTED_PIXEL_SIZE_NEWLINE)
    }

    /// Creates a new PixelBuffer with the default capacity.
    pub fn new() -> PixelBuffer {
        Self::with_capacity(PIXEL_BUFFER_DEFAULT_CAPACITY)
    }

    /// Extracts a slice containing the entire internal buffer.
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Returns `true`, if the internal buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixelflut::{PixelBuffer, Pixel};
    /// let mut buffer = PixelBuffer::new();
    /// assert_eq!(buffer.is_empty(), true);
    /// buffer.write_pixel(&Pixel::default());
    /// assert_eq!(buffer.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns `true`, if the internal buffer is so large,
    /// that the buffer might need to be resized,
    /// if another pixel is added.
    pub fn is_capacity_reached(&self) -> bool {
        self.buffer.capacity() < self.buffer.len() + MAX_FORMATTED_PIXEL_SIZE_NEWLINE
    }

    /// Clears the contained buffer.
    /// After this, no pixels are in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixelflut::{PixelBuffer, Pixel};
    /// let mut buffer = PixelBuffer::new();
    /// buffer.write_pixel(&Pixel::default());
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Writes a pixel to the internal buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixelflut::{PixelBuffer, Pixel};
    /// let mut buffer = PixelBuffer::new();
    /// buffer.write_pixel(&Pixel::new((12, 34).into(), (255, 0, 10).into()));
    /// buffer.write_pixel(&Pixel::new((13, 34).into(), (255, 0, 10).into()));
    /// assert_eq!(buffer.as_slice(), b"PX 12 34 ff000a\nPX 13 34 ff000a\n")
    /// ```
    pub fn write_pixel(&mut self, pixel: &Pixel) {
        let (x, y) = pixel.position.into();
        let color = pixel.color;

        self.buffer.write_all(b"PX ").unwrap();
        self.number_writer.write_decimal(&mut self.buffer, x as usize).unwrap();
        self.buffer.write_all(b" ").unwrap();
        self.number_writer.write_decimal(&mut self.buffer, y as usize).unwrap();
        self.buffer.write_all(b" ").unwrap();
        self.number_writer.write_hex02(&mut self.buffer, color.r).unwrap();
        self.number_writer.write_hex02(&mut self.buffer, color.g).unwrap();
        self.number_writer.write_hex02(&mut self.buffer, color.b).unwrap();
        if let Some(a) = color.a {
            self.number_writer.write_hex02(&mut self.buffer, a).unwrap();
        }
        self.buffer.write_all(b"\n").unwrap();
    }
}

impl AsRef<[u8]> for PixelBuffer {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_slice()
    }
}

lazy_static! {
    static ref NUMBER_WRITER: NumberWriter = NumberWriter::create();
}
static NUMBER_WRITER_DEFAULT_MAX_DECIMAL: usize = 4096;

/// Preformatted numbers for faster integer formatting.
#[derive(Clone)]
struct NumberWriter {
    hex02: Arc<[[u8; 2]; 256]>,
    decimal: Arc<Vec<String>>,
}

impl NumberWriter {
    /// Generate a new NumberWriter
    pub fn with_decimal_size(decimal_size: usize) -> NumberWriter {
        let mut hex02 = [[0u8; 2]; 256];
        for i in 0..=255 {
            let str = format!("{:02x}", i);
            hex02[i][0] = str.as_bytes()[0];
            hex02[i][1] = str.as_bytes()[0];
        }

        let mut decimal = Vec::new();
        for i in 0..decimal_size {
            decimal.push(format!("{}", i));
        }

        NumberWriter {
            hex02: Arc::new(hex02),
            decimal: Arc::new(decimal),
        }
    }

    /// Generate a NumberWriter with default parameters
    pub(crate) fn create() -> NumberWriter {
        Self::with_decimal_size(NUMBER_WRITER_DEFAULT_MAX_DECIMAL)
    }

    /// Get the default NumberWriter
    pub fn new() -> NumberWriter {
        Default::default()
    }

    pub fn write_hex02(&self, mut writer: impl Write, value: u8) -> std::io::Result<()> {
        writer.write_all(&self.hex02[value as usize][..])
    }

    pub fn write_decimal(&self, mut writer: impl Write, value: usize) -> std::io::Result<()> {
        if value < self.decimal.len() {
            writer.write_all(self.decimal[value].as_bytes())
        } else {
            panic!("value is to large");
        }
    }
}

impl Default for NumberWriter {
    fn default() -> Self {
        NUMBER_WRITER.clone()
    }
}
