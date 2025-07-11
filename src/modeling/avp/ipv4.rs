use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_address;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::AvpData;
use std::io::Read;
use std::net::Ipv4Addr;

pub type IPv4 = AvpData<Ipv4Addr>;

impl IPv4 {
    impl_avp_data_encode_to_address!(IPv4, Ipv4Addr);

    pub(super) fn decode_from<R: Read>(reader: &mut R) -> DiameterResult<AvpData<Ipv4Addr>> {
        let mut b = [0; 4];
        reader.read_exact(&mut b)?;
        let ip = Ipv4Addr::new(b[0], b[1], b[2], b[3]);
        Ok(IPv4::new(ip))
    }

    pub(super) fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for IPv4 {
    fn into(self) -> AvpValue {
        AvpValue::AddressIPv4(self)
    }
}
