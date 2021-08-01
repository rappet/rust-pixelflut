extern crate pixelflut;
extern crate tokio;

use pixelflut::async_tokio::PixelflutServerStream;

use tokio::net::TcpListener;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr: SocketAddr = "127.0.0.1:1337".parse()?;

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Got connection from {}", addr);
        let stream = PixelflutServerStream::new(stream, (800, 600));
        tokio::spawn(async {
            if let Err(err) = process(stream).await {
                eprintln!("error in client connection: {}", err);
            }
        })
        .await?;
    }
}

async fn process(mut stream: PixelflutServerStream) -> anyhow::Result<()> {
    while let Some(pixel) = stream.read_pixel().await? {
        println!("Got pixel {:?}", pixel);
    }
    println!("Connection closed");
    Ok(())
}
