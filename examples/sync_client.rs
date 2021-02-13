extern crate clap;
extern crate pixelflut;
extern crate tokio;

use clap::Clap;
use pixelflut::sync::PixelflutClient;
use std::net::SocketAddr;

#[derive(Clap)]
struct Opts {
    #[clap(default_value = "127.0.0.1:1337")]
    addr: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let addr: SocketAddr = opts.addr.parse()?;
    let mut client = PixelflutClient::connect(addr)?;

    let (width, height) = client.dimensions().await.unwrap();
    let (width, height) = (1920u32, 1080u32);
    println!("Size: {}x{}", width, height);

    for h in 0..height {
        for w in 0..width {
            client.set(w, h, (255, 255, 255))?;
        }
    }
    client.flush()?;

    Ok(())
}
