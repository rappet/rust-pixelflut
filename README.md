# rust-pixelflut, a pixelflut client and server library

[![crates.io](https://img.shields.io/crates/v/pixelflut.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/pixelflut)
[![Released API docs](https://img.shields.io/badge/docs.rs-pixelflut-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/pixelflut)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](./LICENSE)
[![Github](https://img.shields.io/badge/github-rappet/rust--pixelflut-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/rappet/rust-pixelflut)

A sync and async implementation of the [Pixelflut] protocol for Rust.

[Pixelflut]: https://cccgoe.de/wiki/Pixelflut

# Usage
Import in rust project:

```toml
[dependencies]                                                                  
pixelflut = "0.2.0-alpha.2"
```

## Feature flags

- `image`: Enable support for color types used in the [`image`] crate
- `tokio-rt`: Enable support for the async client/server

[`image`]: https://docs.rs/image/

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
    pixelflut.flush().await.unwrap();
}
```

## License

This project is licensed under the [MIT license](https://github.com/rappet/rust-pixelflut/blob/master/LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rust-pixelflut by you,
shall be licensed as MIT, without any additional terms or conditions.