//! # Diameter Protocol Message
//! This crate provides functionalities for handling Diameter protocol messages as defined in RFC 6733.
//!
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

use crate::errors::DiameterResult;
use crate::modeling::avp::avp::{Avp, AvpFlags, AvpValue};
use crate::modeling::message::application_id::ApplicationId;
use crate::modeling::message::command_code::CommandCode;
use crate::modeling::message::command_flags::CommandFlag;
use crate::modeling::message::dictionary::Dictionary;
use std::io::{Read, Write};
use std::sync::Arc;

#[derive(Debug)]
pub struct DiameterMessage {
    header: DiameterHeader,
    avps: Vec<Avp>,
}

#[derive(Debug)]
pub struct DiameterHeader {
    version: u8,
    message_length: u32, // 24 bits
    command_flag: u8,
    command_code: CommandCode, // 24 bits
    application_id: ApplicationId,
    hop_by_hop: u32,
    end_to_end: u32,
}

impl DiameterMessage {
    pub fn new(
        command_flag: CommandFlag,
        command_code: CommandCode,
        application_id: ApplicationId,
        hop_by_hop: u32,
        end_to_end: u32,
    ) -> Self {
        Self {
            header: DiameterHeader {
                version: 1,
                message_length: 20,
                command_flag: command_flag.value(),
                command_code,
                application_id,
                hop_by_hop,
                end_to_end,
            },
            avps: vec![],
        }
    }

    pub fn add(&mut self, avp: Avp) {
        self.header.message_length += avp.get_length() + avp.get_padding();
        self.avps.push(avp);
    }

    pub fn add_avp<T: Into<AvpValue>>(
        &mut self,
        code: u32,
        flags: AvpFlags,
        vendor_id: Option<u32>,
        value: T,
    ) {
        let avp: Avp = Avp::new(code, flags, vendor_id, value);
        self.add(avp);
    }

    pub fn encode_to<W: Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
        writer.write(&self.header.version.to_be_bytes())?;
        writer.write(&self.header.message_length.to_be_bytes()[1..])?;
        writer.write(&self.header.command_flag.to_be_bytes())?;
        writer.write(&self.header.command_code.get_code().to_be_bytes()[1..])?;
        writer.write(&self.header.application_id.value().to_be_bytes())?;
        writer.write(&self.header.hop_by_hop.to_be_bytes())?;
        writer.write(&self.header.end_to_end.to_be_bytes())?;
        for avp in self.avps.iter_mut() {
            avp.encode_to(writer)?;
        }
        Ok(())
    }

    pub fn decode_from<R: Read>(
        reader: &mut R,
        dict: Arc<Dictionary>,
    ) -> DiameterResult<DiameterMessage> {
        let mut b = [0u8; 20];
        reader.read_exact(&mut b)?;

        let version = b[0];
        let mut message_length = u32::from_be_bytes([0, b[1], b[2], b[3]]);
        let command_flag = b[4];
        let command_code = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
        let application_id = u32::from_be_bytes([b[8], b[9], b[10], b[11]]);
        let hop_by_hop = u32::from_be_bytes([b[12], b[13], b[14], b[15]]);
        let end_to_end = u32::from_be_bytes([b[16], b[17], b[18], b[19]]);

        let header = DiameterHeader {
            version,
            message_length,
            command_flag,
            application_id: ApplicationId::try_from(application_id)?,
            command_code: CommandCode::try_from(command_code)?,
            hop_by_hop,
            end_to_end,
        };

        let mut message = DiameterMessage {
            header,
            avps: vec![],
        };

        message_length -= 20;
        while message_length > 0 {
            let avp = Avp::decode_from(reader, Arc::clone(&dict))?;
            message_length = message_length - avp.get_length() - avp.get_padding();
            message.add(avp);
        }
        Ok(message)
    }
}
