//! # Diameter Protocol Message
//!
//! This crate provides functionalities for handling Diameter protocol messages as defined in RFC 6733.
//!
//! ## Raw Packet Format
//! The diagram below illustrates the raw packet format for the Diameter header:
//! ```text
//!   0                   1                   2                   3
//!   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |    Version    |                 Message Length                |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  | command flags |                  Command-Code                 |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                         Application-ID                        |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                      Hop-by-Hop Identifier                    |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                      End-to-End Identifier                    |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                              AVPs                             |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                              ...                              |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!
//!  Command Flags:
//!    0 1 2 3 4 5 6 7
//!   +-+-+-+-+-+-+-+-+  R(equest), P(roxyable), E(rror)
//!   |R P E T r r r r|  T(Potentially re-transmitted message), r(eserved)
//!   +-+-+-+-+-+-+-+-+
//! ```

use super::command_codes::CommandCode;
use crate::errors::DiameterResult;
use crate::modeling::avp::avp::Avp;
use std::io::Write;

#[derive(Debug)]
pub struct DiameterMessage {
    header: DiameterHeader,
    avps: Vec<Avp>,
}

#[derive(Debug)]
pub struct DiameterHeader {
    version: u8,
    message_length: u32, // 24 bits
    command_flags: CommandFlags,
    command_code: &'static CommandCode, // 24 bits
    application_id: ApplicationId,
    hop_by_hop: u32,
    end_to_end: u32,
}

#[derive(Debug)]
pub enum CommandFlags {
    REQUEST,
    PROXYABLE,
    ERROR,
    RETRANSMIT,
}

impl CommandFlags {
    fn value(&self) -> u8 {
        match self {
            CommandFlags::REQUEST => 0x80,
            CommandFlags::PROXYABLE => 0x40,
            CommandFlags::ERROR => 0x20,
            CommandFlags::RETRANSMIT => 0x10,
        }
    }
}

#[derive(Debug)]
pub enum ApplicationId {
    Gx,
    Gy,
}

impl ApplicationId {
    fn value(&self) -> u32 {
        match self {
            ApplicationId::Gx => 16777238,
            ApplicationId::Gy => 4,
        }
    }
}

impl DiameterMessage {
    pub fn new(
        command_flags: CommandFlags,
        command_code: &'static CommandCode,
        application_id: ApplicationId,
        hop_by_hop: u32,
        end_to_end: u32,
    ) -> Self {
        Self {
            header: DiameterHeader {
                version: 1,
                message_length: 32,
                command_flags,
                command_code,
                application_id,
                hop_by_hop,
                end_to_end,
            },
            avps: vec![],
        }
    }

    pub fn add_avp(&mut self, avp: Avp) {
        self.avps.push(avp);
    }

    pub fn encode_to<W: Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
        writer.write(&self.header.version.to_be_bytes())?;
        writer.write(&self.header.message_length.to_be_bytes()[1..])?;
        writer.write(&self.header.command_flags.value().to_be_bytes())?;
        writer.write(&self.header.command_code.get_code().to_be_bytes()[1..])?;
        writer.write(&self.header.application_id.value().to_be_bytes())?;
        writer.write(&self.header.hop_by_hop.to_be_bytes())?;
        writer.write(&self.header.end_to_end.to_be_bytes())?;
        for avp in self.avps.iter_mut() {
            avp.encode_to(writer)?;
        }
        Ok(())
    }
}
