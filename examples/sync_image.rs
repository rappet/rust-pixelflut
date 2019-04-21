extern crate pixelflut;
extern crate image;

use pixelflut::sync::Client;
use pixelflut::Pixel;

use image::GenericImageView;

use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<Error>> {
    let filename: String = match std::env::args().nth(1) {
        Some(name) => name,
        None => {
            println!("Usage: {} <filename> [host]", std::env::args().next().unwrap());
            return Ok(())
        }
    };
    let host: SocketAddr = std::env::args().nth(2).unwrap_or("127.0.0.1:1337".to_string()).parse()?;

    let image = image::open(filename)?;

    let mut client = Client::connect(host)?;
    
    // get the screen size
    let (w, h) = client.size()?;
    println!("Size: {}x{}", w, h);

    // draw the image
    for (x, y, color) in image.pixels() {
        client.set(Pixel::new((x,y), color))?;
    }

    Ok(())
}