//! Contains the sync server for pixelflut.
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

use command::{Command, Response};
use error::Result;

/// The `PixelflutStream` struct parses Pixelflut command
/// from any TcpStream.TcpStream
///
/// # Examples
///
/// ```no_run
/// use pixelflut::sync::PixelflutStream;
/// use pixelflut::{Command, Result, Response};
///
/// use std::net::TcpStream;
///
/// fn handle_client(stream: TcpStream) -> Result<()> {
///     let mut stream = PixelflutStream::new(stream);
///     
///     while let Ok(command) = stream.read() {
///         match command {
///             Command::Px(p) => println!("{}", p),
///             Command::Size => {
///                 let response = Response::Size{ w: 800, h: 600 };
///                 stream.send_response(&response)?
///             },
///         }
///     }
///     
///     Ok(())
/// }
/// ```
pub struct PixelflutStream {
    reader: BufReader<TcpStream>,
}

impl PixelflutStream {
    /// Creates a new `PixelflutStream` from a `TcpStream`.
    pub fn new(stream: TcpStream) -> PixelflutStream {
        PixelflutStream {
            reader: BufReader::new(stream),
        }
    }

    /// Sends a `Response` to the client.
    pub fn send_response(&mut self, response: &Response) -> Result<()> {
        self.reader
            .get_mut()
            .write_fmt(format_args!("{}\n", response))?;
        Ok(())
    }

    /// Reads a `Command` from the stream.
    pub fn read(&mut self) -> Result<Command> {
        let mut line = String::new();
        let n = self.reader.read_line(&mut line)?;
        if n > 0 {
            Ok(line[0..line.len()].parse()?)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of stream").into())
        }
    }
}
