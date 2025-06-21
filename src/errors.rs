use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ClientError(&'static str),
    IoError(std::io::Error),
    EncodeError(&'static str),
    DecodeError(&'static str),
}

pub type DiameterResult<T> = Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::ClientError(msg) => write!(f, "{}", msg),
            Error::IoError(e) => write!(f, "{}", e),
            Error::EncodeError(msg) => write!(f, "{}", msg),
            Error::DecodeError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

// io error
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}
