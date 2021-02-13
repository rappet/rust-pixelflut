# rust-pixelflut, a pixelflut client and server library

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/pixelflut?style=for-the-badge)](https://crates.io/crates/pixelflut)
[![Released API docs](https://img.shields.io/docsrs/pixelflut?style=for-the-badge)](https://docs.rs/pixelflut)


rust-pixelflut is a sync and async implementation of the pixelflut protocol for Rust.

# Usage
Import in rust project:

```toml
[dependencies]                                                                  
pixelflut = "0.2.0-alpha.2"
```

## Features
To enable support for the image crate use the "image" feature.

# Performance

The async client archived >450MByte/s to localhost on an Apple m1.
The server code needs improvement.

If you want to send data faster, spawn multiple clients or use the internal
`PixelBuffer` if you want to send allways the same data.

# Example

```rust
extern crate pixelflut;
extern crate tokio;
use pixelflut::async_tokio::PixelflutClient;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "localhost:1337".parse().unwrap();
    let mut pixelflut = PixelflutClient::connect(addr).await.unwrap();
    println!("{:?}", pixelflut.dimensions().await.unwrap());
    pixelflut.set(1, 2, (255, 0, 0)).await.unwrap();
    pixelflut.flush();
}
```

## License

MIT

