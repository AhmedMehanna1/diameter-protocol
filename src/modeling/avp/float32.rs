use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;

pub type Float32 = AvpData<f32>;

impl AvpDataFormater for Float32 {
    type Output = f32;

    impl_avp_data_encode_to_numbers!(Float32, f32);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let num = f32::from_be_bytes(buffer);
        Ok(Float32::new(num))
    }

    fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for Float32 {
    fn into(self) -> AvpValue {
        AvpValue::Float32(self)
    }
}
