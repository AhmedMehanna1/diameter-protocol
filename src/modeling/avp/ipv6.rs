use crate::errors::DiameterResult;
use crate::impl_avp_data_encode_to_address;
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::AvpData;
use std::io::Read;
use std::net::Ipv6Addr;

pub type IPv6 = AvpData<Ipv6Addr>;

impl IPv6 {
    impl_avp_data_encode_to_address!(IPv6, Ipv6Addr);

    pub(super) fn decode_from<R: Read>(reader: &mut R) -> DiameterResult<AvpData<Ipv6Addr>> {
        let mut b = [0; 16];
        reader.read_exact(&mut b)?;

        let ip = Ipv6Addr::new(
            (b[0] as u16) << 8 | b[1] as u16,
            (b[2] as u16) << 8 | b[3] as u16,
            (b[4] as u16) << 8 | b[5] as u16,
            (b[6] as u16) << 8 | b[7] as u16,
            (b[8] as u16) << 8 | b[9] as u16,
            (b[10] as u16) << 8 | b[11] as u16,
            (b[12] as u16) << 8 | b[13] as u16,
            (b[14] as u16) << 8 | b[15] as u16,
        );
        Ok(IPv6::new(ip))
    }

    pub(super) fn len(&self) -> u32 {
        16
    }
}

impl Into<AvpValue> for IPv6 {
    fn into(self) -> AvpValue {
        AvpValue::AddressIPv6(self)
    }
}
