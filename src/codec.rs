use command::Command;
use error::{Error, ErrorKind, Result};

use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

pub struct PixelflutCodec;

impl Decoder for PixelflutCodec {
    type Item = Command;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Command>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            Ok(Some(str::from_utf8(&line)?.parse()?))
        } else if buf.len() > 34 { // longest possible command
            Err(ErrorKind::LineTooLong.into())
        } else {
            Ok(None)
        }
    }
}

impl Encoder for PixelflutCodec {
    type Item = Command;
    type Error = Error;

    fn encode(&mut self, command: Command, buf: &mut BytesMut) -> Result<()> {
        buf.extend(format!("{}\n", command).as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use bytes::BytesMut;
        use pixel::Pixel;
        use command::Command;
        use tokio_io::codec::Decoder;
        use PixelflutCodec;

        let pxcommand = Command::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        let mut buf = BytesMut::from("PX 45 67 112255\n");
        assert_eq!(PixelflutCodec.decode(&mut buf).unwrap(), Some(pxcommand));
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn encode() {
        use bytes::BytesMut;
        use pixel::Pixel;
        use command::Command;
        use tokio_io::codec::Encoder;
        use PixelflutCodec;

        let pxcommand = Command::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        let mut buf = BytesMut::new();
        PixelflutCodec.encode(pxcommand, &mut buf).unwrap();
        assert_eq!(&buf, "PX 45 67 112255\n");

    }
}
