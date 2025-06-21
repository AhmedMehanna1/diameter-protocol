use crate::errors::DiameterResult;
use crate::errors::Error::DecodeError;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::{Read, Write};

pub type UTF8String = AvpData<String>;
pub type Identity = UTF8String;

impl UTF8String {
    pub fn from_str(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

impl AvpDataFormater for UTF8String {
    type Output = String;

    fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        writer.write(self.0.as_bytes())?;
        Ok(())
    }

    fn decode_from<R: Read>(
        reader: &mut R,
        length: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = match length {
            None => Err(DecodeError("Length is required to parse UTF8String")),
            Some(length) => Ok(vec![0u8; length]),
        }?;
        reader.read_exact(&mut buffer)?;
        let string = String::from_utf8(buffer).unwrap();
        Ok(UTF8String::new(string))
    }

    fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Into<AvpValue> for UTF8String {
    fn into(self) -> AvpValue {
        AvpValue::UTF8String(self)
    }
}
