//! Contains the sync client for pixelflut.
pub mod client;
pub mod server;

pub use self::client::PixelflutClient;
pub use self::server::PixelflutServerStream;
