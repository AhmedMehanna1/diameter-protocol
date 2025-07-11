use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::AvpData;
use std::io::Read;

pub type Integer64 = AvpData<i64>;

impl Integer64 {
    impl_avp_data_encode_to_numbers!(Integer64, i64);

    pub(super) fn decode_from<R: Read>(reader: &mut R) -> DiameterResult<AvpData<i64>> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let num = i64::from_be_bytes(buffer);
        Ok(Integer64::new(num))
    }

    pub(super) fn len(&self) -> u32 {
        8
    }
}

impl Into<AvpValue> for Integer64 {
    fn into(self) -> AvpValue {
        AvpValue::Integer64(self)
    }
}
