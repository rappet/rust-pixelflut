use bytes::BytesMut;
use tokio::net::TcpStream;

use crate::command::{Command, Response};
use crate::error::PixelflutErrorKind;
use crate::pixel::MAX_FORMATTED_PIXEL_SIZE_NEWLINE;
use crate::{Pixel, PixelflutResult};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub static SERVER_READ_BUFFER_DEFAULT_CAPACITY: usize = 2 << 16;

/// Async Pixelflut server connection.
pub struct PixelflutServerStream {
    stream: TcpStream,
    read_buf: BytesMut,
    dimensions: (u32, u32),
}

impl PixelflutServerStream {
    pub fn new(stream: TcpStream, dimensions: (u32, u32)) -> Self {
        Self::with_capacity(stream, dimensions, SERVER_READ_BUFFER_DEFAULT_CAPACITY)
    }

    pub fn with_capacity(stream: TcpStream, dimensions: (u32, u32), capacity: usize) -> Self {
        Self {
            stream,
            read_buf: BytesMut::with_capacity(capacity),
            dimensions,
        }
    }

    async fn read_command(&mut self) -> PixelflutResult<Option<Command>> {
        loop {
            if let Some(pos) = memchr::memchr(b'\n', self.read_buf.as_ref()) {
                let slice = &self.read_buf.as_ref()[0..pos];
                let command = match Command::parse_byte_slice(slice) {
                    Ok(command) => command,
                    Err(err) => {
                        self.send_response(&Response::Error(err.to_string().into()))
                            .await?;
                        return Err(err);
                    }
                };
                let _ = self.read_buf.split_to(pos + 1);
                return Ok(Some(command));
            } else if self.read_buf.len() > MAX_FORMATTED_PIXEL_SIZE_NEWLINE {
                return Err(PixelflutErrorKind::Io.with_description("line is to long"));
            } else if self.stream.read_buf(&mut self.read_buf).await? == 0 {
                return if self.read_buf.is_empty() {
                    Ok(None)
                } else {
                    Err(PixelflutErrorKind::Io.with_description("Unexpected end of stream"))
                };
            }
        }
    }

    async fn send_response(&mut self, response: &Response) -> PixelflutResult<()> {
        let buf = format!("{}\n", response);
        self.stream.write_all(buf.as_bytes()).await?;
        Ok(())
    }

    /// Read the next pixel from the client.
    ///
    /// This will automatically respond to "SIZE" requests.
    /// Returns `None` if the stream got closed.
    ///
    /// # Errors
    /// Failing if the underlying socket is failing or the client is sending
    /// a malformed command.
    pub async fn read_pixel(&mut self) -> PixelflutResult<Option<Pixel>> {
        loop {
            match self.read_command().await? {
                Some(Command::Px(pixel)) => return Ok(Some(pixel)),
                Some(Command::Size) => {
                    self.send_response(&Response::Size {
                        w: self.dimensions.0,
                        h: self.dimensions.1,
                    })
                    .await?;
                }
                None => return Ok(None),
            }
        }
    }
}
