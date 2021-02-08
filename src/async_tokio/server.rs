use bytes::BytesMut;
use tokio::net::TcpStream;

use crate::command::{Command, MAX_COMMAND_LENGTH, Response};
use crate::error::ErrorKind;
use crate::{Result, Pixel};
use bstr::ByteSlice;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub static SERVER_READ_BUFFER_DEFAULT_CAPACITY: usize = 2 << 16;

pub struct PixelflutServerStream {
    stream: TcpStream,
    read_buf: BytesMut,
    dimensions: (u32, u32),
    capacity: usize,
}

impl PixelflutServerStream {
    pub fn new(stream: TcpStream, dimensions: (u32, u32)) -> PixelflutServerStream {
        PixelflutServerStream::with_capacity(
            stream,
            dimensions,
            SERVER_READ_BUFFER_DEFAULT_CAPACITY,
        )
    }

    pub fn with_capacity(
        stream: TcpStream,
        dimensions: (u32, u32),
        capacity: usize,
    ) -> PixelflutServerStream {
        PixelflutServerStream {
            stream,
            read_buf: BytesMut::with_capacity(capacity),
            dimensions,
            capacity,
        }
    }

    async fn read_command(&mut self) -> Result<Option<Command>> {
        loop {
            if let Some(pos) = memchr::memchr(b'\n', self.read_buf.as_ref()) {
                let slice = &self.read_buf.as_ref()[0..pos];
                let command = Command::from_str(slice.to_str()?)?;
                let _ = self.read_buf.split_to(pos + 1);
                return Ok(Some(command));
            } else if self.read_buf.len() > MAX_COMMAND_LENGTH {
                return Err(ErrorKind::Io.with_description("line is to long"));
            } else {
                if self.stream.read_buf(&mut self.read_buf).await? == 0 {
                    return if self.read_buf.is_empty() {
                        Ok(None)
                    } else {
                        Err(ErrorKind::Io.with_description("Unexpected end of stream"))
                    };
                }
            }
        }
    }

    async fn send_response(&mut self, response: &Response) -> Result<()> {
        let buf = format!("{}\n", response);
        self.stream.write_all(buf.as_bytes()).await?;
        Ok(())
    }

    pub async fn read_pixel(&mut self) -> Result<Option<Pixel>> {
        loop {
            match self.read_command().await? {
                Some(Command::Px(pixel)) => return Ok(Some(pixel)),
                Some(Command::Size) => {
                    self.send_response(&Response::Size {
                        w: self.dimensions.0,
                        h: self.dimensions.1,
                    }).await?
                },
                None => return Ok(None)
            }
        }
    }
}
