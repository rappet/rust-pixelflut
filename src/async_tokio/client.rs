use crate::error::ErrorKind;
use crate::{Command, Pixel, Response, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct PixelflutClient {
    stream: BufStream<TcpStream>,
}

impl PixelflutClient {
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

    pub async fn write_pixel(&mut self, pixel: impl Into<Pixel>) -> Result<()> {
        self.write_command(&Command::Px(pixel.into())).await
    }

    pub async fn dimensions(&mut self) -> Result<(u32, u32)> {
        self.write_command(&Command::Size).await?;
        self.flush().await?;
        let response = self.read_command().await?;
        Ok(match response {
            Response::Size { w, h } => (w, h),
            _ => return Err(ErrorKind::State.into()),
        })
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.stream.flush().await?;
        Ok(())
    }
}
