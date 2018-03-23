extern crate futures;
extern crate tokio_core;
extern crate pixelflut;
#[macro_use]
extern crate error_chain;

use std::env;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;
use futures::future::Future;
use futures::Sink;
use futures::Stream;

use pixelflut::Client;
use pixelflut::ServerCommand;
use pixelflut::pixel::{Pixel, Coordinate, Color};
use pixelflut::error;

quick_main!(run);

fn run() -> error::Result<()> {
    let mut ev = Core::new()?;
    let handle = ev.handle();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("127.0.0.1:12345"))
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| error::Error::from("Could not convert to SocketAddr"))?;

    let client = Client::new(addr)
        .connect(&handle)
        .and_then(|pf| {
            pf.send(
                ServerCommand::Px(Pixel::new(
                        Coordinate::new(12, 34), 
                        Color::rgb(12, 34, 56)))
                ).and_then(|pf| {
                pf.for_each(|command| {
                    println!("{:?}", command);
                    Ok(())
                }) 
            })

        });


    ev.run(client)?;
    Ok(())
}
