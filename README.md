# rust-pixelflut, a pixelflut client and server library

[![Build status](https://api.travis-ci.org/rappet/rust-pixelflut.png)](https://travis-ci.org/rappet/rust-pixelflut)


rust-pixelflut is a sync and async implementation of the pixelflut protocol for Rust.

# Usage
Import in rust project:

```toml
[dependencies]                                                                  
pixelflut = { version = "0.1.0", features = ["sync"]}
```

## Features
To enable the async library with tokio use the "async" feature.

To enable support for the image crate use the "image" feature.

# Example

```rust
extern crate pixelflut;

use pixelflut::sync::Client;
use pixelflut::Pixel;

use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<Error>> {
    let host: SocketAddr = std::env::args().nth(1).unwrap_or("127.0.0.1:1337".to_string()).parse()?;
    let mut client = Client::connect(host)?;
    
    // get the screen size
    let (w, h) = client.size()?;
    println!("Size: {}x{}", w, h);

    // write a red line
    for i in 5..10 {
        client.set(Pixel::new((i,6), (255,0,0)))?;
    }

    Ok(())
}
```

## TODO:
- [ ] Types and formating are usable, but should be enhanced.
- [X] sync client
- [x] sync server
- [ ] async client
- [ ] async server

## License

MIT/Apache 2

