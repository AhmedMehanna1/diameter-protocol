use crate::errors::Error;
use crate::errors::Error::DecodeError;

#[derive(Debug)]
pub enum ApplicationId {
    Gx,
    Gy,
}

impl ApplicationId {
    pub fn value(&self) -> u32 {
        match self {
            ApplicationId::Gx => 16777238,
            ApplicationId::Gy => 4,
        }
    }
}

impl ApplicationId {
    pub fn get_code(&self) -> u32 {
        match self {
            ApplicationId::Gx => 16777238,
            ApplicationId::Gy => 4,
        }
    }
}

impl TryFrom<u32> for ApplicationId {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            16777238 => Ok(ApplicationId::Gx),
            4 => Ok(ApplicationId::Gy),
            _ => Err(DecodeError("Undefined application id"))?,
        }
    }
}
