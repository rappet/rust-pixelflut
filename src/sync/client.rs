//! Contains the sync client for pixelflut.
use bufstream::BufStream;

use std::io::{self, BufRead, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::command::{Command, Response};
use crate::Result;
use crate::pixel::Pixel;
use crate::error::ErrorKind;

/// a Pixelflut Client connection
pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {
    /// connects to a Pixelflut host at address `addr`
    pub fn connect(addr: impl ToSocketAddrs) -> Result<Client> {
        let stream = TcpStream::connect(addr)?;
        Ok(Client {
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
    pub fn dimensions(&mut self) -> Result<(u32, u32)> {
        self.stream.write_fmt(format_args!("{}\n", Command::Size))?;
        self.stream.flush()?;
        let mut line = String::new();
        let n = self.stream.read_line(&mut line)?;
        if n > 0 {
            let response: Response = line[0..n - 1].parse()?;
            match response {
                Response::Size { w, h } => Ok((w, h)),
                Response::Error(err) => Err(ErrorKind::ServerError.into())
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
    /// [flush]: Self::flush
    pub fn set(&mut self, pixel: impl Into<Pixel>) -> Result<()> {
        self.stream
            .write_fmt(format_args!("{}\n", Command::Px(pixel.into())))?;
        Ok(())
    }

    /// Flushes the internal buffer to the server.
    pub fn flush(&mut self) -> Result<()> {
        self.stream.flush()?;
        Ok(())
    }
}
