extern crate futures;
extern crate tokio_core;
extern crate pixelflut;

use std::net::ToSocketAddrs;
use std::str::FromStr;
use tokio_core::reactor::Core;
use futures::future::Future;
use futures::Sink;
use futures::Stream;
use futures::stream;

use pixelflut::Client;
use pixelflut::Command;
use pixelflut::pixel::{Pixel, Coordinate, Color};

fn main() {
    let mut ev = Core::new().unwrap();
    let handle = ev.handle();

    let server = "127.0.0.1:12345";
    let addr = server.to_socket_addrs().unwrap().next().unwrap();

    let client = Client::new(addr)
        .connect(&handle)
        .and_then(|pf| {
            pf.send(
                Command::Px(Pixel::new(
                        Coordinate::new(12, 34), 
                        Color::rgb(12, 34, 56)))
            ).and_then(|pf| {
                pf.for_each(|command| {
                    println!("{:?}", command);
                    Ok(())
                }) 
            })

        });


    ev.run(client).unwrap();
}
