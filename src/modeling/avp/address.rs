use crate::modeling::avp::data::AvpData;
use std::net::{Ipv4Addr, Ipv6Addr};

pub type AddressIPv4 = AvpData<Ipv4Addr>;
pub type AddressIPv6 = AvpData<Ipv6Addr>;
