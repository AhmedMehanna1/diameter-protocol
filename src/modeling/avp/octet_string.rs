use crate::errors::DiameterResult;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::AvpData;
use std::io::{Read, Write};

pub type OctetString = AvpData<Vec<u8>>;
pub type DiameterURI = OctetString;

impl OctetString {
    pub(super) fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        writer.write(&self.0)?;
        Ok(())
    }

    pub(super) fn decode_from<R: Read>(
        reader: &mut R,
        length: usize,
    ) -> DiameterResult<AvpData<Vec<u8>>> {
        let mut buffer = vec![0u8; length];
        reader.read_exact(&mut buffer)?;
        Ok(AvpData::<Vec<u8>>::new(buffer))
    }

    pub(super) fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Into<AvpValue> for OctetString {
    fn into(self) -> AvpValue {
        AvpValue::OctetString(self)
    }
}
