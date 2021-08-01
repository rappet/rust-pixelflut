extern crate pixelflut;

use pixelflut::sync::PixelflutServerStream;

use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> pixelflut::PixelflutResult<()> {
    let mut stream = PixelflutServerStream::new(stream, (800, 600));

    while let Some(pixel) = stream.read_pixel()? {
        println!("{}", pixel);
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let host: SocketAddr = "127.0.0.1:1337".parse()?;
    let listener = TcpListener::bind(host)?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
