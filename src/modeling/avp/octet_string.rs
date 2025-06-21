use crate::errors::DiameterResult;
use crate::errors::Error::DecodeError;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::{Read, Write};

pub type OctetString = AvpData<Vec<u8>>;
pub type DiameterURI = OctetString;

impl AvpDataFormater for OctetString {
    type Output = Vec<u8>;

    fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        writer.write(&self.0)?;
        Ok(())
    }

    fn decode_from<R: Read>(
        reader: &mut R,
        length: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = match length {
            None => Err(DecodeError("Length is required to parse OctetString")),
            Some(length) => Ok(vec![0u8; length]),
        }?;
        reader.read_exact(&mut buffer)?;
        Ok(AvpData::<Vec<u8>>::new(buffer))
    }

    fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Into<AvpValue> for OctetString {
    fn into(self) -> AvpValue {
        AvpValue::OctetString(self)
    }
}
