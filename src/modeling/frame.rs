use crate::modeling::avp::avp::Avp;
use super::command_codes::CommandCode;

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
    REQUEST = 0x80,
    PROXYABLE = 0x40,
    ERROR = 0x20,
    RETRANSMIT = 0x10,
}

#[derive(Debug)]
pub enum ApplicationId {
    Gx = 16777238,
    Gy = 4,
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
    
    pub fn add_avp(&mut self, avp: Avp) {
        self.avps.push(avp);
    }
}
