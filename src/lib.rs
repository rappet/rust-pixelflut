#![doc = include_str!("../README.md")]

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
