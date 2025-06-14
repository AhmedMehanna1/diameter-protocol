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
use crate::modeling::avp::avp::Avp;
use std::ops::Deref;

#[derive(Debug)]
pub struct DiameterHeader {
    version: u8,
    message_length: u32, // 24 bits
    command_flags: CommandFlags,
    command_code: &'static CommandCode, // 24 bits
    application_id: ApplicationId,
    hop_by_hop: u32,
    end_to_end: u32,
    avps: Vec<Avp>,
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

impl DiameterHeader {
    pub fn new(
        command_flags: CommandFlags,
        command_code: &'static CommandCode,
        application_id: ApplicationId,
        hop_by_hop: u32,
        end_to_end: u32,
    ) -> Self {
        Self {
            version: 1,
            message_length: 32,
            command_flags,
            command_code,
            application_id,
            hop_by_hop,
            end_to_end,
            avps: vec![],
        }
    }

    pub fn add_avp(&mut self, mut avp: Avp) {
        avp.encode();
        self.avps.push(avp);
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded_data = vec![];
        encoded_data.push(self.version);
        encoded_data.extend_from_slice(&self.message_length.to_be_bytes()[1..]);
        encoded_data.push(self.command_flags.value());
        encoded_data.extend_from_slice(&self.command_code.get_code().to_be_bytes()[1..]);
        encoded_data.extend_from_slice(&self.application_id.value().to_be_bytes());
        encoded_data.extend_from_slice(&self.hop_by_hop.to_be_bytes());
        encoded_data.extend_from_slice(&self.end_to_end.to_be_bytes());
        for avp in self.avps.iter() {
            let x = avp.encoded_data.as_ref().unwrap().deref().clone();
            encoded_data.extend_from_slice(&x);
        }
        encoded_data
    }
}
