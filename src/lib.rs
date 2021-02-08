//! [![crates-io]](https://crates.io/crates/pixelflut) [![docs-rs]](https://docs.rs/pixelflut/)
//!
//! [crates-io]: https://img.shields.io/crates/v/pixelflut?style=for-the-badge
//! [docs-rs]: https://img.shields.io/docsrs/pixelflut?style=for-the-badge
//!
//! ---
//!
//! An implementation of the [Pixelflut] protocol.
//!
//! # Example
//!
//! ```no_run
//! // pixelflut = "0.2"
//! extern crate pixelflut;
//! extern crate tokio;
//! use pixelflut::async_tokio::PixelflutClient;
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() {
//!     let addr: SocketAddr = "localhost:1337".parse().unwrap();
//!     let mut pixelflut = PixelflutClient::connect(addr).await.unwrap();
//!     println!("{:?}", pixelflut.dimensions().await.unwrap());
//!     pixelflut.set(((1, 2), (255, 0, 0))).await.unwrap();
//! }
//! ```
//!
//! [Pixelflut]: https://cccgoe.de/wiki/Pixelflut

extern crate bstr;
#[cfg(feature = "sync")]
extern crate bufstream;
extern crate bytes;
#[cfg(feature = "image")]
extern crate image;
extern crate memchr;
#[cfg(feature = "tokio-rt")]
extern crate tokio;

#[cfg(feature = "tokio-rt")]
pub mod async_tokio;
mod command;
mod error;
mod pixel;
#[cfg(feature = "sync")]
pub mod sync;

pub use error::{Error, Result};
pub use pixel::{Color, Coordinate, Pixel};
