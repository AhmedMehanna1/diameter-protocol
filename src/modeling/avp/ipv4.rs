use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_address;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::Read;
use std::net::Ipv4Addr;

pub type IPv4 = AvpData<Ipv4Addr>;

impl AvpDataFormater for IPv4 {
    type Output = Ipv4Addr;

    impl_avp_data_encode_to_address!(IPv4, Ipv4Addr);

    fn decode_from<R: Read>(
        reader: &mut R,
        _: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        let mut b = [0; 4];
        reader.read_exact(&mut b)?;
        let ip = Ipv4Addr::new(b[0], b[1], b[2], b[3]);
        Ok(IPv4::new(ip))
    }

    fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for IPv4 {
    fn into(self) -> AvpValue {
        AvpValue::AddressIPv4(self)
    }
}
