use crate::modeling::diameter::DiameterMessage;
use std::io::{Error, Write};
use std::net::{Shutdown, TcpStream};
use crate::transport::errors::DiameterResult;
use crate::transport::errors::Error::ClientError;

pub struct DiameterClient {
    address: &'static str,
    stream: Option<TcpStream>,
}

impl DiameterClient {
    pub fn new(address: &'static str) -> Self {
        Self {
            address,
            stream: None,
        }
    }

    pub fn connect(&mut self) -> DiameterResult<()> {
        let stream = TcpStream::connect(self.address)?;
        self.stream = Some(stream);
        Ok(())
    }
    
    pub fn close(&mut self) -> DiameterResult<()> {
        if let Some(ref mut stream) = self.stream {
            stream.shutdown(Shutdown::Both)?;
            Ok(())
        } else {
            Err(ClientError("Connection not established yet!"))
        }
    }

    pub fn write(&mut self, message: &DiameterMessage) -> DiameterResult<()> {
        if let Some(ref mut stream) = self.stream {
            stream.write(message.encode().as_slice()).unwrap();
            Ok(())
        } else {
            Err(ClientError("Connection not established yet!"))
        }
    }
}
