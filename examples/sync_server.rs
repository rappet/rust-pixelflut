extern crate pixelflut;

use pixelflut::sync::PixelflutStream;
use pixelflut::{Command, Response};

use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> pixelflut::Result<()> {
    let mut stream = PixelflutStream::new(stream);

    while let Ok(command) = stream.read() {
        match command {
            // The client sends a pixel
            Command::Px(p) => println!("{}", p),
            // The client asks for the screen size
            Command::Size => {
                // respond with the screen size
                let response = Response::Size { w: 800, h: 600 };
                stream.send_response(&response)?
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let host: SocketAddr = "127.0.0.1:1234".parse()?;
    let listener = TcpListener::bind(host)?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
