use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;

pub type Unsigned64 = AvpData<u64>;

impl AvpDataFormater for Unsigned64 {
    type Output = u64;

    impl_avp_data_encode_to_numbers!(Unsigned64, u64);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let num = u64::from_be_bytes(buffer);
        Ok(Unsigned64::new(num))
    }

    fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for Unsigned64 {
    fn into(self) -> AvpValue {
        AvpValue::Unsigned64(self)
    }
}
