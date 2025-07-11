use crate::errors::DiameterResult;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::AvpData;
use std::io::{Read, Write};

pub type UTF8String = AvpData<String>;
pub type Identity = UTF8String;

impl UTF8String {
    pub fn from_str(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

impl UTF8String {
    pub(super) fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        writer.write(self.0.as_bytes())?;
        Ok(())
    }

    pub(super) fn decode_from<R: Read>(
        reader: &mut R,
        length: usize,
    ) -> DiameterResult<AvpData<String>> {
        let mut buffer = vec![0u8; length];
        reader.read_exact(&mut buffer)?;
        let string = String::from_utf8(buffer).unwrap();
        Ok(UTF8String::new(string))
    }

    pub(super) fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Into<AvpValue> for UTF8String {
    fn into(self) -> AvpValue {
        AvpValue::UTF8String(self)
    }
}
