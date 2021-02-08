use crate::command::{Command, Response};
use crate::error::ErrorKind;
use crate::{Pixel, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct PixelflutClient {
    stream: BufStream<TcpStream>,
}

impl PixelflutClient {
    /// Connect to a Pixelflut server.
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<PixelflutClient> {
        let stream = TcpStream::connect(addr).await?;
        Ok(PixelflutClient {
            stream: BufStream::new(stream),
        })
    }

    async fn write_command(&mut self, command: &Command) -> Result<()> {
        self.stream
            .write_all(command.to_string().as_bytes())
            .await?;
        self.stream.write_u8(b'\n').await?;
        Ok(())
    }

    async fn read_command(&mut self) -> Result<Response> {
        let mut line = String::new();
        let _bytes_read = self.stream.read_line(&mut line).await?;
        let response = line.trim_end().parse()?;
        Ok(response)
    }

    /// Writes a Pixel to the server.
    ///
    /// A buffered stream is used for sending.
    /// The pixel is only send if the buffer is full or [flush] is called.
    ///
    /// [flush]: Self::flush
    pub async fn write_pixel(&mut self, pixel: impl Into<Pixel>) -> Result<()> {
        self.write_command(&Command::Px(pixel.into())).await
    }

    /// Asks the server for the dimensions of the canvas.
    ///
    /// A `SIZE` command is send to the server.
    /// If the server replies with a `SIZE <width> <height>` packet,
    /// the dimensions will be returned.
    ///
    /// # Returns
    /// Ok((width, height)) on success
    pub async fn dimensions(&mut self) -> Result<(u32, u32)> {
        self.write_command(&Command::Size).await?;
        self.flush().await?;
        let response = self.read_command().await?;
        Ok(match response {
            Response::Size { w, h } => (w, h),
            _ => return Err(ErrorKind::State.into()),
        })
    }

    /// Flushes the internal buffer to the server.
    pub async fn flush(&mut self) -> Result<()> {
        self.stream.flush().await?;
        Ok(())
    }
}