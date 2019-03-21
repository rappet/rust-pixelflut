//! Contains the sync client for pixelflut.
use std::net::{ToSocketAddrs, TcpStream};
use std::io::{self, Write, BufRead, BufReader};

use command::{Command, Response};
use pixel::Pixel;
use error::{Result};

/// a Pixelflut Client connection
pub struct Client {
    stream: TcpStream,
}

impl Client {

    /// connects to a Pixelflut host at address `addr`
    pub fn connect<A: ToSocketAddrs> (addr: A) -> Result<Client> {
        let stream = TcpStream::connect(addr)?;
        Ok(Client {
            stream
        })
    }

    /// asks the server for the screen size
    pub fn size(&mut self) -> Result<(u32, u32)> {
        self.stream.write_fmt(format_args!("{}\n", Command::Size))?;
        let mut reader = BufReader::new(&mut self.stream);
        let mut line = String::new();
        if reader.read_line(&mut line)? > 0 {
            let response: Response = line[0..line.len()-1].parse()?;
            match response {
                Response::Size { w, h } => Ok((w, h)),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "expected size").into())
        }
    }

    /// sends a PX command to the server
    pub fn set<P: Into<Pixel>>(&mut self, pixel: P) -> Result<()> {
        self.stream.write_fmt(format_args!("{}\n", Command::Px(pixel.into())))?;
        Ok(())
    }

}