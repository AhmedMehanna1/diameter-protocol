use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_numbers;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;

pub type Float64 = AvpData<f64>;

impl AvpDataFormater for Float64 {
    type Output = f64;

    impl_avp_data_encode_to_numbers!(Float64, f64);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let num = f64::from_be_bytes(buffer);
        Ok(Float64::new(num))
    }

    fn len(&self) -> u32 {
        8
    }
}

impl Into<AvpValue> for Float64 {
    fn into(self) -> AvpValue {
        AvpValue::Float64(self)
    }
}
