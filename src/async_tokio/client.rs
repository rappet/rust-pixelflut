use crate::command::{Command, Response};
use crate::error::ErrorKind;
use crate::{Color, Pixel, PixelBuffer, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct PixelflutClient {
    stream: BufReader<TcpStream>,
    write_buf: PixelBuffer,
}

impl PixelflutClient {
    /// Connect to a Pixelflut server.
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<PixelflutClient> {
        let stream = TcpStream::connect(addr).await?;
        Ok(PixelflutClient {
            stream: BufReader::new(stream),
            write_buf: PixelBuffer::new(),
        })
    }

    async fn write_command(&mut self, command: &Command) -> Result<()> {
        self.flush().await?;
        self.stream
            .write_all(format!("{}\n", command).as_bytes())
            .await?;
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
    pub async fn set(&mut self, x: u32, y: u32, color: impl Into<Color>) -> Result<()> {
        let pixel = Pixel::new((x, y).into(), color.into());
        if self.write_buf.is_capacity_reached() {
            self.flush().await?;
        }
        self.write_buf.write_pixel(&pixel);
        Ok(())
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
        let response = self.read_command().await?;
        Ok(match response {
            Response::Size { w, h } => (w, h),
            _ => return Err(ErrorKind::State.into()),
        })
    }

    pub async fn write_buffer(&mut self, buffer: &PixelBuffer) -> Result<()> {
        self.flush();
        self.stream.write_all(buffer.as_slice()).await?;
        Ok(())
    }

    /// Flushes the internal buffer to the server.
    pub async fn flush(&mut self) -> Result<()> {
        if !self.write_buf.is_empty() {
            self.stream.write_all(self.write_buf.as_slice()).await?;
        }
        self.write_buf.clear();
        Ok(())
    }
}
