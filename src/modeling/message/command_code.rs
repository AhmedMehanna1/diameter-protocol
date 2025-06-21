use crate::errors::Error;
use crate::errors::Error::DecodeError;

#[derive(Debug)]
pub enum CommandCode {
    CapabilitiesExchange,
    ReAuth,
    Accounting,
    CreditControl,
    AbortSession,
    SessionTermination,
    DeviceWatchdog,
    DisconnectPeer,
}

impl CommandCode {
    pub fn get_code(&self) -> u32 {
        match self {
            CommandCode::CapabilitiesExchange => 257,
            CommandCode::ReAuth => 258,
            CommandCode::Accounting => 271,
            CommandCode::CreditControl => 272,
            CommandCode::AbortSession => 274,
            CommandCode::SessionTermination => 275,
            CommandCode::DeviceWatchdog => 280,
            CommandCode::DisconnectPeer => 282,
        }
    }
}

impl TryFrom<u32> for CommandCode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            257 => Ok(CommandCode::CapabilitiesExchange),
            258 => Ok(CommandCode::ReAuth),
            271 => Ok(CommandCode::Accounting),
            272 => Ok(CommandCode::CreditControl),
            274 => Ok(CommandCode::AbortSession),
            275 => Ok(CommandCode::SessionTermination),
            280 => Ok(CommandCode::DeviceWatchdog),
            282 => Ok(CommandCode::DisconnectPeer),
            _ => Err(DecodeError("Undefined command code"))?,
        }
    }
}
