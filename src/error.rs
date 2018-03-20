use std::str::Utf8Error;
use std::io::Error as IoError;
use std::num::ParseIntError;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(IoError);
        ParseInt(ParseIntError);
        Utf8(Utf8Error);
    }

    errors {
        ColorLength { description("Invalid color length") }
        InvalidCommand { description("Invalid command") }
        WrongNumberOfArguments { description("Wrong number of arguments") }
        LineTooLong { description("Line is longer than the lingest possible command") }
    }
}
