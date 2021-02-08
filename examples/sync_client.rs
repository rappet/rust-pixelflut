extern crate pixelflut;

use pixelflut::sync::Client;
use pixelflut::Pixel;

use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<Error>> {
    let host: SocketAddr = std::env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:1337".to_string())
        .parse()?;
    let mut client = Client::connect(host)?;

    // get the screen size
    let (w, h) = client.size()?;
    println!("Size: {}x{}", w, h);

    // write a red line
    for i in 5..10 {
        client.set(Pixel::new((i, 6), (255, 0, 0)))?;
    }

    Ok(())
}
