extern crate pixelflut;
extern crate tokio;

use pixelflut::async_tokio::PixelflutClient;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();
    let mut client = PixelflutClient::connect(addr).await.unwrap();

    let (width, height) = client.dimensions().await.unwrap();
    println!("Size: {}x{}", width, height);

    for h in 0..height {
        for w in 0..width {
            client.set(((w, h), (255, 255, 255))).await.unwrap();
        }
    }
    client.flush().await.unwrap();
}
