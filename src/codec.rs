use std::str;
use std::io;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use command::Command;
use error::{Result};

pub struct PixelflutCodec;

impl Decoder for PixelflutCodec {
    type Item = Command;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Command>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => Ok({
                    let command: Result<Command> = s.parse();
                    Some(command.map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "could not parse")
                    })?)
                }),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData,
                                           "invalid UTF-8")),
            }

        } else if buf.len() > 34 { // longest possible command
            Err(io::Error::new(io::ErrorKind::InvalidData, "line too long"))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for PixelflutCodec {
    type Item = Command;
    type Error = io::Error;

    fn encode(&mut self, command: Command, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(format!("{}\n", command).as_bytes());
        Ok(())
    }
}
