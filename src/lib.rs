//! # pixelflut
//!
//! pixelflut is a sync and async_tokio pixelflut implementation for Rust.

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
