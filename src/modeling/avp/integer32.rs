use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;

pub type Integer32 = AvpData<i32>;

impl AvpDataFormater for Integer32 {
    type Output = i32;

    impl_avp_data_encode_to_numbers!(Integer32, i32);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let num = i32::from_be_bytes(buffer);
        Ok(Integer32::new(num))
    }

    fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for Integer32 {
    fn into(self) -> AvpValue {
        AvpValue::Integer32(self)
    }
}
