//! [![crates-io]](https://crates.io/crates/pixelflut)
//! [![docs-rs]](https://docs.rs/pixelflut/)
//! [![license]](../LICENSE)
//!
//! [crates-io]: https://img.shields.io/crates/v/pixelflut?style=for-the-badge
//! [docs-rs]: https://img.shields.io/docsrs/pixelflut?style=for-the-badge
//! [license]: https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge
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
//!     pixelflut.set(1, 2, (255, 0, 0)).await.unwrap();
//!     pixelflut.flush();
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
#[macro_use]
extern crate lazy_static;

#[cfg(any(doc, feature = "tokio-rt"))]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-rt")))]
pub mod async_tokio;
mod command;
mod error;
mod pixel;
mod pixel_buffer;
#[cfg(any(doc, feature = "sync"))]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod sync;

pub use error::{PixelflutError, PixelflutResult};
pub use pixel::{Color, Coordinate, Pixel};
pub use pixel_buffer::PixelBuffer;
