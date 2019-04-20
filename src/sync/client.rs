//! Contains the sync client for pixelflut.
use bufstream::BufStream;

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{self, Write, BufRead};

use command::{Command, Response};
use pixel::Pixel;
use error::Result;

/// a Pixelflut Client connection
pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {

    /// connects to a Pixelflut host at address `addr`
    pub fn connect<A: ToSocketAddrs> (addr: A) -> Result<Client> {
        let stream = TcpStream::connect(addr)?;
        Ok(Client {
            stream: BufStream::new(stream)
        })
    }

    /// asks the server for the screen size
    pub fn size(&mut self) -> Result<(u32, u32)> {
        self.stream.write_fmt(format_args!("{}\n", Command::Size))?;
        self.stream.flush()?;
        let mut line = String::new();
        let n = self.stream.read_line(&mut line)?;
        if n > 0 {
            let response: Response = line[0..n-1].parse()?;
            match response {
                Response::Size { w, h } => Ok((w, h)),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected size").into())
        }
    }

    /// Sends a PX command to the server.
    /// Pixels will be put on a internal buffer and flushed periodically
    /// to the server. You can flush the buffer manualy with `flush`.
    pub fn set<P: Into<Pixel>>(&mut self, pixel: P) -> Result<()> {
        self.stream.write_fmt(format_args!("{}\n", Command::Px(pixel.into())))?;
        Ok(())
    }

    /// flushes the pixels to the socket
    pub fn flush(&mut self) -> Result<()> {
        self.stream.flush()?;
        Ok(())
    }

}