pub struct PixelflutCodec;

impl Decoder for PixelflutCodec {
    type Item = Command;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Command>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            Ok(str::from_utf8(&line)?.parse()?)
        } else if buf.len() > 34 { // longest possible command
            Err(Error::LineTooLong)
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
