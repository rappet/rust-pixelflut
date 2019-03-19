use command::{Command, Response};
use error::{Error, ErrorKind, Result};

use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

pub struct PixelflutServerCodec;

impl Decoder for PixelflutServerCodec {
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

impl Encoder for PixelflutServerCodec {
    type Item = Response;
    type Error = Error;

    fn encode(&mut self, command: Response, buf: &mut BytesMut) -> Result<()> {
        buf.extend(format!("{}\n", command).as_bytes());
        Ok(())
    }
}

pub struct PixelflutClientCodec;

impl Decoder for PixelflutClientCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Response>> {
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

impl Encoder for PixelflutClientCodec {
    type Item = Command;
    type Error = Error;

    fn encode(&mut self, command: Command, buf: &mut BytesMut) -> Result<()> {
        buf.extend(format!("{}\n", command).as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use pixel::Pixel;

    #[test]
    fn decode_server() {
        use command::Command;
        use tokio_io::codec::Decoder;
        use PixelflutServerCodec;

        let pxcommand = Command::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        let mut buf = BytesMut::from("PX 45 67 112255\n");
        assert_eq!(PixelflutServerCodec.decode(&mut buf).unwrap(), Some(pxcommand));
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn encode_server() {
        use command::Command;
        use tokio_io::codec::Encoder;
        use PixelflutClientCodec;

        let pxcommand = Command::Px(Pixel::new((45, 67), (0x11, 0x22, 0x55)));

        let mut buf = BytesMut::new();
        PixelflutClientCodec.encode(pxcommand, &mut buf).unwrap();
        assert_eq!(&buf, "PX 45 67 112255\n");

    }

    #[test]
    fn decode_client() {
        use command::Response;
        use tokio_io::codec::Decoder;
        use PixelflutClientCodec;

        let sizecommand = Response::Size { w: 12, h: 34 };

        let mut buf = BytesMut::from("SIZE 12 34\n");
        assert_eq!(PixelflutClientCodec.decode(&mut buf).unwrap(), Some(sizecommand));
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn encode_client() {
        use command::Response;
        use tokio_io::codec::Encoder;
        use PixelflutServerCodec;

        let sizecommand = Response::Size { w: 12, h: 34 };

        let mut buf = BytesMut::new();
        PixelflutServerCodec.encode(sizecommand, &mut buf).unwrap();
        assert_eq!(&buf, "SIZE 12 34\n");

    }
}
