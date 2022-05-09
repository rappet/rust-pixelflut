//! Contains the sync server for pixelflut.
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::command::{Command, Response};
use crate::error::PixelflutResult;
use crate::Pixel;

/// Sync Pixelflut server connection.
///
/// # Examples
///
/// ```no_run
/// use pixelflut::sync::PixelflutServerStream;
/// use pixelflut::PixelflutResult;
///
/// use std::net::TcpStream;
///
/// fn handle_client(stream: TcpStream) -> PixelflutResult<()> {
///     let mut stream = PixelflutServerStream::new(stream, (800, 600));
///     
///     while let Some(pixel) = stream.read_pixel()? {
///         println!("{:?}", pixel);
///     }
///     
///     Ok(())
/// }
/// ```
pub struct PixelflutServerStream {
    reader: BufReader<TcpStream>,
    dimensions: (u32, u32),
}

impl PixelflutServerStream {
    /// Creates a new `PixelflutStream` from a `TcpStream`.
    #[must_use]
    pub fn new(stream: TcpStream, dimensions: (u32, u32)) -> Self {
        Self {
            reader: BufReader::new(stream),
            dimensions,
        }
    }

    /// Sends a `Response` to the client.
    fn send_response(&mut self, response: &Response) -> PixelflutResult<()> {
        self.reader
            .get_mut()
            .write_fmt(format_args!("{}\n", response))?;
        Ok(())
    }

    /// Reads a `Command` from the stream.
    fn read_command(&mut self) -> PixelflutResult<Option<Command>> {
        let mut line = String::new();
        let _len = match self.reader.read_line(&mut line) {
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        Ok(Some(line[0..line.len()].parse()?))
    }

    /// Read the next pixel from the client.
    ///
    /// This will automatically respond to "SIZE" requests.
    /// Returns `None` if the stream got closed.
    ///
    /// # Errors
    /// Failing if the underlying socket is failing or the client is sending
    /// a malformed command.
    pub fn read_pixel(&mut self) -> PixelflutResult<Option<Pixel>> {
        loop {
            match self.read_command()? {
                Some(Command::Px(pixel)) => return Ok(Some(pixel)),
                Some(Command::Size) => self.send_response(&Response::Size {
                    w: self.dimensions.0,
                    h: self.dimensions.1,
                })?,
                None => return Ok(None),
            }
        }
    }
}
