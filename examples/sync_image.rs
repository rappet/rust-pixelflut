extern crate image;
extern crate pixelflut;

use pixelflut::sync::PixelflutClient;

use image::GenericImageView;

use std::net::SocketAddr;

fn main() -> anyhow::Result<()> {
    let filename: String = match std::env::args().nth(1) {
        Some(name) => name,
        None => {
            println!(
                "Usage: {} <filename> [host]",
                std::env::args().next().unwrap()
            );
            return Ok(());
        }
    };
    let host: SocketAddr = std::env::args()
        .nth(2)
        .unwrap_or("127.0.0.1:1337".to_string())
        .parse()?;

    let image = image::open(filename)?;

    let mut client = PixelflutClient::connect(host)?;

    // get the screen size
    let (w, h) = client.dimensions()?;
    println!("Size: {}x{}", w, h);

    // draw the image
    for (x, y, color) in image.pixels() {
        client.set(x, y, color)?;
    }

    Ok(())
}
