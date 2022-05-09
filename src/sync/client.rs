//! Contains the sync client for pixelflut.
use bufstream::BufStream;

use std::io::{self, BufRead, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::command::{Command, Response};
use crate::error::PixelflutErrorKind;
use crate::pixel::Pixel;
use crate::{Color, PixelflutResult};

/// Sync Pixelflut client.
pub struct PixelflutClient {
    stream: BufStream<TcpStream>,
}

impl PixelflutClient {
    /// Connects to a Pixelflut host at address `addr`
    ///
    /// # Errors
    /// Failes if the TCP connection failes.
    pub fn connect(addr: impl ToSocketAddrs) -> PixelflutResult<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            stream: BufStream::new(stream),
        })
    }

    /// Asks the server for the dimensions of the canvas.
    ///
    /// A `SIZE` command is send to the server.
    /// If the server replies with a `SIZE <width> <height>` packet,
    /// the dimensions will be returned.
    ///
    /// # Returns
    /// Ok((width, height)) on success
    ///
    /// # Errors
    /// Failes if the response is malformed or the socket failes.
    pub fn dimensions(&mut self) -> PixelflutResult<(u32, u32)> {
        self.stream.write_fmt(format_args!("{}\n", Command::Size))?;
        self.stream.flush()?;
        let mut line = String::new();
        let n = self.stream.read_line(&mut line)?;
        if n > 0 {
            let response: Response = line[0..n - 1].parse()?;
            match response {
                Response::Size { w, h } => Ok((w, h)),
                Response::Error(_err) => Err(PixelflutErrorKind::ServerError.into()),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected size").into())
        }
    }

    /// Writes a Pixel to the server.
    ///
    /// A buffered stream is used for sending.
    /// The pixel is only send if the buffer is full or [flush] is called.
    ///
    /// # Errors
    /// Failing if the underling socket is failing.
    /// [flush]: `Self::flush`
    pub fn set(&mut self, x: u32, y: u32, color: impl Into<Color>) -> PixelflutResult<()> {
        let pixel = Pixel::new((x, y).into(), color.into());
        self.stream
            .write_fmt(format_args!("{}\n", Command::Px(pixel)))?;
        Ok(())
    }

    /// Flushes the internal buffer to the server.
    ///
    /// # Errors
    /// Failing if the underlying writer files.
    pub fn flush(&mut self) -> PixelflutResult<()> {
        self.stream.flush()?;
        Ok(())
    }
}
