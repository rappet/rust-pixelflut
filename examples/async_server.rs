extern crate pixelflut;
extern crate futures;
extern crate tokio;
//extern crate tokio_io;
extern crate tokio_codec;

use pixelflut::codec::PixelflutServerCodec;

use tokio_codec::Decoder;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<Error>> {
    let addr: SocketAddr = "127.0.0.1:1234".parse()?;
    
    let listener = TcpListener::bind(&addr)?;
    println!("Listening in: {}", addr);

    tokio::run({
        listener
            .incoming()
            .map_err(|e| println!("failed to accept socket; error = {:?}", e))
            .for_each(|socket| {
                process(socket);
                Ok(())
            })
    });
    Ok(())
}

fn process(socket: TcpStream) {
    let peer_addr = socket.peer_addr().unwrap();
    let (_tx, rx) =
        PixelflutServerCodec.framed(socket)
        .split();
    
    let task = rx
        .map_err(move |e| println!("{}: {}", peer_addr, e))
        .for_each(|req| {
            println!("{:?}", req);
            Ok(())
        });

    tokio::spawn(task);
}