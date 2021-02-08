//! Contains the sync client for pixelflut.
pub mod client;
pub mod server;

pub use self::client::Client;
pub use self::server::PixelflutStream;
