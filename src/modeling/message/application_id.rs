use crate::errors::Error;
use crate::errors::Error::DecodeError;

#[derive(Debug)]
pub enum ApplicationId {
    Common,
    Accounting,
    CreditControl,
    Gx,
    Rx,
    Sy,
}

impl ApplicationId {
    pub fn value(&self) -> u32 {
        match self {
            ApplicationId::Common => 0,
            ApplicationId::Accounting => 3,
            ApplicationId::CreditControl => 4,
            ApplicationId::Rx => 16777236,
            ApplicationId::Gx => 16777238,
            ApplicationId::Sy => 16777302,
        }
    }
}

impl ApplicationId {
    pub fn get_code(&self) -> u32 {
        match self {
            ApplicationId::Common => 0,
            ApplicationId::Accounting => 3,
            ApplicationId::CreditControl => 4,
            ApplicationId::Rx => 16777236,
            ApplicationId::Gx => 16777238,
            ApplicationId::Sy => 16777302,
        }
    }
}

impl TryFrom<u32> for ApplicationId {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ApplicationId::Common),
            3 => Ok(ApplicationId::Accounting),
            4 => Ok(ApplicationId::CreditControl),
            16777236 => Ok(ApplicationId::Rx),
            16777238 => Ok(ApplicationId::Gx),
            16777302 => Ok(ApplicationId::Sy),
            _ => Err(DecodeError("Undefined application id"))?,
        }
    }
}
