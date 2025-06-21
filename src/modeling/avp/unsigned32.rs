use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;

pub type Unsigned32 = AvpData<u32>;

impl AvpDataFormater for Unsigned32 {
    type Output = u32;

    impl_avp_data_encode_to_numbers!(Unsigned32, u32);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let num = u32::from_be_bytes(buffer);
        Ok(Unsigned32::new(num))
    }

    fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for Unsigned32 {
    fn into(self) -> AvpValue {
        AvpValue::Unsigned32(self)
    }
}
