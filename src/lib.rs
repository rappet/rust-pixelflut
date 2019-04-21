#[cfg(feature = "sync")]
extern crate bufstream;
#[cfg(feature = "async")]
extern crate bytes;
#[cfg(feature = "async")]
extern crate futures;
#[cfg(feature = "async")]
extern crate tokio_io;
#[cfg(feature = "async")]
extern crate tokio_codec;
#[cfg(feature = "async")]
extern crate tokio_core;
#[cfg(feature = "image")]
extern crate image;

pub mod error;
pub mod pixel;
pub mod command;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "async")]
pub mod async;

pub use error::{Error, Result};
pub use pixel::{Pixel, Color, Coordinate};
pub use command::{Command, Response};
