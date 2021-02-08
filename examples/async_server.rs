extern crate pixelflut;
extern crate tokio;

use pixelflut::async_tokio::PixelflutServerStream;

use tokio::net::TcpListener;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:1234".parse().unwrap();

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("Got connection from {}", addr);
        let stream = PixelflutServerStream::new(stream, (800, 600));
        tokio::spawn(async { process(stream).await }).await.unwrap();
    }
}

async fn process(mut stream: PixelflutServerStream) {
    while let Some(pixel) = stream.read_pixel().await.unwrap() {
        println!("Got pixel {:?}", pixel);
    }
    println!("Foo");
}
